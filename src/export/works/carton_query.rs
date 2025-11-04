use std::collections::{HashMap, HashSet}; // 引入 HashSet

use crate::{
    configs::type_config::{Infos, get_type_infos}, structs::{BandData, CartonData, Data, Datas, PackData}, utils::{
        error::MyError,
        sql::{client, get_tables},
    }
};
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use futures::{
    TryStreamExt as _,
    stream::{StreamExt as _, iter},
};
use tokio::{
    fs,
    io::{AsyncBufReadExt as _, BufReader}
};
use tracing::info;

async fn get_info() -> anyhow::Result<Vec<String>, MyError> {
    info!("开始执行文件选择...");
    let pick = rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(MyError::Zdyknown(format!("选择框关闭，查询取消。")));
    let path = if let Ok(path) = pick {
        let p_str = path.path().display().to_string();
        info!("文件选择成功: path={}", p_str);
        p_str
    } else {
        return Err(MyError::Zdyknown(format!("未选择文件.")));
    };
    let op = fs::File::open(&path).await?;
    let mut infos = vec![];
    let mut reader = BufReader::new(op).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        infos.push(line);
    }
    info!("文件内容读取完毕，共 {} 行", infos.len());
    Ok(infos)
}

pub async fn do_carton_query(
    carton: String,
    typeinfos: String,
    is_multi: bool,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    info!(
        "开始执行箱号查询: carton={}, typeinfos={}, is_multi={}",
        carton, typeinfos, is_multi
    );
    let client = client().await?;
    let pool = &client;

    if carton.is_empty() && is_multi {
        info!("执行批量查询模式");
        let cartons = get_info().await?;
        let mut all_datas = Vec::new();

        // --- 优化点 5: 并发执行批量查询 ---
        let mut tasks = iter(cartons)
            .map(|carton| {
                let typeinfos = typeinfos.clone();
                let pool = pool.clone();
                // 为每个查询创建一个异步任务
                tokio::spawn(
                    async move { carton_query_datas(carton.clone(), &pool, typeinfos).await },
                )
            })
            .buffer_unordered(10); // 限制并发数为 10

        while let Some(result) = tasks.next().await {
            match result {
                Ok(Ok(res)) => all_datas.extend(res), // 成功, 扩展结果
                Ok(Err(e)) => {
                    info!("批量查询中有一个任务失败: {:?}", e);
                    // 可以选择继续或在这里返回错误
                }
                Err(e) => {
                    info!("批量查询任务执行失败: {:?}", e);
                    // Tokio task join error
                }
            }
        }
        Ok(all_datas)
    } else {
        info!("执行单箱查询模式");
        carton_query_datas(carton, pool, typeinfos).await
    }
}

/**
 * @description: 查询数据的核心流程 (已重构)
 * - 优化: 巨大的 match 语句被替换为一次对 get_base_data_unified 的调用
 */
pub async fn carton_query_datas(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
    typeinfos: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    if carton.is_empty() {
        return Err(MyError::CartonNoEmpty);
    }

    // 1. 获取类型信息
    let infos = get_type_infos(typeinfos).await?.1;
    info!("类型信息解析完毕: {:?}", infos);

    // --- 优化点 1, 3, 4: ---
    // 2. 调用统一的函数获取基础数据 (箱、盒、SN、绑定数据)
    //    这个函数替换了之前所有的 get_data_for... 函数
    let all_datas = get_base_data_unified(carton.clone(), pool, &infos).await?;

    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }

    info!(
        "基础数据和绑定数据获取完毕，共 {} 条，开始查询最新测试数据",
        all_datas.len()
    );

    // 3. 提取 SNs 以查询测试数据 (与原逻辑相同)
    let sn_placeholders = all_datas
        .iter()
        .map(|s| format!("'{}'", s.sn_data.sn))
        .collect::<Vec<String>>();

    // 4. 获取测试数据 (与原逻辑相同, 包括并行处理 > 1000 SNs)
    let sn_list_chunks = sn_placeholders
        .chunks(1000) // SQL Server IN 子句限制约 2100, 900 是个安全数
        .map(|chunk| chunk.join(", "))
        .collect::<Vec<String>>();

    let mut datas = Vec::new();

    if sn_list_chunks.is_empty() {
        // 如果基础数据为空 (虽然前面有检查, 但这里做个保险)
        info!("基础数据为空, 无需查询测试数据");
    } else if sn_list_chunks.len() == 1 {
        // 只有 1 块 (最常见的情况), 正常执行
        info!("开始查询 {} 个SN的测试数据", sn_placeholders.len());
        let sql_text_s = build_query_sql(&sn_list_chunks[0], pool).await?;
        datas = execute_query(&sql_text_s, pool).await?;
    } else {
        // (优化) 并发执行多个 Chunks
        info!(
            "SN总数 {} 超过900，将执行 {} 个并行查询",
            sn_placeholders.len(),
            sn_list_chunks.len()
        );

        let mut tasks = iter(sn_list_chunks)
            .map(|sn_list_chunk| {
                let pool = pool.clone();
                tokio::spawn(async move {
                    let sql_text_s = build_query_sql(&sn_list_chunk, &pool).await?;
                    execute_query(&sql_text_s, &pool).await
                })
            })
            .buffer_unordered(5); // 限制 5 个并发 SQL 查询

        while let Some(result) = tasks.next().await {
            match result {
                Ok(Ok(res_chunk)) => datas.extend(res_chunk),
                Ok(Err(e)) => info!("一个测试数据块查询失败: {:?}", e),
                Err(e) => info!("Tokio 任务失败: {:?}", e),
            }
        }
    }

    info!("测试数据获取完毕，开始整合数据");
    // 5. 合并 TestDate 数据并转换为 HashMap (与原逻辑相同)
    let mut all = all_datas
        .into_iter()
        .map(|mut d| {
            // 找到对应的最新测试数据
            if let Some(sn_datas) = datas.iter().find(|x| x.sn == d.sn_data.sn) {
                d.sn_data = sn_datas.clone();
            }

            // 展平 Datas 为 HashMap
            let mut map = HashMap::new();
            // CartonData
            map.insert("carton_no".to_string(), d.carton_data.carton_no);
            map.insert("pch".to_string(), d.carton_data.pch);
            map.insert("yypn".to_string(), d.carton_data.yypn);
            map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
            map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // BandData
            map.insert("w_sn".to_string(), d.band_data.w_sn);
            map.insert("b_sn".to_string(), d.band_data.b_sn);
            map.insert("bandtime".to_string(), d.band_data.band_time);
            map.insert("band_worker".to_string(), d.band_data.band_worker);
            // Data
            map.insert("sn".to_string(), d.sn_data.sn);
            map.insert("ith".to_string(), d.sn_data.ith);
            map.insert("vf".to_string(), d.sn_data.vf);
            map.insert("im".to_string(), d.sn_data.im);
            map.insert("po".to_string(), d.sn_data.po);
            map.insert("rs".to_string(), d.sn_data.rs);
            map.insert("se".to_string(), d.sn_data.se);
            map.insert("iop".to_string(), d.sn_data.iop);
            map.insert("kink".to_string(), d.sn_data.kink);
            map.insert("imkink".to_string(), d.sn_data.imkink);
            map.insert("sen".to_string(), d.sn_data.sen);
            map.insert("vbr".to_string(), d.sn_data.vbr);
            map.insert("res".to_string(), d.sn_data.res);
            map.insert("icc".to_string(), d.sn_data.icc);
            map.insert("idark".to_string(), d.sn_data.idark);
            map.insert("testdate".to_string(), d.sn_data.testdate);
            map.insert("result".to_string(), d.sn_data.result);
            map.insert("tester".to_string(), d.sn_data.tester);
            map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk);
            map.insert("mdpid".to_string(), d.sn_data.mdpid);
            map
        })
        .collect::<Vec<HashMap<String, String>>>();

    info!("数据整合完毕,一共{}条，开始排序", all.len());
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));
    info!("排序完毕，查询结束");
    Ok(all)
}
async fn get_base_data_unified(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
    infos: &Infos,
) -> anyhow::Result<Vec<Datas>, MyError> {
    info!("开始执行统一查询 (get_base_data_unified)...");

    // --- 1. (最终修复 CTE) 查找最新批次的 CreateTime 窗口 ---
    let cte = "WITH LatestBatchTime AS (
        -- 查找该箱号的绝对最新 CreateTime (T_max)
        SELECT TOP 1 CreateTime AS MaxTime 
        FROM [mes_Factory].[dbo].[packing_carton]
        WHERE CartonNo = @P1 AND PnOptionID = '-100'
        ORDER BY CreateTime DESC
    )";

    // --- 2. 基础表和别名 (保持不变) ---
    let (
        from_clause,
        mut join_list,
        sn_field,
        box_no_field,
        yypn_field,
        pack_worker_field,
        pack_time_field,
        order_by_pack_no,
        pch_join_key_carton,
        pch_join_key_box,
    ) = if infos.zdy_box {
        // ...with_zdy... (自定义盒)
        (
            "FROM [mes_Factory].[dbo].[jz_carton_bind] a".to_string(),
            vec![
                "INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.box_no = b.Packing_no".to_string(),
                // 关键修改: JOIN 最新批次 (LatestBatchTime) 并使用时间窗口过滤
                "INNER JOIN LatestBatchTime lbt ON b.CreateTime <= lbt.MaxTime AND DATEDIFF(SECOND, b.CreateTime, lbt.MaxTime) < 5".to_string(), // <-- 5秒窗口
                "INNER JOIN [mes_Factory].[dbo].[MaterialPackSn] d ON d.Pack_no = b.Packing_no".to_string(),
            ],
            "d.sn", "a.pkg_no", "d.pn", "d.creator", "d.createtime", "d.Pack_no", "a.Pack_no", "d.Pack_no",
        )
    } else {
        // ...no_zdy... (标准盒)
        (
            "FROM [mes_Factory].[dbo].[MaterialPackSn] a".to_string(),
            vec![
                "INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no".to_string(),
                // 关键修改: JOIN 最新批次 (LatestBatchTime) 并使用时间窗口过滤
                "INNER JOIN LatestBatchTime lbt ON b.CreateTime <= lbt.MaxTime AND DATEDIFF(SECOND, b.CreateTime, lbt.MaxTime) < 5".to_string(), // <-- 5秒窗口
            ],
            "a.sn", "a.Pack_no", "a.pn", "a.creator", "a.createtime", "a.Pack_no", "b.CartonNo", "a.Pack_no",
        )
    };
    
    // --- 3. 基础 SELECT 字段 (保持不变) ---
    let mut select_list = vec![
        format!("{} AS sn", sn_field),
        format!("{} AS box_no", box_no_field),
        format!("{} AS yypn", yypn_field),
        format!("{} AS pack_worker", pack_worker_field),
        format!("{} AS pack_time_dt", pack_time_field),
        "b.creator AS carton_worker".to_string(),
        "b.createtime AS carton_time_dt".to_string(),
    ];

    // --- 4. PCH (批次号) 逻辑 (保持不变) ---
    if infos.is_have_pch {
        let join_key = if infos.carton_pch { pch_join_key_carton } else { pch_join_key_box };
        join_list.push(format!(
            "OUTER APPLY (SELECT TOP 1 c.parameter FROM [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c WHERE c.LABEL_KEY = {} ORDER BY c.CreateTime DESC) AS pch_log",
            join_key
        ));
        select_list.push("SUBSTRING(pch_log.parameter, CHARINDEX('YEAR=', pch_log.parameter) + 5, CHARINDEX(';', pch_log.parameter + ';', CHARINDEX('YEAR=', pch_log.parameter)) - (CHARINDEX('YEAR=', pch_log.parameter) + 5)) AS ExtractedYear".to_string());
        select_list.push("SUBSTRING(pch_log.parameter, CHARINDEX('WEEK=', pch_log.parameter) + 6, CHARINDEX(';', pch_log.parameter + ';', CHARINDEX('WEEK=', pch_log.parameter)) - (CHARINDEX('WEEK=', pch_log.parameter) + 6)) AS ExtractedWeek".to_string());
    } else {
        select_list.push("'' AS ExtractedYear".to_string());
        select_list.push("'' AS ExtractedWeek".to_string());
    }

    // --- 5. Band (绑定) 逻辑 (保持不变) ---
    if infos.jz_band {
        join_list.push(format!("LEFT JOIN [mes_Factory].[dbo].[QA_snRelation] qr ON {} = qr.sn", sn_field));
        select_list.push("qr.SN_ShipMent AS b_sn".to_string());
        select_list.push(format!("{} AS w_sn", sn_field));
        select_list.push("qr.userno AS band_worker".to_string());
        select_list.push("qr.Relationtime AS band_time".to_string());
    } else {
        select_list.push("'' AS b_sn".to_string());
        select_list.push(format!("{} AS w_sn", sn_field));
        select_list.push("'' AS band_worker".to_string());
        select_list.push("'' AS band_time".to_string());
    }

    // --- 6. 组装并执行 SQL ---
    let select_clause = select_list.join(", ");
    let join_clause = join_list.join(" ");
    let sql = format!(
        // 注意: 我们仍然保留了 b.CartonNo = @P1 AND b.PnOptionID = '-100' 作为最终 WHERE 条件
        // 因为 CTE TOP 1 无法保证只选择了 @P1 的 CartonNo（虽然在 CTE 里已过滤）
        "{0} SELECT {1} {2} {3} WHERE b.CartonNo = @P1 AND b.PnOptionID = '-100' ORDER BY b.Packing_no desc, {4} asc",
        cte, 
        select_clause, from_clause, join_clause, order_by_pack_no
    );

    info!("执行统一 SQL 查询 (已最终优化): {}", sql);
    let mut client = pool.get().await.unwrap();
    // CartonNo 的值 @P1 现在被用于 CTE 内部
    let stream = client.query(&sql, &[&carton]).await.unwrap(); 

    // --- 7. 解析循环 (保持不变) ---
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();
    let mut rows = stream.into_row_stream();

    while let Ok(Some(row)) = rows.try_next().await {
        let sn = match row.get::<&str, _>("sn") {
            Some(s) => s.to_string(),
            None => { continue; }
        };
        if seen_sns.contains(&sn) { continue; }

        let year = row.get::<&str, _>("ExtractedYear").unwrap_or_default();
        let week = row.get::<&str, _>("ExtractedWeek").unwrap_or_default();
        let pch = if !year.is_empty() && !week.is_empty() && year.len() >= 2 {
            let year_last_two = &year[year.len() - 2..];
            format!("{}{}", year_last_two, week)
        } else {
            "None".to_string()
        };

        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.get::<&str, _>("yypn").unwrap_or_default().to_string(),
            carton_worker: row.get::<&str, _>("carton_worker").unwrap_or_default().to_string(),
            carton_packtime: row.get::<NaiveDateTime, _>("carton_time_dt").unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
            pch,
        };
        let pack_data = PackData {
            box_no: row.get::<&str, _>("box_no").unwrap_or_default().to_string(),
            pack_worker: row.get::<&str, _>("pack_worker").unwrap_or_default().to_string(),
            pack_packtime: row.get::<NaiveDateTime, _>("pack_time_dt").unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let band_data = BandData {
            w_sn: sn.clone(),
            b_sn: row.get::<&str, _>("b_sn").unwrap_or_default().to_string(),
            band_time: row.get::<&str, _>("band_time").unwrap_or_default().to_string(),
            band_worker: row.get::<&str, _>("band_worker").unwrap_or_default().to_string(),
        };
        let sn_data = Data { sn: sn.clone(), ..Default::default() };

        all_datas.push(Datas { carton_data, pack_data, sn_data, band_data });
        seen_sns.insert(sn);
    }
    info!("统一查询处理完毕 (已最终优化)，共 {} 条唯一SN数据", all_datas.len());
    Ok(all_datas)
}

pub async fn execute_query(
    sql_text_s: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    // info!("开始执行 carton_query 查询 (已优化): {}", sql_text_s);
    let mut client = pool.get().await.unwrap();
    let stream = client.query(sql_text_s, &[&1i32]).await?;
    info!("查询执行完毕，开始处理结果集...");
    let mut rows = stream.into_row_stream();

    // 优化: 不再需要 HashMap 去重，SQL 已经保证了唯一性
    let mut datas: Vec<Data> = Vec::new();
    let mut row_count = 0;

    while let Ok(Some(row)) = rows.try_next().await {
        row_count += 1;
        let sn = row.get::<&str, _>(0).unwrap().to_string();
        let kink = row.get::<&str, _>(11).unwrap_or_default();
        let imkink = row.get::<&str, _>(12).unwrap_or_default();
        let mdpid = if row.get::<&str, _>(19).unwrap_or_default() == "0" {
            "".to_string()
        } else {
            row.get::<&str, _>(19).unwrap_or_default().to_string()
        };
        let yypn = row.get::<&str, _>(20).unwrap_or_default().to_string();

        let data = Data {
            sn: sn.clone(),
            ith: row.get::<&str, _>(1).unwrap_or_default().to_string(),
            vf: row.get::<&str, _>(3).unwrap_or_default().to_string(),
            im: row.get::<&str, _>(4).unwrap_or_default().to_string(),
            po: row.get::<&str, _>(2).unwrap_or_default().to_string(),
            rs: row.get::<&str, _>(5).unwrap_or_default().to_string(),
            se: row.get::<&str, _>(6).unwrap_or_default().to_string(),
            sen: row.get::<&str, _>(7).unwrap_or_default().to_string(),
            res: row.get::<&str, _>(8).unwrap_or_default().to_string(),
            icc: row.get::<&str, _>(9).unwrap_or_default().to_string(),
            vbr: row.get::<&str, _>(10).unwrap_or("0.00").to_string(),
            kink: kink.to_string(),
            imkink: imkink.to_string(),
            testdate: row
                .get::<NaiveDateTime, _>(13)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            tester: row.get::<&str, _>(16).unwrap_or_default().to_string(),
            iop: row.get::<&str, _>(17).unwrap_or_default().to_string(),
            idark: row.get::<&str, _>(14).unwrap_or_default().to_string(),
            result: row.get::<&str, _>(15).unwrap_or_default().to_string(),
            i_xtalk: row.get::<&str, _>(18).unwrap_or_default().to_string(),
            mdpid,
            yypn,
        };

        // 直接添加，无需检查
        datas.push(data);
    }

    info!(
        "共处理 {} 行原始数据 (已在SQL去重)，得到 {} 条最新SN数据。",
        row_count,
        datas.len()
    );
    if datas.is_empty() {
        return Err(MyError::NoResult(format!("")));
    }
    Ok(datas)
}

/**
 * @description: (已修改) 构建测试数据查询 SQL
 * - 优化: 使用 ROW_NUMBER() 在 SQL 端对每个 SN 按 TestDate 排序，只取最新 (rn = 1)
 */
pub async fn build_query_sql(
    sn_list: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<String, MyError> {
    // 定义字段
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }

    // --- 核心优化 ---
    // 1. 将所有 UNION ALL 的结果作为子查询 (AllData)
    // 2. 在外层使用 ROW_NUMBER() 进行分区排序
    // 3. 在最外层 SELECT 中筛选 rn = 1
    let base_query = format!(
        "SELECT *, ROW_NUMBER() OVER(PARTITION BY SN ORDER BY TestDate DESC) as rn FROM ({}) AllData",
        sql_text
    );

    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'OK'") // 如果 SN 列表为空，这个查询意义不大，但保留原逻辑
    } else {
        format!("WHERE SN IN ({}) AND Result = 'OK'", sn_list)
    };

    // 最终 SQL: 从已排序和编号的 (tmp) 结果中只选择 rn = 1 的行
    Ok(format!(
        "SELECT * FROM ({}) tmp {} AND tmp.rn = 1",
        base_query, query_ty
    ))
}

// pub async fn build_query_sql(
//     sn_list: &str,
//     pool: &bb8::Pool<ConnectionManager>,
// ) -> anyhow::Result<String, MyError> {
//     // 定义字段
//     let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
//     let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
//     let sql_10 = format!(
//         "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
//         testtype
//     );
//     let mut sql_text = String::from(&sql_10);
//     let tables = get_tables(pool).await?;
//     for i in tables {
//         let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
//         sql_text.push_str(&s);
//     }
//     if sql_text.ends_with(" UNION ALL ") {
//         sql_text.truncate(sql_text.len() - " UNION ALL ".len());
//     }
//     let query_ty = if sn_list.is_empty() {
//         format!("WHERE Result = 'OK' ORDER BY TestDate DESC")
//     } else {
//         format!(
//             "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
//             sn_list
//         )
//     };
//     Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
// }
