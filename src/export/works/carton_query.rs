use crate::{
    configs::type_config::{Infos, get_type_infos},
    structs::{BandData, CartonData, Data, Datas, PackData},
    utils::{
        error::MyError,
        sql::{client, get_tables},
    },
};
use chrono::NaiveDateTime;
use futures::{
    TryStreamExt as _,
    stream::{StreamExt as _, iter},
};
use sqlx_oldapi::{MssqlPool, Row as _};
use std::collections::{HashMap, HashSet}; // 引入 HashSet
use tokio::{
    fs,
    io::{AsyncBufReadExt as _, BufReader},
};
use tracing::info;

#[derive(Debug, Clone, Default)]
pub struct QueryResultRow {
    // MaterialPackSn/jz_carton_bind 的 SN 和相关信息
    pub sn: String,

    // Box/Pack Data (来自 a 或 d)
    pub box_no: String,
    pub pack_worker: String,
    pub pack_time_dt: NaiveDateTime, // 对应 pack_time_field

    // Carton Data (来自 b)
    pub yypn: String,
    pub carton_worker: String,
    pub carton_time_dt: NaiveDateTime, // 对应 b.createtime

    // 可能为 NULL 的字段，使用 Option<String>
    pub extracted_year: Option<String>,
    pub extracted_week: Option<String>,

    // Band 逻辑提取的字段 (LEFT JOIN 或被 SELECT NULL 占位)
    pub b_sn: Option<String>,
    pub w_sn: String, // 这个字段始终是 sn_field 的别名，保证非 NULL
    pub band_worker: Option<String>,
    pub band_time: Option<String>,
}
impl sqlx_oldapi::FromRow<'_, sqlx_oldapi::mssql::MssqlRow> for QueryResultRow {
    fn from_row(row: &sqlx_oldapi::mssql::MssqlRow) -> Result<Self, sqlx_oldapi::Error> {
        Ok(QueryResultRow {
            // 非 Option 字段 (使用 try_get 提取)
            sn: row.try_get("sn")?,
            box_no: row.try_get("box_no")?,
            pack_worker: row.try_get("pack_worker")?,
            pack_time_dt: row.try_get("pack_time_dt")?,
            yypn: row.try_get("yypn")?,
            carton_worker: row.try_get("carton_worker")?,
            carton_time_dt: row.try_get("carton_time_dt")?,
            w_sn: row.try_get("w_sn")?,

            // Option 字段 (使用 try_get 提取，sqlx 会自动将其映射为 Option<T>)
            extracted_year: row.try_get("extracted_year")?,
            extracted_week: row.try_get("extracted_week")?,
            b_sn: row.try_get("b_sn")?,
            band_worker: row.try_get("band_worker")?,
            band_time: row.try_get("band_time")?,
        })
    }
}

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
    let pool = client().await?;

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
        carton_query_datas(carton, &pool, typeinfos).await
    }
}

/**
 * @description: 查询数据的核心流程 (已重构)
 * - 优化: 巨大的 match 语句被替换为一次对 get_base_data_unified 的调用
 */
pub async fn carton_query_datas(
    carton: String,
    pool: &MssqlPool,
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
        .chunks(1400) // SQL Server IN 子句限制约 2100, 900 是个安全数
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
    pool: &MssqlPool,
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
        let join_key = if infos.carton_pch {
            pch_join_key_carton
        } else {
            pch_join_key_box
        };
        join_list.push(format!(
            "OUTER APPLY (SELECT TOP 1 c.parameter FROM [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c WHERE c.LABEL_KEY = {} ORDER BY c.CreateTime DESC) AS pch_log",
            join_key
        ));
        select_list.push("SUBSTRING(pch_log.parameter, CHARINDEX('YEAR=', pch_log.parameter) + 5, CHARINDEX(';', pch_log.parameter + ';', CHARINDEX('YEAR=', pch_log.parameter)) - (CHARINDEX('YEAR=', pch_log.parameter) + 5)) AS ExtractedYear".to_string());
        select_list.push("SUBSTRING(pch_log.parameter, CHARINDEX('WEEK=', pch_log.parameter) + 6, CHARINDEX(';', pch_log.parameter + ';', CHARINDEX('WEEK=', pch_log.parameter)) - (CHARINDEX('WEEK=', pch_log.parameter) + 6)) AS ExtractedWeek".to_string());
    } else {
        select_list.push("NULL AS extracted_year".to_string());
        select_list.push("NULL AS extracted_week".to_string());
    }

    // --- 5. Band (绑定) 逻辑 (保持不变) ---
    if infos.jz_band {
        join_list.push(format!(
            "LEFT JOIN [mes_Factory].[dbo].[QA_snRelation] qr ON {} = qr.sn",
            sn_field
        ));
        select_list.push("qr.SN_ShipMent AS b_sn".to_string());
        select_list.push(format!("{} AS w_sn", sn_field));
        select_list.push("qr.userno AS band_worker".to_string());
        select_list.push("qr.Relationtime AS band_time".to_string());
    } else {
        select_list.push("NULL AS b_sn".to_string());
        select_list.push(format!("{} AS w_sn", sn_field));
        select_list.push("NULL AS band_worker".to_string());
        select_list.push("NULL AS band_time".to_string());
    }

    // --- 6. 组装并执行 SQL ---
    let select_clause = select_list.join(", ");
    let join_clause = join_list.join(" ");
    let sql = format!(
        // 注意: 我们仍然保留了 b.CartonNo = @P1 AND b.PnOptionID = '-100' 作为最终 WHERE 条件
        // 因为 CTE TOP 1 无法保证只选择了 @P1 的 CartonNo（虽然在 CTE 里已过滤）
        "{0} SELECT {1} {2} {3} WHERE b.CartonNo = @P1 AND b.PnOptionID = '-100' ORDER BY b.Packing_no desc, {4} asc",
        cte, select_clause, from_clause, join_clause, order_by_pack_no
    );

    info!("执行统一 SQL 查询 (已最终优化): {}", sql);
    // CartonNo 的值 @P1 现在被用于 CTE 内部
    let mut query = sqlx_oldapi::query_as::<_, QueryResultRow>(&sql)
        .bind(&carton)
        .fetch(pool);
    // --- 7. 解析循环 (保持不变) ---
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    while let Ok(Some(row)) = query.try_next().await {
        let sn = row.sn;
        if sn.is_empty() {
            continue;
        }
        if seen_sns.contains(&sn) {
            continue;
        }
        let year = row.extracted_year.as_deref().unwrap_or_default();
        let week = row.extracted_week.as_deref().unwrap_or_default();

        let pch = if !year.is_empty() && !week.is_empty() && year.len() >= 2 {
            let year_last_two = &year[year.len() - 2..];
            format!("{}{}", year_last_two, week)
        } else {
            "None".to_string()
        };

        // 映射到最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.yypn.clone(),
            carton_worker: row.carton_worker.clone(),
            carton_packtime: row.carton_time_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            pch,
        };
        let pack_data = PackData {
            box_no: row.box_no.clone(),
            pack_worker: row.pack_worker.clone(),
            pack_packtime: row.pack_time_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let band_data = BandData {
            w_sn: row.w_sn.clone(),
            // 使用 unwrap_or_default() 将 Option<String> 转换为 String (None => "")
            b_sn: row.b_sn.unwrap_or_default(),
            band_time: row.band_time.unwrap_or_default(),
            band_worker: row.band_worker.unwrap_or_default(),
        };
        let sn_data = Data {
            sn: sn.clone(),
            ..Default::default()
        };

        all_datas.push(Datas {
            carton_data,
            pack_data,
            sn_data,
            band_data,
        });
        seen_sns.insert(sn);
    }
    info!(
        "统一查询处理完毕 (已最终优化)，共 {} 条唯一SN数据",
        all_datas.len()
    );
    Ok(all_datas)
}

pub async fn execute_query(sql_text_s: &str, pool: &MssqlPool) -> Result<Vec<Data>, MyError> {
    // info!("开始执行 carton_query 查询: {}", sql_text_s);
    let mut rows = sqlx_oldapi::query_as::<sqlx_oldapi::Mssql, Data>(sql_text_s).fetch(pool);
    info!("查询执行完毕，开始处理结果集...");
    let mut sn_map: HashMap<String, Data> = HashMap::new();
    let mut row_count = 1;
    while let Some(data) = rows.try_next().await.map_err(MyError::from)? {
        row_count += 1;
        let sn = data.sn.clone();

        // 3. 只保留最新的测试数据 (去重逻辑不变)
        if let Some(existing_data) = sn_map.get(&sn) {
            if existing_data.testdate < data.testdate {
                sn_map.insert(sn, data);
            }
        } else {
            sn_map.insert(sn, data);
        }
    }
    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();
    info!(
        "共处理 {} 行原始数据，去重后得到 {} 条最新SN数据。",
        row_count,
        datas.len() + 1
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
pub async fn build_query_sql(sn_list: &str, pool: &MssqlPool) -> anyhow::Result<String, MyError> {
    
    // 1. 定义最终结果集中的所有列 (全部小写，用于 Rust 映射)
    let final_columns = "sn,ith,po,vf,im,rs,se,sen,res,icc,vbr,kink,imkink,testdate,idark,result,tester,iop,i_xtalk,mdpid,yypn";

    // 2. 定义第一个表的 SELECT 映射 (MAC_10GBOSADATA)
    let select_10 = format!(
        // 注意：将所有类型不确定的列（如 TestDate）强制转换为统一类型，并设置统一别名（小写）
        "SELECT 
            SN AS sn, Ith AS ith, 
            Pf AS po, Vop AS vf, Im AS im, Rs AS rs, 
            Se AS se, Sen AS sen, Res AS res, ICC AS icc, Vbr AS vbr, 
            Kink AS kink, imkink AS imkink, 
            CAST(TestDate AS DATETIME2(0)) AS testdate, /* 强制类型转换 */
            Idark AS idark, Result AS result, ProductBill AS tester, 
            iop AS iop, ixtalk AS i_xtalk, MDPId AS mdpid, testtype AS yypn
        FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA]"
    );

    // 3. 定义后续表的 SELECT 映射 (MAC_xxx)
    let select_other_template = format!(
        // 注意：使用 CAST(NULL AS TYPE) 占位缺失的列，并统一相似的列名
        "UNION ALL SELECT 
            SN AS sn, Ith AS ith, 
            Po AS po, Vf AS vf, Im AS im, Rs AS rs, 
            Pslop AS se, /* Pslop 映射到 se */
            Sen AS sen, Res AS res, ICC AS icc, Vbr AS vbr, 
            Kink_I AS kink, kinkim_i AS imkink, 
            CAST(TestDate AS DATETIME2(0)) AS testdate, /* 强制类型转换 */
            Idark AS idark, Result AS result, ProductBill AS tester, 
            io AS iop, /* io 映射到 iop */
            xtalk AS i_xtalk, /* xtalk 映射到 i_xtalk */
            Te AS mdpid, /* Te 映射到 mdpid */
            testtype AS yypn
        FROM {{}} /* 占位符 for 表名 */ "
    );

    let mut sql_text = String::from(&select_10);
    
    let tables = get_tables(pool).await?; // 假设 get_tables 成功返回表名
    
    for table_name in tables {
        // 使用 format! 插入表名到模板中
        let s = select_other_template.replace("{}", &table_name);
        sql_text.push_str(&s);
    }

    // 4. 构建最终查询
    
    // 使用统一的列名来构建外部 SELECT
    let base_query = format!(
        "SELECT {final_columns}, ROW_NUMBER() OVER(PARTITION BY sn ORDER BY testdate DESC) as rn FROM ({sql_text}) AllData",
        final_columns = final_columns,
        sql_text = sql_text
    );

    let query_ty = if sn_list.is_empty() {
        // 在 WHERE 子句中，使用小写别名
        "WHERE result = 'OK'".to_string() 
    } else {
        // 在 WHERE 子句中，使用小写别名
        format!("WHERE sn IN ({}) AND result = 'OK'", sn_list)
    };

    // 最终 SQL: 从已排序和编号的 (tmp) 结果中只选择 rn = 1 的行
    Ok(format!(
        "SELECT {final_columns} FROM ({base_query}) tmp {query_ty} AND tmp.rn = 1",
        final_columns = final_columns,
        base_query = base_query,
        query_ty = query_ty
    ))
}

