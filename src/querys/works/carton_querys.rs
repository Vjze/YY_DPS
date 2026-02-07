use std::collections::{HashMap, HashSet};

use crate::{
    structs::{CartonData, Data, Datas, PackData},
    utils::{error::MyError, query_builder::QueryBuilder},
};
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use tiberius::Query;

fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    let all = all_datas
        .into_iter()
        .map(|d| {
            // 展平 Datas 为 HashMap
            let mut map = HashMap::new();
            // CartonData
            map.insert("carton_no".to_string(), d.carton_data.carton_no);
            map.insert("yypn".to_string(), d.carton_data.yypn);
            map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
            map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // Data
            map.insert("sn".to_string(), d.sn_data.sn);

            map
        })
        .collect::<Vec<HashMap<String, String>>>();
    all
}

pub async fn get_carton_datas(
    carton: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    if carton.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }
    // 使用参数化查询构建器，防止 SQL 注入
    let base_sql = "SELECT a.sn, a.Pack_no, a.pn, a.creator, a.createtime, 
                    b.creator, b.createtime, b.CartonNo
                    FROM [mes_Factory].[dbo].[MaterialPackSn] a
                    INNER JOIN [mes_Factory].[dbo].[packing_carton] b 
                    ON a.Pack_no = b.Packing_no";
    
    let (sql_text, params) = QueryBuilder::new(base_sql)
        .add_raw("b.PnOptionID = '-100'")
        .add_equals("b.CartonNo", &carton)
        .add_equals("b.pn", &pn)
        .add_between("b.createtime", &date_time_start, &date_time_end)
        .add_order_by(&["b.CreateTime DESC", "b.Packing_no DESC", "a.Pack_no ASC"])
        .build();
    
    tracing::info!("执行 SQL 查询: {}, 参数: {:?}", sql_text, params);
    
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    
    // 构建参数化查询
    let mut query = Query::new(&sql_text);
    for param in &params {
        query.bind(param.as_str());
    }
    
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::DbConnectionError(e))?;

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

            let box_no = row.get::<&str, _>(1)
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
            let carton_worker = row.get::<&str, _>(5)
                .ok_or_else(|| MyError::DataConversionError("缺少装箱工号".to_string()))?
                .to_string();
            let carton_time = row
                .get::<NaiveDateTime, _>(6)
                .ok_or_else(|| MyError::DataConversionError("缺少装箱时间".to_string()))?
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let carton_no = row.get::<&str, _>(7)
                .ok_or_else(|| MyError::DataConversionError("缺少箱号信息".to_string()))?
                .to_string();
            let carton_data = CartonData {
                carton_no,
                yypn,
                carton_worker,
                carton_packtime: carton_time,
                ..Default::default()
            };
            let pack_data = PackData {
                box_no,
                pack_worker,
                pack_packtime: pack_time,
            };
            let sn_data = Data {
                sn: sn.clone(),
                ..Default::default()
            };

            let data = Datas {
                carton_data,
                pack_data,
                sn_data,
                ..Default::default()
            };
            all_datas.push(data);
            seen_sns.insert(sn);
        }
    }

    if all_datas.is_empty() {
        return Err(MyError::Zdyknown(format!("箱号 '{}' 没有找到数据", carton)));
    }
    let datas = format_data(all_datas.clone());
    Ok(datas)
}
