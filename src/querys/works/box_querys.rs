use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use crate::{structs::{Data, Datas, PackData}, utils::{error::MyError, sql::client}};

fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    let all = all_datas
        .into_iter()
        .map(|d| {
            // 展平 Datas 为 HashMap
            let mut map = HashMap::new();
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // Data
            map.insert("sn".to_string(), d.sn_data.sn);
            map.insert("yypn".to_string(), d.sn_data.yypn);
            map
        })
        .collect::<Vec<HashMap<String, String>>>();
    all
}

pub async fn get_box_datas(
    box_no: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
) -> anyhow::Result<Vec<HashMap<String,String>>,MyError> {
    let pool = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    if box_no.is_empty() && pn.is_empty() && date_time_start.is_empty() && date_time_end.is_empty() && !use_time {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }
    let sql_text = if !box_no.is_empty() && pn.is_empty() && !use_time {
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where Pack_no = '{}' and PnOptionID = '-100' 
            order by CreateTime desc",
            box_no
        )
    } else if !box_no.is_empty() && !pn.is_empty() && !use_time {
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where Pack_no = '{}' and PnOptionID = '-100' and pn = '{}'
            order by CreateTime desc",
            box_no, pn
        )
    } else if !box_no.is_empty() && pn.is_empty() && use_time {
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where Pack_no = '{}' and PnOptionID = '-100' and createtime between '{}' and '{}'
            order by CreateTime desc",
            box_no, date_time_start, date_time_end
        )
    } else if box_no.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where pn = '{}' and PnOptionID = '-100' and createtime between '{}' and '{}'
            order by CreateTime desc",
            pn, date_time_start, date_time_end
        )
    } else if !box_no.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where Pack_no = '{}' and pn = '{}' and PnOptionID = '-100' and createtime between '{}' and '{}'
            order by CreateTime desc",
            box_no, pn, date_time_start, date_time_end
        )
    } else if box_no.is_empty() && pn.is_empty() && use_time {
        // 新增：仅提供时间范围
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where PnOptionID = '-100' and createtime between '{}' and '{}'
            order by CreateTime desc",
            date_time_start, date_time_end
        )
    } else if box_no.is_empty() && !pn.is_empty() && !use_time {
        // 新增：仅提供 pn
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where pn = '{}' and PnOptionID = '-100'
            order by CreateTime desc",
            pn
        )
    } else {
        // 新增：无任何条件（全量查询）
        format!(
            "select sn, Pack_no, pn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where PnOptionID = '-100'
            order by CreateTime desc"
        )
    };
    let stream = client
        .simple_query(
            sql_text
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::Zdyknown(format!(
                        "盒号:{},没有找到sn信息，请注意箱号是否正确!!!",
                        box_no
                    )));
                }
            };
            if seen_sns.contains(&sn) {
                continue; // 跳过重复的 SN
            }
            
            let box_no = row.get::<&str, _>(1).unwrap().to_string();
            let yypn = row.get::<&str, _>(2).unwrap().to_string();
            let pack_worker = row.get::<&str, _>(3).unwrap().to_string();
            let pack_time = row
                .get::<NaiveDateTime, _>(4)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            
            let pack_data = PackData {
                box_no,
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
    }

    

    if all_datas.is_empty() {
        return Err(MyError::Zdyknown(format!("盒号 '{}' 没有找到数据", box_no)));
    }
    let datas = format_data(all_datas.clone());
    Ok(datas)
}