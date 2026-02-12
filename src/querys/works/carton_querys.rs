use std::collections::{HashMap, HashSet};

use crate::{
    structs::{CartonData, Data, Datas, PackData},
    utils::error::MyError,
};
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;

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
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_box = HashSet::new(); // 用于存储已见的 sn
    if carton.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }
    let sql_text = if !carton.is_empty() && pn.is_empty() && !use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton
        )
    } else if !carton.is_empty() && !pn.is_empty() && !use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100' and b.pn = '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, pn
        )
    } else if !carton.is_empty() && pn.is_empty() && use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, date_time_start, date_time_end
        )
    } else if carton.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.pn = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            pn, date_time_start, date_time_end
        )
    } else if !carton.is_empty() && !pn.is_empty() && use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.pn = '{}' and b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            carton, pn, date_time_start, date_time_end
        )
    } else if carton.is_empty() && pn.is_empty() && use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.PnOptionID = '-100' and b.createtime between '{}' and '{}'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            date_time_start, date_time_end
        )
    } else if carton.is_empty() && !pn.is_empty() && !use_time {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.pn = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            pn
        )
    } else {
        format!(
            "select  a.Pack_no, a.pn, a.creator, a.createtime, b.creator, b.createtime, b.CartonNo
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc"
        )
    };
    println!("执行 SQL 查询: {}", sql_text);
    let mut pool = pool.get().await.unwrap();

    let stream = pool.simple_query(sql_text).await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let box_no = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::Zdyknown(format!(
                        "箱号:{},没有找到sn信息，请注意箱号是否正确!!!",
                        carton
                    )));
                }
            };
            if seen_box.contains(&box_no) {
                continue; // 跳过重复的 SN
            }

            // let box_no = row.get::<&str, _>(1).unwrap().to_string();
            let yypn = row.get::<&str, _>(1).unwrap().to_string();
            let pack_worker = row.get::<&str, _>(2).unwrap().to_string();
            let pack_time = row
                .get::<NaiveDateTime, _>(3)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let carton_worker = row.get::<&str, _>(4).unwrap().to_string();
            let carton_time = row
                .get::<NaiveDateTime, _>(5)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let carton_no = row.get::<&str, _>(6).unwrap().to_string();
            let carton_data = CartonData {
                carton_no,
                yypn,
                carton_worker,
                carton_packtime: carton_time,
                ..Default::default()
            };
            let pack_data = PackData {
                box_no: box_no.clone(),
                pack_worker,
                pack_packtime: pack_time,
            };
            // let sn_data = Data {
            //     sn: sn.clone(),
            //     ..Default::default()
            // };

            let data = Datas {
                carton_data,
                pack_data,
                // sn_data,
                ..Default::default()
            };
            all_datas.push(data);
            seen_box.insert(box_no);
        }
    }

    if all_datas.is_empty() {
        return Err(MyError::Zdyknown(format!("箱号 '{}' 没有找到数据", carton)));
    }
    all_datas.sort_by(|a, b| a.pack_data.box_no.cmp(&b.pack_data.box_no));
    // let datas = format_data(all_datas.clone());
    Ok(all_datas)
}
