// sn_query.rs

use std::collections::HashMap;
use bb8_tiberius::ConnectionManager;
use chrono::NaiveDateTime;
use tracing::info; // 引入 info!

use super::query_utils::*;
use crate::{structs::Data, utils::{error::MyError, sql::client}};

fn format_data(data: Vec<Data>) -> Vec<HashMap<String, String>> {
    data.into_iter()
        .map(|d| {
            let mut map = HashMap::new();
            map.insert("sn".to_string(), d.sn);
            map.insert("ith".to_string(), d.ith);
            map.insert("vf".to_string(), d.vf);
            map.insert("im".to_string(), d.im);
            map.insert("po".to_string(), d.po);
            map.insert("rs".to_string(), d.rs);
            map.insert("se".to_string(), d.se);
            map.insert("sen".to_string(), d.sen);
            map.insert("res".to_string(), d.res);
            map.insert("icc".to_string(), d.icc);
            map.insert("vbr".to_string(), d.vbr);
            map.insert("kink".to_string(), d.kink);
            map.insert("imkink".to_string(), d.imkink);
            map.insert("testtime".to_string(), d.testtime);
            map.insert("tester".to_string(), d.tester);
            map.insert("iop".to_string(), d.iop);
            map.insert("idark".to_string(), d.idark);
            map.insert("result".to_string(), d.result);
            map.insert("i_xtalk".to_string(), d.i_xtalk);
            if !d.mdpid.is_empty() {
                map.insert("mdpid".to_string(), d.mdpid);
            }
            map.insert("yypn".to_string(), d.yypn);
            map
        })
        .collect()
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
) -> Result<Vec<HashMap<String, String>>, MyError> {
    info!(
        "开始SN查询: sns_count={}, pn={}, use_time={}, start='{}', end='{}', result='{}', device='{}', worker='{}'",
        sns.len(), pn, use_time, date_time_start, date_time_end, test_result, test_devices, worker
    );
    let client = client().await?;
    let pool = &client;
    let sn_list = if sns.is_empty() {
        info!("sns 列表为空, 将查询所有 SN");
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
        // ... (所有 match 分支，逻辑不变)
        // 为了简洁，这里省略了所有分支，它们不包含新增的日志代码
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
    let datas = format_data(data);
    info!("SN查询完成，共找到 {} 条记录", datas.len());
    Ok(datas)
}

pub async fn execute_query_sn(
    sql_text_s: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<Data>, MyError> {
    info!("开始执行 sn_query 查询: {}", sql_text_s);
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
            // 这里不过滤，直接插入，因为业务逻辑是查询所有测试记录
            sn_map.insert(sn.clone(), data);
        }
    }

    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();
    info!("共处理 {} 行数据，得到 {} 条SN数据。", row_count, datas.len());
    if datas.is_empty() {
        return Err(MyError::NoResult("箱号".to_string()));
    }
    Ok(datas)
}