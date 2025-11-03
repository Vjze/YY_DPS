use crate::export::works::carton_query::build_query_sql;
use crate::{
    structs::{Data, Datas, PackData},
    utils::{error::MyError, sql::client},
};
use futures::{
    TryStreamExt as _,
    stream::{StreamExt as _, iter},
};
use sqlx_oldapi::mssql::MssqlRow;
use sqlx_oldapi::types::chrono::NaiveDateTime;
use sqlx_oldapi::{Error as SqlxError, MssqlPool, Row, query, query_as};
use std::collections::{HashMap, HashSet};
use tracing::info;
#[derive(Debug)]
struct QueryRow {
    pub sn: String,
    pub box_no: String,
    pub yypn: String,
    pub pack_worker: String,
    pub create_time: NaiveDateTime,
}

impl sqlx_oldapi::FromRow<'_, MssqlRow> for QueryRow {
    fn from_row(row: &MssqlRow) -> Result<Self, SqlxError> {
        Ok(QueryRow {
            sn: row.try_get("sn")?,
            box_no: row.try_get("Pack_no")?,
            yypn: row.try_get("pn")?,
            pack_worker: row.try_get("creator")?,
            create_time: row.try_get("createtime")?,
        })
    }
}

#[allow(unused_assignments)]
pub async fn get_box_datas(
    box_no: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    let pool: &MssqlPool = &client().await?;

    // 预检查：所有条件为空且不使用时间查询
    if box_no.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }

    // 构建动态SQL查询
    let mut sql = String::from(
        "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE PnOptionID = '-100'",
    );
    let mut params = Vec::new();
    let mut param_index = 1;

    // 添加box_no条件
    if !box_no.is_empty() {
        sql.push_str(&format!(" AND Pack_no = @P{}", param_index));
        params.push(box_no.clone());
        param_index += 1;
    }

    // 添加pn条件
    if !pn.is_empty() {
        sql.push_str(&format!(" AND pn = @P{}", param_index));
        params.push(pn.clone());
        param_index += 1;
    }

    // 添加时间范围条件
    if use_time && !date_time_start.is_empty() && !date_time_end.is_empty() {
        sql.push_str(&format!(
            " AND createtime BETWEEN @P{} AND @P{}",
            param_index,
            param_index + 1
        ));
        params.push(date_time_start);
        params.push(date_time_end);
        param_index += 2;
    }

    // 添加排序
    sql.push_str(" ORDER BY CreateTime DESC");
    info!("SQL: {}", sql);
    info!("Params: {:?}", params);

    // 创建查询并绑定参数
    let mut query = query_as::<_, QueryRow>(&sql);
    for param in params {
        query = query.bind(param);
    }

    // 执行查询
    let mut rows = query.fetch(pool);

    // 处理结果
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
        let sn = row.sn.clone();

        // 去重逻辑
        if seen_sns.contains(&sn) {
            continue;
        }

        let pack_data = PackData {
            box_no: row.box_no,
            pack_worker: row.pack_worker,
            pack_packtime: row.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let sn_data = Data {
            sn: sn.clone(),
            yypn: row.yypn,
            ..Default::default()
        };

        all_datas.push(Datas {
            pack_data,
            sn_data,
            ..Default::default()
        });
        seen_sns.insert(sn);
    }
    info!("一共:{}条数据.", all_datas.len());
    let sn_placeholders = all_datas
        .iter()
        .map(|s| format!("'{}'", s.sn_data.sn))
        .collect::<Vec<String>>();
    let sn_list_chunks = sn_placeholders
        .chunks(1400) // SQL Server IN 子句限制约 2100, 900 是个安全数
        .map(|chunk| chunk.join(", "))
        .collect::<Vec<String>>();
    // let sn_datas = get_sn_info(sns).await?;
    let mut sn_datas = vec![];
    if sn_list_chunks.is_empty() {
        // 如果基础数据为空 (虽然前面有检查, 但这里做个保险)
        info!("基础数据为空, 无需查询测试数据");
    } else if sn_list_chunks.len() == 1 {
        // 只有 1 块 (最常见的情况), 正常执行
        info!("开始查询 {} 个SN的测试数据", sn_placeholders.len());
        sn_datas = get_sn_info(&sn_list_chunks[0]).await?;
        // let sql_text_s = build_query_sql(&sn_list_chunks[0], pool).await?;
        // datas = execute_query(&sql_text_s, pool).await?;
    } else {
        // (优化) 并发执行多个 Chunks
        info!(
            "SN总数 {} 超过900，将执行 {} 个并行查询",
            sn_placeholders.len(),
            sn_list_chunks.len()
        );

        let mut tasks = iter(sn_list_chunks)
            .map(|sn_list_chunk| tokio::spawn(async move { get_sn_info(&sn_list_chunk).await }))
            .buffer_unordered(5); // 限制 5 个并发 SQL 查询

        while let Some(result) = tasks.next().await {
            match result {
                Ok(Ok(res_chunk)) => sn_datas.extend(res_chunk),
                Ok(Err(e)) => info!("一个测试数据块查询失败: {:?}", e),
                Err(e) => info!("Tokio 任务失败: {:?}", e),
            }
        }
    }
    let carton_data = get_carton_data(&box_no).await;
    let all = all_datas
        .into_iter()
        .map(|mut d| {
            if carton_data.is_some() {
                let carton_data_map = carton_data.as_ref().unwrap();
                d.carton_data.carton_no = carton_data_map.get("carton_no").unwrap().to_string();
            }

            d
        })
        .collect::<Vec<Datas>>();
    let mut all = all
        .into_iter()
        .map(|mut d| {
            let sn_datas = sn_datas
                .iter()
                .find(|x| x.sn == d.sn_data.sn)
                .map(|s| s.clone())
                .unwrap_or_default();
            d.sn_data = sn_datas;

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
    if all.is_empty() {
        let query_key = match (box_no.is_empty(), pn.is_empty()) {
            (false, _) => &box_no,
            (_, false) => &pn,
            (true, true) => "时间范围",
        };
        return Err(MyError::Zdyknown(format!(
            "查询条件 '{}' 没有找到数据",
            query_key
        )));
    }
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));

    Ok(all)
}

async fn get_sn_info(sns: &str) -> anyhow::Result<Vec<Data>, MyError> {
    let pool = &client().await?;
    let sql_text = build_query_sql(&sns, pool).await?;
    let mut rows = query_as::<sqlx_oldapi::Mssql, Data>(&sql_text).fetch(pool);
    info!("sn查询执行完毕，开始处理结果集...");
    let mut sn_map: HashMap<String, Data> = HashMap::new();
    let mut row_count = 0;
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
        datas.len()
    );
    if datas.is_empty() {
        return Err(MyError::NoResult(format!("")));
    }
    Ok(datas)
}
async fn get_carton_data(box_no: &str) -> Option<HashMap<String, String>> {
    let pool = &client().await.unwrap();
    let sql_text = format!(
        "SELECT TOP 1 CartonNo FROM [mes_Factory].[dbo].[packing_carton] WHERE Packing_no = '{}'",
        box_no
    );

    let row = query(&sql_text)
        .fetch_optional(pool) // 💥 关键修改：使用 fetch_optional
        .await
        .unwrap();
    if row.is_none() {
        return None;
    }
    let carton_no: Option<String> = row.map(|r| r.get("CartonNo")).unwrap();
    let mut carton_data = HashMap::new();
    if carton_no.is_some() {
        carton_data.insert("carton_no".to_string(), carton_no.clone().unwrap());
        carton_data.insert("box_no".to_string(), box_no.to_string());
    } else {
        carton_data.insert("carton_no".to_string(), "".to_string());
    }

    Some(carton_data)
}
