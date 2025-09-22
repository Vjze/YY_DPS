use std::collections::{HashMap, HashSet}; // 引入 HashSet

use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use tokio::{
    fs,
    io::{AsyncBufReadExt as _, BufReader},
    task,
};

use super::query_utils::build_query_sql;
use crate::{
    configs::type_config::get_type_infos,
    export::works::sn_query::execute_query,
    structs::{BandData, CartonData, Data, Datas, PackData},
    utils::error::MyError,
};

async fn get_info() -> anyhow::Result<Vec<String>, MyError> {
    let pick = rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(MyError::Zdyknown(format!("选择框关闭，查询取消。")));
    let path = if let Ok(path) = pick {
        path.path().display().to_string()
    } else {
        return Err(MyError::Zdyknown(format!("未选择文件.")));
    };
    let op = fs::File::open(&path).await?;
    let mut infos = vec![];
    let mut reader = BufReader::new(op).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        infos.push(line);
    }

    Ok(infos)
}
pub async fn do_carton_query(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
    typeinfos: String,
    is_multi: bool,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    if carton.is_empty() && is_multi {
        let cartons = get_info().await?;
        let mut datas = Vec::new();
        for carton in cartons {
            let typeinfos = typeinfos.clone();
            let res = carton_query_datas(carton.clone(), &pool, typeinfos).await?;
            datas.extend(res);
        }

        Ok(datas)
    } else {
        carton_query_datas(carton, pool, typeinfos).await
    }
}
pub async fn carton_query_datas(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
    typeinfos: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    if carton.is_empty() {
        return Err(MyError::CartonNoEmpty);
    }
    let infos = get_type_infos(typeinfos).await?.1;
    // 将所有并存条件放入 match 元组中，处理所有组合
    let all_datas = match (
        infos.is_have_pch,
        infos.carton_pch,
        infos.zdy_box,
        infos.jz_band,
    ) {
        // --- 场景 A: is_have_pch = true ---

        // A.1: carton_pch = true (is_have_pch=true, carton_pch=true)
        (true, true, true, true) => {
            get_data_for_pch_carton_with_zdy_with_jzband(carton, pool).await?
        }
        (true, true, true, false) => {
            get_data_for_pch_carton_with_zdy_no_jzband(carton, pool).await?
        }
        (true, true, false, true) => {
            get_all_data_for_carton_with_pch_with_jzband(carton, pool).await?
        } // Existing
        (true, true, false, false) => get_all_data_for_carton_with_pch(carton, pool).await?, // Existing

        // A.2: box_pch = true (is_have_pch=true, carton_pch=false)
        (true, false, true, true) => {
            get_data_for_pch_box_with_zdy_with_jzband(carton, pool).await?
        }
        (true, false, true, false) => get_data_for_pch_box_with_zdy_no_jzband(carton, pool).await?,
        (true, false, false, true) => {
            get_all_data_for_box_with_pch_with_jzband(carton, pool).await?
        } // Existing
        (true, false, false, false) => get_all_data_for_box_with_pch(carton, pool).await?, // Existing

        // --- 场景 B: is_have_pch = false ---
        // 此时 carton_pch 和 box_pch 均为 false，因此第二个参数用 _ 通配
        (false, _, true, true) => get_data_for_no_pch_with_zdy_with_jzband(carton, pool).await?,
        (false, _, true, false) => get_data_for_no_pch_with_zdy_no_jzband(carton, pool).await?,
        (false, _, false, true) => get_data_no_pch_with_jzband(carton, pool).await?, // Existing
        (false, _, false, false) => get_data_no_pch(carton, pool).await?,            // Existing
    };
    

    // 查询每个 SN 的最新 TestDate
    let sn_placeholders = all_datas
        .iter()
        .map(|s| format!("'{}'", s.sn_data.sn))
        .collect::<Vec<String>>();

    let datas = if sn_placeholders.len() < 1000 {
        let sn_list = sn_placeholders.join(", ");
        let sql_text_s = build_query_sql(&sn_list, pool).await?;
        execute_query(&sql_text_s, pool).await?
    } else {
        // 创建拥有的 String，避免临时值
        let v1 = sn_placeholders[..1000].join(", ");
        let v2 = sn_placeholders[1000..].join(", ");
        let pool = pool.clone();
        let pool1 = pool.clone();
        // 使用 tokio::try_join! 并行执行
        let (data1, data2) = tokio::try_join!(
            task::spawn(async move {
                let sql_text_s_1 = build_query_sql(&v1, &pool).await?;
                execute_query(&sql_text_s_1, &pool).await
            }),
            task::spawn(async move {
                let sql_text_s_2 = build_query_sql(&v2, &pool1).await?;
                execute_query(&sql_text_s_2, &pool1).await
            })
        )
        .unwrap();

        // 合并结果
        let mut data1 = data1?;
        data1.extend(data2?);
        data1
    };

    // 合并 TestDate 数据并转换为 HashMap
    let mut all = all_datas
        .into_iter()
        .map(|mut d| {
            let sn_datas = datas
                .iter()
                .find(|x| x.sn == d.sn_data.sn)
                .map(|s| s.clone())
                .unwrap();
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
            map.insert("testtime".to_string(), d.sn_data.testtime);
            map.insert("result".to_string(), d.sn_data.result);
            map.insert("tester".to_string(), d.sn_data.tester);
            map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk);
            map.insert("mdpid".to_string(), d.sn_data.mdpid);
            map
        })
        .collect::<Vec<HashMap<String, String>>>();

    // 按 box_no 排序
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));
    Ok(all)
}

async fn get_data_for_no_pch_with_zdy_no_jzband(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_data_no_pch：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo='{}'
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query("select a.sn,c.pkg_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                inner join [mes_Factory].[dbo].[jz_carton_bind] c on a.Pack_no=c.box_no
                where b.CartonNo=@P1
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",&[&carton],
        )
        .await
        .unwrap();
    let rows = stream.into_results().await;
    match rows {
        Ok(rowsets) => {
            for i in 0..rowsets.len() {
                let rows = rowsets.get(i).unwrap();
                for row in rows {
                    let sn = match row.get::<&str, _>(0) {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(MyError::NoResult(format!("箱号:{carton}")));
                        }
                    };
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

                    // !!! 添加防重检查 !!!
                    if seen_sns.contains(&sn) {
                        // 如果有任何一个重复，则跳过当前数据
                        continue;
                    }

                    let carton_data = CartonData {
                        carton_no: carton.clone(),
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
                        sn: sn.clone(), // 克隆 sn 用于存储到 HashSet
                        ..Default::default()
                    };

                    let data = Datas {
                        carton_data,
                        pack_data,
                        sn_data,
                        ..Default::default()
                    };
                    all_datas.push(data);

                    // 将已添加的 sn, b_sn, w_sn 插入到 HashSet 中
                    seen_sns.insert(sn);
                }
            }
        }
        Err(_) => return Err(MyError::NoResult(format!("箱号:{carton}"))),
    };

    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
async fn get_data_for_no_pch_with_zdy_with_jzband(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    let stream = client
        .query(
            "select c.sn,a.pkg_no,c.pn,d.creator,c.createtime,b.creator,b.createtime
            from [mes_Factory].[dbo].[jz_carton_bind] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
            inner join [mes_Factory].[dbo].[MaterialPackSn] c on c.Pack_no=b.Packing_no
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await
        .unwrap();
    let rows = stream.into_results().await;
    match rows {
        Ok(rowsets) => {
            for i in 0..rowsets.len() {
                let rows = rowsets.get(i).unwrap();
                for row in rows {
                    let sn = match row.get::<&str, _>(0) {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(MyError::NoResult(format!("箱号:{carton}")));
                        }
                    };
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

                    // !!! 添加防重检查 !!!
                    if seen_sns.contains(&sn) {
                        // 如果有任何一个重复，则跳过当前数据
                        continue;
                    }

                    let carton_data = CartonData {
                        carton_no: carton.clone(),
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
                        sn: sn.clone(), // 克隆 sn 用于存储到 HashSet
                        ..Default::default()
                    };

                    let data = Datas {
                        carton_data,
                        pack_data,
                        sn_data,
                        ..Default::default()
                    };
                    all_datas.push(data);

                    // 将已添加的 sn, b_sn, w_sn 插入到 HashSet 中
                    seen_sns.insert(sn);
                }
            }
        }
        Err(_) => return Err(MyError::NoResult(format!("箱号:{carton}"))),
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}
async fn get_data_for_pch_box_with_zdy_with_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_data_for_pch_box_with_zdy_with_jzband：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select d.sn,a.pkg_no,d.pn,d.creator,d.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[jz_carton_bind] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on d.Pack_no=c.LABEL_KEY
            inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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

    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}
async fn get_data_for_pch_box_with_zdy_no_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_data_for_pch_box_with_zdy_no_jzband：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
    where b.CartonNo='{}' and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select d.sn,a.pkg_no,d.pn,d.creator,d.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[jz_carton_bind] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on d.Pack_no=c.LABEL_KEY
            inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
async fn get_data_for_pch_carton_with_zdy_no_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("get_data_for_pch_carton_with_zdy_no_jzband a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select d.sn,a.pkg_no,d.pn,d.creator,d.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[jz_carton_bind] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
async fn get_data_for_pch_carton_with_zdy_with_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("get_data_for_pch_carton_with_zdy_with_jzband a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select d.sn,a.pkg_no,d.pn,d.creator,d.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[jz_carton_bind] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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

    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}
async fn get_all_data_for_box_with_pch_with_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_all_data_for_box_with_pch_with_jzband：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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

    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}
async fn get_all_data_for_box_with_pch(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_all_data_for_box_with_pch：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
    where b.CartonNo='{}' and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
// 新增函数：根据已验证的 parameter 字符串获取所有数据
async fn get_all_data_for_carton_with_pch_with_jzband(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_all_data_for_carton_with_pch_with_jzband：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}
async fn get_all_data_for_carton_with_pch(
    carton: String,
    // verified_pchs_string: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_all_data_for_carton_with_pch：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
            where b.CartonNo='{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query(
            "select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime,c.parameter
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
            where b.CartonNo=@P1 and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
            &[&carton],
        )
        .await?;

    let rows = stream.into_results().await?;

    for rowset in rows {
        for row in rowset {
            let sn = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => {
                    return Err(MyError::NoResult(format!("箱号:{carton}")));
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
            let p = row.get::<&str, _>(7).unwrap().to_string();
            println!("完整信息：{}", p);
            let year = extract_value(&p, "YEAR");
            let week = extract_value(&p, "WEEK");
            let pch = match (year, week) {
                (Some(y), Some(w)) => {
                    // 只取 YEAR 的最后两位数并拼接 WEEK
                    let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                    let pch = format!("{}{}", year_last_two, w);
                    pch
                }
                _ => {
                    let pch = format!("{}", "None");
                    pch
                }
            };
            println!("批次号截取: {}", pch);
            let carton_data = CartonData {
                pch: pch.clone(), // 使用已经验证的 pch
                carton_no: carton.clone(),
                yypn,
                carton_worker,
                carton_packtime: carton_time,
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
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
async fn get_data_no_pch(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    println!("执行get_data_no_pch：select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo='{}'
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",carton);
    let stream = client
        .query("select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo=@P1
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",&[&carton],
        )
        .await
        .unwrap();
    let rows = stream.into_results().await;
    match rows {
        Ok(rowsets) => {
            for i in 0..rowsets.len() {
                let rows = rowsets.get(i).unwrap();
                for row in rows {
                    let sn = match row.get::<&str, _>(0) {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(MyError::NoResult(format!("箱号:{carton}")));
                        }
                    };
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

                    // !!! 添加防重检查 !!!
                    if seen_sns.contains(&sn) {
                        // 如果有任何一个重复，则跳过当前数据
                        continue;
                    }

                    let carton_data = CartonData {
                        carton_no: carton.clone(),
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
                        sn: sn.clone(), // 克隆 sn 用于存储到 HashSet
                        ..Default::default()
                    };

                    let data = Datas {
                        carton_data,
                        pack_data,
                        sn_data,
                        ..Default::default()
                    };
                    all_datas.push(data);

                    // 将已添加的 sn, b_sn, w_sn 插入到 HashSet 中
                    seen_sns.insert(sn);
                }
            }
        }
        Err(_) => return Err(MyError::NoResult(format!("箱号:{carton}"))),
    };

    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    Ok(all_datas)
}
async fn get_data_no_pch_with_jzband(
    carton: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<Datas>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    let stream = client
        .query("select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo=@P1
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",&[&carton],
        )
        .await
        .unwrap();
    let rows = stream.into_results().await;
    match rows {
        Ok(rowsets) => {
            for i in 0..rowsets.len() {
                let rows = rowsets.get(i).unwrap();
                for row in rows {
                    let sn = match row.get::<&str, _>(0) {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(MyError::NoResult(format!("箱号:{carton}")));
                        }
                    };
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

                    // !!! 添加防重检查 !!!
                    if seen_sns.contains(&sn) {
                        // 如果有任何一个重复，则跳过当前数据
                        continue;
                    }

                    let carton_data = CartonData {
                        carton_no: carton.clone(),
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
                        sn: sn.clone(), // 克隆 sn 用于存储到 HashSet
                        ..Default::default()
                    };

                    let data = Datas {
                        carton_data,
                        pack_data,
                        sn_data,
                        ..Default::default()
                    };
                    all_datas.push(data);

                    // 将已添加的 sn, b_sn, w_sn 插入到 HashSet 中
                    seen_sns.insert(sn);
                }
            }
        }
        Err(_) => return Err(MyError::NoResult(format!("箱号:{carton}"))),
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    Ok(a_datas)
}

async fn get_band_data(
    sn: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<BandData, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let stream = client
        .query(
            "select SN_ShipMent,Sn,userno,Relationtime from [mes_Factory].[dbo].[QA_snRelation] where sn=@P1",
            &[&sn],
        )
        .await?; // 添加错误处理
    let rowets = stream.into_row().await?; // 添加错误处理
    let (b_sn, w_sn, band_worker, band_time) = match rowets {
        Some(row) => {
            let b_sn = row.get::<&str, _>(0).unwrap_or_default().to_string();
            let w_sn = row.get::<&str, _>(1).unwrap_or_default().to_string();
            let band_worker = row.get::<&str, _>(2).unwrap_or_default().to_string();
            // 确保 band_time 字段存在且可解析，否则提供默认值
            let band_time = row.get::<&str, _>(3).unwrap_or_default().to_string();
            (b_sn, w_sn, band_worker, band_time)
        }
        None => (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ),
    };
    let band_data = BandData {
        w_sn: w_sn.clone(),
        b_sn: b_sn.clone(),
        band_time,
        band_worker,
    };
    Ok(band_data)
}
pub fn extract_value(input: &str, key: &str) -> Option<String> {
    let search_key = format!("{}=", key);
    if let Some(start_index) = input.find(&search_key) {
        let start = start_index + search_key.len();
        if let Some(end_index) = input[start..].find(';') {
            let end = start + end_index;
            return Some(input[start..end].to_string());
        } else {
            // 如果没有分号，则取到字符串末尾
            return Some(input[start..].to_string());
        }
    }
    None
}
