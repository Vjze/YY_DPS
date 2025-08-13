use std::collections::{HashMap, HashSet};

use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;

use crate::structs::{CartonData, Data, Datas, PackData};

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
) -> anyhow::Result<Vec<HashMap<String, String>>, String> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    if carton.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err("所有条件不能为空".to_string());
    }
    let sql_text = if !carton.is_empty() && pn.is_empty() && !use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton
        )
    } else if !carton.is_empty() && !pn.is_empty() && !use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100' and b.pn = '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, pn
        )
    } else if !carton.is_empty() && pn.is_empty() && use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, date_time_start, date_time_end
        )
    } else if carton.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.pn = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            pn, date_time_start, date_time_end
        )
    } else if !carton.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.pn = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, pn, date_time_start, date_time_end
        )
    } else if carton.is_empty() && pn.is_empty() && use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            date_time_start, date_time_end
        )
    } else if carton.is_empty() && !pn.is_empty() && !use_time {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.pn = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            pn
        )
    } else {
        format!(
            "select a.sn, a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc"
        )
    };
    println!("执行 SQL 查询: {}", sql_text);
    let stream = client
        .simple_query(sql_text)
        .await
        .map_err(|e| format!("查询箱号 '{}' 的 MaterialPackSn 失败: {}", carton, e))?;

    let rows = stream
        .into_results()
        .await
        .map_err(|e| format!("获取 MaterialPackSn 行失败: {}", e))?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(format!(
                        "箱号:{},没有找到sn信息，请注意箱号是否正确!!!",
                        carton
                    ));
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
            let carton_worker = row.get::<&str, _>(5).unwrap().to_string();
            let carton_time = row
                .get::<NaiveDateTime, _>(6)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let carton_no = row.get::<&str, _>(7).unwrap().to_string();
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
        return Err(format!("箱号 '{}' 没有找到数据", carton));
    }
    let datas = format_data(all_datas.clone());
    Ok(datas)
}
