use crate::{
    export::works::carton_query::build_query_sql,
    structs::{Data, Datas, PackData},
    utils::{error::MyError, merge_and_format::merge_and_format_results},
};
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use futures::TryStreamExt as _;
use std::collections::{HashMap, HashSet};
use tiberius::Query;
use tiberius_mappers::TryFromRow as _; // 引入 TryStreamExt
use tracing::info;
#[allow(unused_assignments)]
pub async fn get_box_datas(
    box_no: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    // 1. 初始条件检查保持不变
    if box_no.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err(MyError::AllNone);
    }

    // 2. 动态构建 SQL 语句和参数列表
    let mut sql_text = "
        SELECT sn, Pack_no, pn, creator, createtime
        FROM [mes_Factory].[dbo].[MaterialPackSn]
        WHERE PnOptionID = '-100'
    "
    .to_string();

    // 存储需要绑定的参数的引用
    let mut params: Vec<&str> = Vec::new();
    let mut param_counter = 1;

    // A. 盒号 (Pack_no) 条件
    if !box_no.is_empty() {
        sql_text.push_str(&format!(" AND Pack_no = @P{}", param_counter));
        params.push(&box_no);
        param_counter += 1;
    }

    // B. 料号 (pn) 条件
    if !pn.is_empty() {
        sql_text.push_str(&format!(" AND pn = @P{}", param_counter));
        params.push(&pn);
        param_counter += 1;
    }

    // C. 时间 (createtime) 条件
    if use_time {
        if date_time_start.is_empty() || date_time_end.is_empty() {
            return Err(MyError::Zdyknown(
                "使用时间查询时，开始时间和结束时间不能为空".to_string(),
            ));
        }
        sql_text.push_str(&format!(
            " AND createtime BETWEEN @P{} AND @P{}",
            param_counter,
            param_counter + 1
        ));
        params.push(&date_time_start);
        params.push(&date_time_end);
        param_counter += 2;
    }

    sql_text.push_str(" ORDER BY CreateTime DESC");

    info!("执行的SQL语句 (参数化): {}", sql_text);
    info!("Params: {:?}", params);

    // 这里我们使用 `params` 的切片作为参数。
    let mut query = Query::new(&sql_text);
    for param in params.into_iter() {
        query.bind(param);
    }
    let stream = query.query(&mut client).await?;
    let mut rows = stream.into_row_stream();
    while let Ok(Some(row)) = rows.try_next().await {
        let sn = row
            .get::<&str, _>(0)
            .ok_or_else(|| MyError::DataConversionError("查询结果中缺少 SN 信息".to_string()))?
            .to_string();

        if seen_sns.contains(&sn) {
            continue; // 跳过重复的 SN
        }

        // 注意：这里的 `box_no` 变量名与函数输入参数重名，为了清晰，我改用 `current_box_no`
        let current_box_no = row.get::<&str, _>(1)
            .ok_or_else(|| MyError::DataConversionError("缺少盒号信息".to_string()))?
            .to_string();
        let yypn = row.get::<&str, _>(2)
            .ok_or_else(|| MyError::DataConversionError("缺少料号信息".to_string()))?
            .to_string();
        let pack_worker = row.get::<&str, _>(3)
            .ok_or_else(|| MyError::DataConversionError("缺少包装工号".to_string()))?
            .to_string();
        let pack_time = row
            .get::<NaiveDateTime, _>(4)
            .ok_or_else(|| MyError::DataConversionError("缺少包装时间".to_string()))?
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let pack_data = PackData {
            box_no: current_box_no, // 使用正确的变量名
            pack_worker,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: sn.clone(),
            yypn,
            ..Default::default()
        };

        let data = Datas {
            pack_data,
            sn_data,
            ..Default::default()
        };
        all_datas.push(data);
        seen_sns.insert(sn);
    }
    let sns = all_datas
        .iter()
        .map(|d| format!("'{}'", d.sn_data.sn))
        .collect::<Vec<String>>()
        .join(",");
    let sn_datas = get_sn_info(sns, pool).await?;
    let carton_data = get_carton_data(&box_no, pool).await;
    let mut all = all_datas
        .into_iter()
        .map(|mut d| {
            if carton_data.is_some() {
                let carton_data_map = carton_data.as_ref().unwrap();
                d.carton_data.carton_no = carton_data_map.get("carton_no").unwrap().to_string();
            }

            d
        })
        .collect::<Vec<Datas>>();
    let mut all = merge_and_format_results(&mut all, &sn_datas);
    //     .into_iter()
    //     .map(|mut d| {
    //         let sn_datas = sn_datas
    //             .iter()
    //             .find(|x| x.sn == d.sn_data.sn)
    //             .map(|s| s.clone())
    //             .unwrap_or_default();
    //         d.sn_data = sn_datas;

    //         // 展平 Datas 为 HashMap
    //         let mut map = HashMap::new();
    //         // CartonData
    //         map.insert("carton_no".to_string(), d.carton_data.carton_no);
    //         map.insert("pch".to_string(), d.carton_data.pch);
    //         map.insert("yypn".to_string(), d.carton_data.yypn);
    //         map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
    //         map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
    //         // PackData
    //         map.insert("box_no".to_string(), d.pack_data.box_no);
    //         map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
    //         map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
    //         // BandData
    //         map.insert("w_sn".to_string(), d.band_data.w_sn);
    //         map.insert("b_sn".to_string(), d.band_data.b_sn);
    //         map.insert("bandtime".to_string(), d.band_data.band_time);
    //         map.insert("band_worker".to_string(), d.band_data.band_worker);
    //         // Data
    //         map.insert("sn".to_string(), d.sn_data.sn);
    //         map.insert("ith".to_string(), d.sn_data.ith);
    //         map.insert("vf".to_string(), d.sn_data.vf);
    //         map.insert("im".to_string(), d.sn_data.im);
    //         map.insert("po".to_string(), d.sn_data.po);
    //         map.insert("rs".to_string(), d.sn_data.rs);
    //         map.insert("se".to_string(), d.sn_data.se);
    //         map.insert("iop".to_string(), d.sn_data.iop);
    //         map.insert("kink".to_string(), d.sn_data.kink);
    //         map.insert("imkink".to_string(), d.sn_data.imkink);
    //         map.insert("sen".to_string(), d.sn_data.sen);
    //         map.insert("vbr".to_string(), d.sn_data.vbr);
    //         map.insert("res".to_string(), d.sn_data.res);
    //         map.insert("icc".to_string(), d.sn_data.icc);
    //         map.insert("idark".to_string(), d.sn_data.idark);
    //         map.insert("testdate".to_string(), d.sn_data.testdate.format("%Y-%m-%d %H:%M:%S").to_string());
    //         map.insert("result".to_string(), d.sn_data.result);
    //         map.insert("tester".to_string(), d.sn_data.tester);
    //         map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk);
    //         map.insert("mdpid".to_string(), d.sn_data.mdpid);
    //         map
    //     })
    //     .collect::<Vec<HashMap<String, String>>>();
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

async fn get_sn_info(
    sns: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Data>, MyError> {
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    let sql_text = build_query_sql(&sns, &pool).await?;
    let mut rows = client.simple_query(sql_text).await?.into_row_stream();
    info!("sn查询执行完毕，开始处理结果集...");
    let mut sn_map: HashMap<String, Data> = HashMap::new();
    let mut row_count = 0;
    while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
        row_count += 1;
        let data = Data::try_from_row(row)
            .map_err(|e| MyError::Zdyknown(format!("从行转换为 Data 结构体失败: {:?}", e)))?;
        // 使用 Entry API 避免重复 clone
        use std::collections::hash_map::Entry;
        match sn_map.entry(data.sn.clone()) {
            Entry::Occupied(mut entry) => {
                if entry.get().testdate < data.testdate {
                    entry.insert(data);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(data);
            }
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
async fn get_carton_data(
    box_no: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Option<HashMap<String, String>> {
    // 使用参数化查询，防止 SQL 注入
    let sql_text = "SELECT TOP 1 CartonNo FROM [mes_Factory].[dbo].[packing_carton] WHERE Packing_no = @P1";
    
    let mut client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return None,
    };
    
    let mut query = Query::new(sql_text);
    query.bind(box_no);
    
    let stream = match query.query(&mut client).await {
        Ok(s) => s,
        Err(_) => return None,
    };
    
    let row = match stream.into_row().await {
        Ok(r) => r,
        Err(_) => return None,
    };
    
    let mut carton_data = HashMap::new();
    if let Some(row) = row {
        if let Some(carton_no) = row.get::<&str, _>(0) {
            carton_data.insert("carton_no".to_string(), carton_no.to_string());
            carton_data.insert("box_no".to_string(), box_no.to_string());
        }
    }

    Some(carton_data)
}
