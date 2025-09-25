use std::collections::{HashMap, HashSet}; // 引入 HashSet

use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use tokio::{
    fs,
    io::{AsyncBufReadExt as _, BufReader},
    task,
};
use tracing::info;

use crate::{
    configs::type_config::get_type_infos,
    structs::{BandData, CartonData, Data, Datas, PackData},
    utils::{error::MyError, sql::{client, get_tables}},
};

async fn get_info() -> anyhow::Result<Vec<String>, MyError> {
    info!("开始执行文件选择...");
    let pick = rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(MyError::Zdyknown(format!("选择框关闭，查询取消。")));
    let path = if let Ok(path) = pick {
        let p_str = path.path().display().to_string();
        info!("文件选择成功: path={}", p_str);
        p_str
    } else {
        return Err(MyError::Zdyknown(format!("未选择文件.")));
    };
    let op = fs::File::open(&path).await?;
    let mut infos = vec![];
    let mut reader = BufReader::new(op).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        infos.push(line);
    }
    info!("文件内容读取完毕，共 {} 行", infos.len());
    Ok(infos)
}
pub async fn do_carton_query(
    carton: String,
    typeinfos: String,
    is_multi: bool,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    info!("开始执行箱号查询: carton={}, typeinfos={}, is_multi={}", carton, typeinfos, is_multi);
    let client = client().await?;
    let pool = &client;
    if carton.is_empty() && is_multi {
        info!("执行批量查询模式");
        let cartons = get_info().await?;
        let mut datas = Vec::new();
        for carton in cartons {
            let typeinfos = typeinfos.clone();
            let res = carton_query_datas(carton.clone(), &pool, typeinfos).await?;
            datas.extend(res);
        }
        Ok(datas)
    } else {
        info!("执行单箱查询模式");
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
    info!("类型信息解析完毕: {:?}", infos);
    // 将所有并存条件放入 match 元组中，处理所有组合
    let all_datas = match (
        infos.is_have_pch,
        infos.carton_pch,
        infos.zdy_box,
        infos.jz_band,
    ) {
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
    
    info!("基础数据获取完毕，开始查询最新测试数据");
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
        info!("SN数量超过1000，将执行并行查询");
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
    info!("测试数据获取完毕，开始整合数据");
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
    info!("数据整合完毕，开始排序");
    // 按 box_no 排序
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));
    info!("排序完毕，查询结束");
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
    info!("开始执行 get_data_for_no_pch_with_zdy_no_jzband: select a.sn,c.pkg_no... where b.CartonNo='{}'", carton);
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
    info!("SQL查询执行完毕，开始处理结果集");
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行 get_data_for_no_pch_with_zdy_with_jzband: select c.sn,a.pkg_no... where b.CartonNo='{}'", carton);
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
    info!("SQL查询执行完毕，开始处理结果集");
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
    info!("基础数据处理完毕，开始获取绑定数据");
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
    info!("绑定数据获取完毕，开始整合");
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
    info!("数据整合完毕，准备返回结果");
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
    info!("开始执行 get_data_for_pch_box_with_zdy_with_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'", carton);
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

    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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

    info!("基础数据处理完毕，开始获取绑定数据");
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
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
    info!("数据整合完毕，准备返回结果");
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
    info!("开始执行 get_data_for_pch_box_with_zdy_no_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'", carton);
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
    
    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行 get_data_for_pch_carton_with_zdy_no_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'", carton);
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

    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行get_data_for_pch_carton_with_zdy_with_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'",carton);
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
    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("基础数据处理完毕，开始获取绑定信息");
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定信息获取完毕，开始整合数据");
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
    info!("数据整合完毕，准备返回结果");
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
    info!("开始执行 get_all_data_for_box_with_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
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
    
    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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

    info!("基础数据处理完毕，开始获取绑定数据");
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
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
    info!("数据整合完毕，准备返回结果");
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
    info!("开始执行 get_all_data_for_box_with_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
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

    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行 get_all_data_for_carton_with_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
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

    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("基础数据处理完毕，开始获取绑定数据");
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
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
    info!("数据整合完毕，准备返回结果");
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
    info!("开始执行 get_all_data_for_carton_with_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
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

    info!("SQL查询执行完毕，开始处理结果集");
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
            // info!("完整信息：{}", p);
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
            // info!("批次号截取: {}", pch);
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行 get_data_no_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
    let stream = client
        .query("select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo=@P1
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",&[&carton],
        )
        .await
        .unwrap();
    info!("SQL查询执行完毕，开始处理结果集");
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
    info!("数据处理完毕，准备返回结果");
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
    info!("开始执行 get_data_no_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'", carton);
    let stream = client
        .query("select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime
                from [mes_Factory].[dbo].[MaterialPackSn] a
                inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
                where b.CartonNo=@P1
                and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",&[&carton],
        )
        .await
        .unwrap();
    info!("SQL查询执行完毕，开始处理结果集");
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
    info!("基础数据处理完毕，开始获取绑定数据");
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
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
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}

async fn get_band_data(
    sn: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<BandData, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    info!("开始为SN: {} 查询绑定数据", sn);
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
    info!("SN: {} 绑定数据查询完毕", sn);
    Ok(band_data)
}

pub async fn execute_query(
    sql_text_s: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    info!("开始执行 carton_query 查询: {}", sql_text_s);
    let mut client = pool.get().await.unwrap();
    let stream = client.query(sql_text_s, &[&1i32]).await?;
    info!("查询执行完毕，开始处理结果集...");
    let rowsets = stream.into_results().await?;

    let mut sn_map: HashMap<String, Data> = HashMap::new();
    let mut row_count = 0;
    for rows in rowsets {
        for row in rows {
            row_count += 1;
            let sn = row.get::<&str, _>(0).unwrap().to_string();
            let kink = row.get::<&str, _>(11).unwrap_or_default();
            let imkink = row.get::<&str, _>(12).unwrap_or_default();
            let mdpid = if row.get::<&str, _>(19).unwrap_or_default() == "0" {
                "".to_string()
            } else {
                row.get::<&str, _>(19).unwrap_or_default().to_string()
            };
            let yypn = row.get::<&str, _>(20).unwrap_or_default().to_string();
            let data = Data {
                sn: sn.clone(),
                ith: row.get::<&str, _>(1).unwrap_or_default().to_string(),
                vf: row.get::<&str, _>(3).unwrap_or_default().to_string(),
                im: row.get::<&str, _>(4).unwrap_or_default().to_string(),
                po: row.get::<&str, _>(2).unwrap_or_default().to_string(),
                rs: row.get::<&str, _>(5).unwrap_or_default().to_string(),
                se: row.get::<&str, _>(6).unwrap_or_default().to_string(),
                sen: row.get::<&str, _>(7).unwrap_or_default().to_string(),
                res: row.get::<&str, _>(8).unwrap_or_default().to_string(),
                icc: row.get::<&str, _>(9).unwrap_or_default().to_string(),
                vbr: row.get::<&str, _>(10).unwrap_or("0.00").to_string(),
                kink: kink.to_string(),
                imkink: imkink.to_string(),
                testtime: row
                    .get::<NaiveDateTime, _>(13)
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                tester: row.get::<&str, _>(16).unwrap_or_default().to_string(),
                iop: row.get::<&str, _>(17).unwrap_or_default().to_string(),
                idark: row.get::<&str, _>(14).unwrap_or_default().to_string(),
                result: row.get::<&str, _>(15).unwrap_or_default().to_string(),
                i_xtalk: row.get::<&str, _>(18).unwrap_or_default().to_string(),
                mdpid,
                yypn,
            };
            // 只保留最新的测试数据
            if let Some(existing_data) = sn_map.get(&sn) {
                if existing_data.testtime < data.testtime {
                    sn_map.insert(sn.clone(), data);
                }
            } else {
                sn_map.insert(sn.clone(), data);
            }
        }
    }
    
    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();
    info!("共处理 {} 行原始数据，去重后得到 {} 条最新SN数据。", row_count, datas.len());
    if datas.is_empty() {
        return Err(MyError::NoResult(format!("")));
    }
    Ok(datas)
}
pub async fn build_query_sql(
    sn_list: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<String, MyError> {
    // 定义字段
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'OK' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
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