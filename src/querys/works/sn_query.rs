use bb8_tiberius::ConnectionManager;
use futures::stream::TryStreamExt;
use std::collections::HashMap;
use tiberius::Query;
use tiberius_mappers::TryFromRow as _;
use tracing::info;
// 导入新的、唯一的SQL构建函数
use super::query_utils::build_base_union_query;
use crate::{structs::Data, utils::error::MyError};

fn format_data(data: Vec<Data>) -> Vec<HashMap<String, String>> {
    data.into_iter()
        .map(|d| {
            let mut map = HashMap::new();
            // ... (format_data 函数内容不变) ...
            map.insert("sn".to_string(), d.sn);
            map.insert("ith".to_string(), d.ith);
            map.insert("vf".to_string(), d.vf);
            map.insert("im".to_string(), d.im);
            map.insert("po".to_string(), d.po);
            map.insert("rs".to_string(), d.rs);
            map.insert("se".to_string(), d.se);
            map.insert("sen".to_string(), d.sen);
            map.insert("res".to_string(), d.res);
            map.insert("icc".to_string(), d.icc);
            map.insert("vbr".to_string(), d.vbr);
            map.insert("kink".to_string(), d.kink);
            map.insert("imkink".to_string(), d.imkink);
            map.insert(
                "testdate".to_string(),
                d.testdate.format("%Y-%m-%d %H:%M:%S").to_string(),
            );
            map.insert("tester".to_string(), d.tester);
            map.insert("iop".to_string(), d.iop);
            map.insert("idark".to_string(), d.idark);
            map.insert("result".to_string(), d.result);
            map.insert("i_xtalk".to_string(), d.i_xtalk);
            if !d.mdpid.is_empty() {
                map.insert("mdpid".to_string(), d.mdpid);
            }
            map.insert("yypn".to_string(), d.yypn);
            map
        })
        .collect()
}

pub async fn sn_query_datas(
    sns: Vec<String>,
    pn: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    test_result: String,
    test_devices: String,
    worker: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<HashMap<String, String>>, MyError> {
    info!(
        "开始SN查询: sns_count={}, pn={}, use_time={}, start='{}', end='{}', result='{}', device='{}', worker='{}'",
        sns.len(),
        pn,
        use_time,
        date_time_start,
        date_time_end,
        test_result,
        test_devices,
        worker
    );

    // 1. 获取基础的 UNION ALL SQL
    // 这个函数替换了 query_utils.rs 中所有 70+ 个函数
    let base_sql = build_base_union_query(&test_devices, pool).await?;

    // 2. 动态构建 WHERE 子句和绑定参数
    let mut params = Vec::<String>::new();
    let mut where_clauses = Vec::<String>::new();
    let mut param_index = 1;

    // 添加 SNs 条件
    if !sns.is_empty() {
        // 为IN子句动态生成占位符, e.g., "(@P1, @P2, @P3)"
        let placeholders: Vec<String> = sns
            .iter()
            .map(|sn| {
                let p = format!("@P{}", param_index);
                params.push(sn.clone()); // 将 SN 值添加到绑定列表
                param_index += 1;
                p
            })
            .collect();
        where_clauses.push(format!("SN IN ({})", placeholders.join(", ")));
    }

    // 添加 test_result 条件
    if test_result != "全部" {
        where_clauses.push(format!("Result = @P{}", param_index));
        params.push(test_result); // "OK" or "NG"
        param_index += 1;
    }

    // 添加 pn (yypn) 条件 (注意: 别名是 Yypn)
    if !pn.is_empty() {
        where_clauses.push(format!("Yypn = @P{}", param_index));
        params.push(pn);
        param_index += 1;
    }

    // 添加 worker (tester) 条件 (注意: 别名是 Tester)
    if !worker.is_empty() {
        where_clauses.push(format!("Tester = @P{}", param_index));
        params.push(worker);
        param_index += 1;
    }

    // 添加 time 条件
    if use_time {
        if date_time_start.is_empty() || date_time_end.is_empty() {
            return Err(MyError::Zdyknown(
                "启用时间查询时，开始和结束时间不能为空".to_string(),
            ));
        }
        where_clauses.push(format!(
            "TestDate BETWEEN @P{} AND @P{}",
            param_index,
            param_index + 1
        ));
        params.push(date_time_start);
        params.push(date_time_end);
        // param_index += 2; // (不需要，因为我们已经使用了param_index和param_index + 1)
    }

    // 3. 组装最终的 WHERE SQL
    let where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };
    let final_sql = format!(
        "SELECT
        sn, ith, po, vf, im, rs, se, sen, res, icc, vbr, kink, imkink,
        testtime, idark, result, tester, iop, i_xtalk, mdpid, yypn
    FROM (
        {}
    ) AS FinalData
    {}
    ORDER BY testtime DESC",
        base_sql,  // 包含所有 UNION ALL 的 SELECT 语句
        where_sql  // 包含 WHERE SN IN (@P1)
    );
    // 5. 创建参数化查询
    let mut query = Query::new(&final_sql);
    for param in params {
        query.bind(param);
    }

    // 6. 执行查询
    let data = execute_query_sn(query, pool).await?;
    let datas = format_data(data);
    let sn = if !datas.is_empty() {
        let sn = &datas[0].get("sn").cloned().unwrap();
        sn.to_string()
    } else {
        "".to_string()
    };
    let infos = get_box_caoton(sn, pool).await?;
    let all_datas = if !infos.is_empty() {
        let all_datas: Vec<HashMap<String, String>> = datas
            .into_iter()
            .map(|mut d| {
                d.insert(
                    "box_no".to_string(),
                    infos.get("box_no").cloned().unwrap_or_default(),
                );
                d.insert(
                    "carton_no".to_string(),
                    infos.get("carton_no").cloned().unwrap_or_default(),
                );
                d
            })
            .collect();
        all_datas
    } else {
        datas
    };

    info!("SN查询完成，共找到 {} 条记录", all_datas.len());
    Ok(all_datas)
}
async fn get_box_caoton(
    sn: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<HashMap<String, String>, MyError> {
    let sql = "SELECT TOP 1 a.Pack_no, b.cartonno FROM [mes_Factory].[dbo].[MaterialPackSn]a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no WHERE a.sn = @P1 AND a.PnOptionID = '-100' ORDER BY a.CreateTime DESC";
    let mut pool = pool.get().await.unwrap();
    let row = pool.query(sql, &[&sn]).await?;
    let data = row.into_row().await?;
    let mut infos = HashMap::new();
    if let Some(row) = data {
        let box_no = row.get::<&str, _>(0).unwrap().to_string();
        let carton_no = row.get::<&str, _>(1).unwrap().to_string();
        infos.insert("box_no".to_string(), box_no);
        infos.insert("carton_no".to_string(), carton_no);
    }
    Ok(infos)
}

pub async fn execute_query_sn(
    query: Query<'_>,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    // 不再打印 SQL 字符串，以避免日志中泄露敏感数据（如SN列表）
    info!("开始执行 sn_query 参数化查询...");
    let mut client = pool.get().await.unwrap();
    let stream = query.query(&mut client).await?;

    info!("查询执行完毕，开始处理结果集...");
    // 移除了 HashMap 去重逻辑，因为 SQL 查询已经完成了去重
    let mut datas: Vec<Data> = Vec::new();
    let mut row_count = 0;
    let mut rows = stream.into_row_stream();
    while let Ok(Some(row)) = rows.try_next().await {
        row_count += 1;

        let data = Data::try_from_row(row)
            .map_err(|e| MyError::Zdyknown(format!("从行转换为 Data 结构体失败: {:?}", e)))?;
        datas.push(data);
    }

    // 日志现在显示的是去重后的最终SN数量
    info!(
        "共处理 {} 行数据，得到 {} 条SN数据。",
        row_count,
        datas.len()
    );

    if datas.is_empty() {
        return Err(MyError::NoResult("sn_query".to_string()));
    }

    Ok(datas)
}
