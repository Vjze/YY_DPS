use bb8_tiberius::ConnectionManager;
use futures::stream::TryStreamExt;
use std::collections::HashMap;
use tiberius::Query;
use tiberius_mappers::TryFromRow as _;
use tracing::info;
// 导入新的、唯一的SQL构建函数
use super::query_utils::build_base_union_query;
use crate::{
    structs::Data,
    utils::{
        error::MyError,
        pagination::{build_count_query, build_paginated_query, PaginatedQueryResult, PaginationParams},
    },
};

fn format_data(data: Vec<Data>) -> Vec<HashMap<String, String>> {
    data.into_iter()
        .map(|d| {
            let mut map = HashMap::new();
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
            // 处理新增的 Option<String> 字段
            map.insert("box_no".to_string(), d.box_no.unwrap_or_default());
            map.insert("carton_no".to_string(), d.carton_no.unwrap_or_default());
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

    let pagination_params = PaginationParams::new(1, usize::MAX); // 获取所有数据

    // 为了复用逻辑, 直接调用已有的分页函数
    let result = query_sn_paginated(
        &sns,
        &date_time_start,
        &date_time_end,
        &pn,
        &worker,
        &test_result,
        &[test_devices], // query_sn_paginated 期望一个 slice
        use_time,
        pool,
        &pagination_params,
    )
    .await?;

    info!("SN查询完成，共找到 {} 条记录", result.total_count);
    Ok(result.data)
}

/// 支持分页的 SN 查询函数
pub async fn query_sn_paginated(
    sns: &[String],
    date_time_start: &str,
    date_time_end: &str,
    pn: &str,
    worker: &str,
    test_result: &str,
    test_devices: &[String],
    use_time: bool,
    pool: &bb8::Pool<ConnectionManager>,
    pagination_params: &PaginationParams,
) -> Result<PaginatedQueryResult, MyError> {
    // 1. 构建基础 UNION ALL 查询
    let test_devices_str = test_devices.join(",");
    let base_sql = build_base_union_query(
        &test_devices_str,
        pool,
        use_time,
        date_time_start,
        date_time_end,
    )
    .await?;

    // 如果 base_sql 为空 (例如，没有匹配日期的表)，则直接返回空结果
    if base_sql.is_empty() {
        info!("没有可查询的数据表，提前返回空结果。");
        return Ok(PaginatedQueryResult::new(Vec::new(), 0, pagination_params));
    }

    // 2. 动态构建 WHERE 子句和绑定参数
    let mut params = Vec::<String>::new();
    let mut where_clauses = Vec::<String>::new();
    let mut param_index = 1;

    if !sns.is_empty() {
        let placeholders: Vec<String> = sns
            .iter()
            .map(|sn| {
                let p = format!("@P{}", param_index);
                params.push(sn.clone());
                param_index += 1;
                p
            })
            .collect();
        where_clauses.push(format!("FinalData.SN IN ({})", placeholders.join(", ")));
    }

    if test_result != "全部" {
        where_clauses.push(format!("FinalData.Result = @P{}", param_index));
        params.push(test_result.to_string());
        param_index += 1;
    }

    if !pn.is_empty() {
        where_clauses.push(format!("FinalData.Yypn = @P{}", param_index));
        params.push(pn.to_string());
        param_index += 1;
    }

    if !worker.is_empty() {
        where_clauses.push(format!("FinalData.Tester = @P{}", param_index));
        params.push(worker.to_string());
        param_index += 1;
    }

    if use_time {
        if date_time_start.is_empty() || date_time_end.is_empty() {
            return Err(MyError::Zdyknown(
                "启用时间查询时，开始和结束时间不能为空".to_string(),
            ));
        }
        where_clauses.push(format!(
            "FinalData.TestDate BETWEEN @P{} AND @P{}",
            param_index,
            param_index + 1
        ));
        params.push(date_time_start.to_string());
        params.push(date_time_end.to_string());
    }

    let where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // 4. 构建包含 CTE 和 LEFT JOIN 的最终查询
    let base_final_sql = format!(
        r#"
        WITH LatestPack AS (
            SELECT
                a.sn,
                a.Pack_no,
                b.cartonno,
                ROW_NUMBER() OVER(PARTITION BY a.sn ORDER BY a.CreateTime DESC) as rn
            FROM [mes_Factory].[dbo].[MaterialPackSn] a
            INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
            WHERE a.PnOptionID = '-100'
        )
        SELECT
            FinalData.sn, FinalData.ith, FinalData.po, FinalData.vf, FinalData.im, FinalData.rs,
            FinalData.se, FinalData.sen, FinalData.res, FinalData.icc, FinalData.vbr,
            FinalData.kink, FinalData.imkink, FinalData.testtime, FinalData.idark,
            FinalData.result, FinalData.tester, FinalData.iop, FinalData.i_xtalk,
            FinalData.mdpid, FinalData.yypn,
            lp.Pack_no as box_no,
            lp.cartonno as carton_no
        FROM (
            {}
        ) AS FinalData
        LEFT JOIN LatestPack lp ON FinalData.sn = lp.sn AND lp.rn = 1
        {}
        "#,
        base_sql, where_sql
    );

    let paginated_sql = build_paginated_query(&base_final_sql, pagination_params);
    let count_sql = build_count_query(&base_final_sql);

    // 6. 执行分页查询
    let mut query = Query::new(&paginated_sql);
    for param in &params {
        query.bind(param.clone());
    }
    let data = execute_query_sn(query, pool).await?;
    let datas = format_data(data);

    // 7. 执行计数查询
    let mut count_query = Query::new(&count_sql);
    for param in &params {
        count_query.bind(param.clone());
    }
    let total_count = execute_count_query(count_query, pool).await?;

    Ok(PaginatedQueryResult::new(datas, total_count, pagination_params))
}

/// 执行计数查询
async fn execute_count_query(
    query: Query<'_>,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<usize, MyError> {
    let mut conn = pool.get().await.map_err(|e| {
        MyError::Zdyknown(format!("获取数据库连接失败: {}", e))
    })?;

    let stream: tiberius::QueryStream<'_> = query.query(&mut conn).await.map_err(|e| {
        MyError::Zdyknown(format!("执行计数查询失败: {}", e))
    })?;

    if let Some(row) = stream.into_row().await.map_err(|e| {
        MyError::Zdyknown(format!("获取计数结果失败: {}", e))
    })? {
        // COUNT_BIG (i64) is not always returned by SQL Server, sometimes it's a regular INT (i32)
        let total_count: i32 = row.get(0).unwrap_or(0);
        Ok(total_count as usize)
    } else {
        Ok(0)
    }
}

pub async fn execute_query_sn(
    query: Query<'_>,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    info!("开始执行 sn_query 参数化查询...");
    let mut client = pool.get().await.map_err(|_| MyError::DatabaseNotConnected)?;
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::Zdyknown(format!("SN查询执行失败: {}", e)))?;

    info!("查询执行完毕，开始处理结果集...");
    let mut datas: Vec<Data> = Vec::new();
    let mut row_count = 0;
    let mut rows = stream.into_row_stream();
    while let Ok(Some(row)) = rows.try_next().await {
        row_count += 1;

        let data = Data::try_from_row(row)
            .map_err(|e| MyError::Zdyknown(format!("从行转换为 Data 结构体失败: {:?}", e)))?;
        datas.push(data);
    }

    info!(
        "共处理 {} 行数据，得到 {} 条SN数据。",
        row_count,
        datas.len()
    );

    // In a paginated query, an empty result is normal and should not be an error.
    // if datas.is_empty() {
    //     return Err(MyError::NoResult("sn_query".to_string()));
    // }

    Ok(datas)
}
