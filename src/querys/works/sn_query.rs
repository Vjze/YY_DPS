use std::collections::HashMap;

use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;

use super::query_utils::*;
use crate::{
    structs::{CartonData, Data, Datas, PackData},
    utils::error::MyError,
};

fn format_data(datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    let all = datas
        .into_iter()
        .map(|d| {
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
    all
}

pub async fn sn_query_datas(
    sns: Vec<String>,
    pn: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    test_result: String,
    test_devices: String,
    worker: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<HashMap<String, String>>, MyError> {
    let sn_list = if sns.is_empty() {
        println!("sns is empty");
        "".to_string()
    } else {
        let v: Vec<String> = sns.iter().map(|sn| format!("'{}'", sn)).collect();
        v.join(", ")
    };

    // 根据参数组合选择对应的 SQL 构建函数
    let sql_text_s = match (
        test_devices.as_str(),
        test_result.as_str(),
        use_time,
        pn.is_empty(),
        worker.is_empty(),
    ) {
        // test_devices == "全部"
        ("全部", "Ok", false, true, true) => build_query_sql(&sn_list, pool).await?,
        ("全部", "全部", false, true, true) => build_query_sql_res_all(&sn_list, pool).await?,
        ("全部", "NG", false, true, true) => build_query_sql_res_ng(&sn_list, pool).await?,
        ("全部", "Ok", true, true, true) => {
            build_query_sql_with_time(&sn_list, &date_time_start, &date_time_end, pool).await?
        }
        ("全部", "全部", true, true, true) => {
            build_query_sql_res_all_with_time(&sn_list, &date_time_start, &date_time_end, pool)
                .await?
        }
        ("全部", "NG", true, true, true) => {
            build_query_sql_res_ng_with_time(&sn_list, &date_time_start, &date_time_end, pool)
                .await?
        }
        ("全部", "Ok", false, false, true) => {
            build_query_sql_with_testtype(&sn_list, &pn, pool).await?
        }
        ("全部", "全部", false, false, true) => {
            build_query_sql_res_all_with_testtype(&sn_list, &pn, pool).await?
        }
        ("全部", "NG", false, false, true) => {
            build_query_sql_res_ng_with_testtype(&sn_list, &pn, pool).await?
        }
        ("全部", "Ok", true, false, true) => {
            build_query_sql_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("全部", "全部", true, false, true) => {
            build_query_sql_res_all_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("全部", "NG", true, false, true) => {
            build_query_sql_res_ng_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("全部", "Ok", false, true, false) => {
            build_query_sql_with_worker(&sn_list, &worker, pool).await?
        }
        ("全部", "全部", false, true, false) => {
            build_query_sql_res_all_with_worker(&sn_list, &worker, pool).await?
        }
        ("全部", "NG", false, true, false) => {
            build_query_sql_res_ng_with_worker(&sn_list, &worker, pool).await?
        }
        ("全部", "Ok", true, true, false) => {
            build_query_sql_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("全部", "全部", true, true, false) => {
            build_query_sql_res_all_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("全部", "NG", true, true, false) => {
            build_query_sql_res_ng_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("全部", "Ok", false, false, false) => {
            build_query_sql_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("全部", "全部", false, false, false) => {
            build_query_sql_res_all_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("全部", "NG", false, false, false) => {
            build_query_sql_res_ng_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("全部", "Ok", true, false, false) => {
            build_query_sql_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }
        ("全部", "全部", true, false, false) => {
            build_query_sql_res_all_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }
        ("全部", "NG", true, false, false) => {
            build_query_sql_res_ng_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }

        // test_devices == "10G"
        ("10G", "Ok", false, true, true) => build_query_sql_10g(&sn_list).await?,
        ("10G", "全部", false, true, true) => build_query_sql_10g_res_all(&sn_list).await?,
        ("10G", "NG", false, true, true) => build_query_sql_10g_res_ng(&sn_list).await?,
        ("10G", "Ok", true, true, true) => {
            build_query_sql_10g_with_time(&sn_list, &date_time_start, &date_time_end).await?
        }
        ("10G", "全部", true, true, true) => {
            build_query_sql_10g_res_all_with_time(&sn_list, &date_time_start, &date_time_end)
                .await?
        }
        ("10G", "NG", true, true, true) => {
            build_query_sql_10g_res_ng_with_time(&sn_list, &date_time_start, &date_time_end).await?
        }
        ("10G", "Ok", false, false, true) => {
            build_query_sql_10g_with_testtype(&sn_list, &pn).await?
        }
        ("10G", "全部", false, false, true) => {
            build_query_sql_10g_res_all_with_testtype(&sn_list, &pn).await?
        }
        ("10G", "NG", false, false, true) => {
            build_query_sql_10g_res_ng_with_testtype(&sn_list, &pn).await?
        }
        ("10G", "Ok", true, false, true) => {
            build_query_sql_10g_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
            )
            .await?
        }
        ("10G", "全部", true, false, true) => {
            build_query_sql_10g_res_all_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
            )
            .await?
        }
        ("10G", "NG", true, false, true) => {
            build_query_sql_10g_res_ng_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
            )
            .await?
        }
        ("10G", "Ok", false, true, false) => {
            build_query_sql_10g_with_worker(&sn_list, &worker).await?
        }
        ("10G", "全部", false, true, false) => {
            build_query_sql_10g_res_all_with_worker(&sn_list, &worker).await?
        }
        ("10G", "NG", false, true, false) => {
            build_query_sql_10g_res_ng_with_worker(&sn_list, &worker).await?
        }
        ("10G", "Ok", true, true, false) => {
            build_query_sql_10g_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
            )
            .await?
        }
        ("10G", "全部", true, true, false) => {
            build_query_sql_10g_res_all_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
            )
            .await?
        }
        ("10G", "NG", true, true, false) => {
            build_query_sql_10g_res_ng_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
            )
            .await?
        }
        ("10G", "Ok", false, false, false) => {
            build_query_sql_10g_with_testtype_and_worker(&sn_list, &pn, &worker).await?
        }
        ("10G", "全部", false, false, false) => {
            build_query_sql_10g_res_all_with_testtype_and_worker(&sn_list, &pn, &worker).await?
        }
        ("10G", "NG", false, false, false) => {
            build_query_sql_10g_res_ng_with_testtype_and_worker(&sn_list, &pn, &worker).await?
        }
        ("10G", "Ok", true, false, false) => {
            build_query_sql_10g_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
            )
            .await?
        }
        ("10G", "全部", true, false, false) => {
            build_query_sql_10g_res_all_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
            )
            .await?
        }
        ("10G", "NG", true, false, false) => {
            build_query_sql_10g_res_ng_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
            )
            .await?
        }

        // test_devices == "2.5G"
        ("2.5G", "Ok", false, true, true) => build_query_sql_2(&sn_list, pool).await?,
        ("2.5G", "全部", false, true, true) => build_query_sql_2_res_all(&sn_list, pool).await?,
        ("2.5G", "NG", false, true, true) => build_query_sql_2_res_ng(&sn_list, pool).await?,
        ("2.5G", "Ok", true, true, true) => {
            build_query_sql_2_with_time(&sn_list, &date_time_start, &date_time_end, pool).await?
        }
        ("2.5G", "全部", true, true, true) => {
            build_query_sql_2_res_all_with_time(&sn_list, &date_time_start, &date_time_end, pool)
                .await?
        }
        ("2.5G", "NG", true, true, true) => {
            build_query_sql_2_res_ng_with_time(&sn_list, &date_time_start, &date_time_end, pool)
                .await?
        }
        ("2.5G", "Ok", false, false, true) => {
            build_query_sql_2_with_testtype(&sn_list, &pn, pool).await?
        }
        ("2.5G", "全部", false, false, true) => {
            build_query_sql_2_res_all_with_testtype(&sn_list, &pn, pool).await?
        }
        ("2.5G", "NG", false, false, true) => {
            build_query_sql_2_res_ng_with_testtype(&sn_list, &pn, pool).await?
        }
        ("2.5G", "Ok", true, false, true) => {
            build_query_sql_2_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("2.5G", "全部", true, false, true) => {
            build_query_sql_2_res_all_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("2.5G", "NG", true, false, true) => {
            build_query_sql_2_res_ng_with_time_and_testtype(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                pool,
            )
            .await?
        }
        ("2.5G", "Ok", false, true, false) => {
            build_query_sql_2_with_worker(&sn_list, &worker, pool).await?
        }
        ("2.5G", "全部", false, true, false) => {
            build_query_sql_2_res_all_with_worker(&sn_list, &worker, pool).await?
        }
        ("2.5G", "NG", false, true, false) => {
            build_query_sql_2_res_ng_with_worker(&sn_list, &worker, pool).await?
        }
        ("2.5G", "Ok", true, true, false) => {
            build_query_sql_2_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("2.5G", "全部", true, true, false) => {
            build_query_sql_2_res_all_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("2.5G", "NG", true, true, false) => {
            build_query_sql_2_res_ng_with_time_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &worker,
                pool,
            )
            .await?
        }
        ("2.5G", "Ok", false, false, false) => {
            build_query_sql_2_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("2.5G", "全部", false, false, false) => {
            build_query_sql_2_res_all_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("2.5G", "NG", false, false, false) => {
            build_query_sql_2_res_ng_with_testtype_and_worker(&sn_list, &pn, &worker, pool).await?
        }
        ("2.5G", "Ok", true, false, false) => {
            build_query_sql_2_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }
        ("2.5G", "全部", true, false, false) => {
            build_query_sql_2_res_all_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }
        ("2.5G", "NG", true, false, false) => {
            build_query_sql_2_res_ng_with_time_and_testtype_and_worker(
                &sn_list,
                &date_time_start,
                &date_time_end,
                &pn,
                &worker,
                pool,
            )
            .await?
        }

        _ => return Err(MyError::QueryErr),
    };

    let data = execute_query_sn(&sql_text_s, pool).await?;
    let datas = get_all_datas(pool, data).await?;
    let datas = format_data(datas);
    Ok(datas)
}
async fn get_all_datas(
    pool: &bb8::Pool<ConnectionManager>,
    sn_data: Vec<Data>,
) -> Result<Vec<Datas>, MyError> {
    println!("开始查询Box");
    let mut datas_list = vec![];
    let mut box_nos = vec![];
    let sns = sn_data
        .iter()
        .map(|x| x.sn.clone())
        .collect::<Vec<String>>();
    let sn = sns.join(", ");
    let mut client = pool.get().await.unwrap();
    let sql_text_s = if sns.len() == 1 {
        format!(
            "select Pack_no, Sn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where sn IN ('{}') and PnOptionID = '-100'
            order by CreateTime desc, Pack_no asc",
            sn
        )
    } else {
        format!(
            "select Pack_no, Sn, creator, createtime
            from [mes_Factory].[dbo].[MaterialPackSn]
            where sn IN ({}) and PnOptionID = '-100'
            order by CreateTime desc, Pack_no asc",
            sn
        )
    };
    println!("查询Box的SQL: {}", sql_text_s);
    let stream = client.simple_query(sql_text_s).await?;
    let rows = stream.into_results().await?;
    for rowset in rows {
        for row in rowset {
            let box_no = match row.get::<&str, _>(0) {
                Some(s) => s.to_string(),
                None => "".to_string(),
            };
            if box_no.is_empty() {
                break;
            }
            let sn = row.get::<&str, _>(1).unwrap().to_string();
            let pack_worker = row.get::<&str, _>(2).unwrap().to_string();
            let pack_time = row
                .get::<NaiveDateTime, _>(3)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

            let pack_data = PackData {
                box_no: box_no.clone(),
                pack_worker,
                pack_packtime: pack_time,
            };
            let sn_data = sn_data
                .iter()
                .find(|x| x.sn == sn)
                .map(|x| x.clone())
                .unwrap();
            let datas = Datas {
                pack_data: pack_data,
                sn_data,
                ..Default::default()
            };
            datas_list.push(datas);
            box_nos.push(box_no.clone());
        }
    }
    println!("查询到的Box数量: {}", box_nos.len());

    if !box_nos.is_empty() {
        println!("开始查询Caroton");
        let box_no = datas_list
            .iter()
            // .find(|s| s.pack_data.box_no == box_no)
            .map(|x| x.pack_data.box_no.clone())
            .collect::<Vec<String>>()
            .join(", ");

        let sql_text_s = if box_nos.len() == 1 {
            format!(
                "select CartonNo, Packing_no from [mes_Factory].[dbo].[packing_carton] 
            where Packing_no in ('{}') and PnOptionID = '-100'",
                box_no
            )
        } else {
            format!(
                "select CartonNo, Packing_no from [mes_Factory].[dbo].[packing_carton] 
            where Packing_no in ({}) and PnOptionID = '-100'",
                box_no
            )
        };
        let stream = client.simple_query(sql_text_s).await?;

        let rows = stream.into_results().await?;
        for rowset in rows {
            for row in rowset {
                let carton_no = match row.get::<&str, _>(0) {
                    Some(s) => s.to_string(),
                    None => "".to_string(),
                };
                let box_no = row.get::<&str, _>(1).unwrap().to_string();
                let carton_data = CartonData {
                    carton_no,
                    ..Default::default()
                };
                let box_data = datas_list
                    .iter()
                    .find(|x| x.pack_data.box_no == box_no)
                    .map(|x| x.pack_data.clone())
                    .unwrap();
                let sn_data = datas_list
                    .iter()
                    .find(|x| x.pack_data.box_no == box_no)
                    .map(|x| x.sn_data.clone())
                    .unwrap();
                let datas = Datas {
                    pack_data: box_data,
                    carton_data: carton_data,
                    sn_data,
                    ..Default::default()
                };
                datas_list.push(datas);
            }
        }

        Ok(datas_list)
    } else {
        Ok(Vec::new())
    }
}
pub async fn execute_query(
    sql_text_s: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    // println!("执行查询: {}", sql_text_s);
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let stream = client.query(sql_text_s, &[&1i32]).await?;
    let rowsets = stream.into_results().await?;

    let mut sn_map: HashMap<String, Data> = HashMap::new();
    for rows in rowsets {
        for row in rows {
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
    if datas.is_empty() {
        return Err(MyError::NoResult(format!("")));
    }
    Ok(datas)
}

pub async fn execute_query_sn(
    sql_text_s: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    println!("执行查询: {}", sql_text_s);
    let mut client = pool.get().await.unwrap();
    // let mut client = client().await?;
    let stream = client.query(sql_text_s, &[&1i32]).await?;
    let rowsets = stream.into_results().await?;

    let mut sn_map: HashMap<String, Data> = HashMap::new();
    for rows in rowsets {
        for row in rows {
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

            sn_map.insert(sn.clone(), data);
        }
    }

    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();

    if datas.is_empty() {
        return Err(MyError::NoResult("箱号".to_string()));
    }
    Ok(datas)
}
