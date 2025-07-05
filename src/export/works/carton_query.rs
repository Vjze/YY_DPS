use crate::{structs::{BandData, CartonData, Data, Datas, PackData}, utils::{error::MyError, sql::client}};

use super::query_utils::{build_query_sql, execute_query};

use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet}; // 引入 HashSet

// 辅助函数：尝试从 LABEL_KEY 对应的 parameter 中提取 pch
async fn try_extract_pch_from_label_key(
    client: &mut tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    label_key: &str,
) -> anyhow::Result<Option<String>, MyError> {
    let stream = client
        .query(
            "select parameter from [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] where LABEL_KEY=@P1",
            &[&label_key],
        )
        .await?;

    let row = stream.into_row().await?;

    if let Some(row) = row {
        let pchs = row.get::<&str, _>(0).unwrap_or_default().to_string();
        if !pchs.is_empty() {
            let year = extract_value(&pchs, "YEAR");
            let week = extract_value(&pchs, "WEEK");
            if let (Some(y), Some(w)) = (year, week) {
                let year_last_two = &y[y.len() - 2..];
                return Ok(Some(format!("{}{}", year_last_two, w)));
            }
        }
    }
    Ok(None) // 没有找到有效的 pch
}

pub async fn carton_query_datas(
    carton: String,
    tables: Vec<String>
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {

    let mut client = client().await?;
    let mut actual_pch_source_carton = None; // 存储最终确定 pch 的 carton/box_no 值

    // 1. 优先尝试直接用 carton 作为 LABEL_KEY 查找 parameter 并提取 pch
    println!("尝试将箱号 '{}' 作为 LABEL_KEY 查询批次号...", carton);
    if let Some(pch) = try_extract_pch_from_label_key(&mut client, &carton).await? {
        println!("成功通过箱号 '{}' 获取到批次号: {}", carton, pch);
        actual_pch_source_carton = Some(carton.clone());
    } else {
        println!("未通过箱号 '{}' 获取到有效批次号。尝试通过盒号查找...", carton);
        // 2. 如果直接用 carton 没找到有效 pch，则查找关联的盒号
        let box_nos_stream = client
            .query(
                "select Packing_no from [mes_Factory].[dbo].[packing_carton] where CartonNo=@P1",
                &[&carton],
            )
            .await?;

        let box_no_rows = box_nos_stream.into_results().await?;

        for rowset in box_no_rows {
            for row in rowset {
                let box_no = row.get::<&str, _>(0).unwrap_or_default().to_string();
                if box_no.is_empty() {
                    continue;
                }
                println!("尝试将盒号 '{}' 作为 LABEL_KEY 查询批次号...", box_no);
                if let Some(pch) = try_extract_pch_from_label_key(&mut client, &box_no).await? {
                    println!("成功通过盒号 '{}' 获取到批次号: {}", box_no, pch);
                    actual_pch_source_carton = Some(box_no); // 使用找到有效 pch 的盒号
                    break; // 找到第一个有效 pch 就退出循环
                }
            }
            if actual_pch_source_carton.is_some() {
                break; // 找到第一个有效 pch 就退出外层循环
            }
        }
    }

    let all_datas = if let Some(source_key) = actual_pch_source_carton {
        let stream_param = client
            .query(
                "select parameter from [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] where LABEL_KEY=@P1",
                &[&source_key], // 使用找到有效 pch 的 LABEL_KEY
            )
            .await?;

        let row_param = stream_param.into_row().await?;
        
        let pchs_final = row_param.map(|r| r.get::<&str, _>(0).unwrap_or_default().to_string()).unwrap_or_default();
        
        // 现在 pchs_final 包含了正确的 parameter 字符串，我们可以用它来驱动查询
        println!("最终确定的有效批次号参数字符串为: '{}'", pchs_final);

        // 调用 get_all_data_for_carton_with_pch 确保使用正确的 pch 来源
        get_all_data_for_carton_with_pch(carton.clone(), pchs_final).await?

    } else {
        println!("未找到有效批次号，回退到不带批次号的查询。");
        get_data_no_pch(carton.clone()).await?
    };

    // 查询每个 SN 的最新 TestDate
    let sn_placeholders = all_datas
        .iter()
        .map(|s| format!("'{}'", s.sn_data.sn))
        .collect::<Vec<String>>();
    let sn_list = sn_placeholders.join(", ");
    let sql_text_s = build_query_sql(&sn_list, tables).await?;
    let datas = execute_query(&sql_text_s).await?;

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
            map
        })
        .collect::<Vec<HashMap<String, String>>>();

    // 按 box_no 排序
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));
    Ok(all)
}

// 新增函数：根据已验证的 parameter 字符串获取所有数据
async fn get_all_data_for_carton_with_pch(
    carton: String,
    verified_pchs_string: String,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = client().await?;
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn

    let year = extract_value(&verified_pchs_string, "YEAR");
    let week = extract_value(&verified_pchs_string, "WEEK");
    let pch = match (year, week) {
        (Some(y), Some(w)) => {
            let year_last_two = &y[y.len() - 2..];
            format!("{}{}", year_last_two, w)
        }
        _ => "".to_string(), // 如果无法解析，则 pch 为空
    };

    let stream = client
        .query(
            "select a.sn,a.Pack_no,a.pn,a.creator,a.createtime,b.creator,b.createtime 
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no 
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
                    return Err(MyError::NoResult(format!("箱号：{}",carton))
                    )
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

    for data in &mut all_datas {
        let band_data = get_band_data(data.sn_data.sn.clone()).await?;
        if band_data == Default::default() {
            // 如果 band_data 为默认值，表示未找到绑定数据，可以根据业务需求选择跳过或报错
            // 这里选择跳过，确保只处理有完整绑定数据的 SN
            continue; 
        }
        data.band_data = band_data;
    }

    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号: '{}'", carton)));
    }
    Ok(all_datas)
}

async fn get_data_no_pch(carton: String) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut client = client().await?;
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
                            return Err(MyError::NoResult(format!(
                                "箱号:{}",
                                carton
                            )))
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
        Err(_) => return Err(MyError::NoResult("".to_string())),
    }
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone()).await?;
        if band_data == Default::default() {
            break;
        }
        data.band_data = band_data;
    }
    if all_datas.is_empty() {
        return Err(MyError::NoResult("".to_string()));
    }
    Ok(all_datas)
}

// 移除 get_pch_in_box 和 get_pch_in_carton 函数，它们现在被新的逻辑取代。
// 如果您的其他部分代码还在直接调用它们，您需要将这些调用改为 carton_query_datas。

async fn get_band_data(sn: String) -> Result<BandData, MyError> {
    let mut client = client().await?;
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
