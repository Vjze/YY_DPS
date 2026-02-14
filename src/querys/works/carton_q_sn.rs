use crate::{
    export::works::carton_query::build_query_sql,
    structs::{CartonData, Data, Datas, PackData},
    utils::error::MyError,
};
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use futures::{
    TryStreamExt as _,
    stream::{StreamExt as _, iter},
};
use std::collections::{HashMap, HashSet};
use tiberius_mappers::TryFromRow;
use tracing::info;

pub async fn get_carton_sn_datas(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn

    let sql_text = format!(
        "select a.sn, a.Pack_no, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
        carton
    );
    println!("执行 SQL 查询: {}", sql_text);
    let mut pool_1 = pool.get().await.unwrap();

    let stream = pool_1.simple_query(sql_text).await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::Zdyknown(format!(
                        "箱号:{},没有找到sn信息，请注意箱号是否正确!!!",
                        carton
                    )));
                }
            };
            if seen_sns.contains(&sn) {
                continue; // 跳过重复的 SN
            }
            let box_data = PackData {
                box_no: row.get::<&str, _>(1).unwrap_or_default().to_string(),
                ..Default::default()
            };
            let sn_data = Data {
                sn: sn.clone(),
                ..Default::default()
            };
            let datas = Datas {
                pack_data: box_data,
                sn_data: sn_data,
                ..Default::default()
            };
            all_datas.push(datas);
            seen_sns.insert(sn);
        }
    }

    if all_datas.is_empty() {
        return Err(MyError::Zdyknown(format!("箱号 '{}' 没有找到数据", carton)));
    }
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
        let sql_text_s = build_query_sql(&sn_list_chunks[0], &pool).await?;
        datas = execute_query(&sql_text_s, &pool).await?;
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
    let a_datas = all_datas
        .iter_mut()
        .map(|d| {
            let sn_d = datas.iter_mut().find(|s_d| d.sn_data.sn == s_d.sn).unwrap();
            d.sn_data = sn_d.clone();
            d.clone()
        })
        .collect::<Vec<Datas>>();
    info!("测试数据获取完毕，开始整合数据");
    Ok(all_datas)
}
async fn execute_query(
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
        let data = Data::try_from_row(row)
            .map_err(|e| MyError::Zdyknown(format!("从行转换为 Data 结构体失败: {:?}", e)))?;
        row_count += 1;
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
