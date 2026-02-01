use crate::querys::get_box_datas;
use crate::querys::get_carton_datas;
use crate::querys::sn_query_datas;
use crate::{export::works::carton_query::get_info, utils::error::MyError};
use bb8_tiberius::ConnectionManager;
use futures::StreamExt;
use futures::stream::iter;
use std::collections::HashMap;
use tracing::info;
pub async fn batch_query(
    q_type: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
    test_result: String,
    test_devices: String,
    worker: String,
    pool: &bb8::Pool<ConnectionManager>,
) -> Result<Vec<HashMap<String, String>>, MyError> {
    let sns = get_info().await?;
    if sns.is_empty() {
        return Err(MyError::Zdyknown(format!(
            "文件内未找到条码数据，请确认文件是否正确."
        )));
    }
    let mut all_datas = Vec::new();
    let snss = sns.clone();
    let mut tasks = iter(sns)
        .map(|sn| {
            let pool = pool.clone();
            let sns = snss.clone();
            let date_time_start = date_time_start.clone();
            let date_time_end = date_time_end.clone();
            let pn = pn.clone();
            let worker = worker.clone();
            let test_result = test_result.clone();
            let test_devices = test_devices.clone();
            // 为每个查询创建一个异步任务
            let res = if q_type == "箱号" {
                tokio::spawn(async move {
                    get_carton_datas(
                        sn.clone(),
                        use_time,
                        date_time_start.clone(),
                        date_time_end.clone(),
                        pn.clone(),
                        &pool,
                    )
                    .await
                })
            } else if q_type == "盒号" {
                tokio::spawn(async move {
                    get_box_datas(
                        sn.clone(),
                        use_time,
                        date_time_start.clone(),
                        date_time_end.clone(),
                        pn.clone(),
                        &pool,
                    )
                    .await
                })
            } else {
                tokio::spawn(async move {
                    sn_query_datas(
                        sns,
                        pn.clone(),
                        use_time,
                        date_time_start.clone(),
                        date_time_end.clone(),
                        test_result.clone(),
                        test_devices.clone(),
                        worker.clone(),
                        &pool,
                    )
                    .await
                })
            };
            res
        })
        .buffer_unordered(10); // 限制并发数为 10

    while let Some(result) = tasks.next().await {
        match result {
            Ok(Ok(res)) => all_datas.extend(res), // 成功, 扩展结果
            Ok(Err(e)) => {
                info!("批量查询中有一个任务失败: {:?}", e);
                // 可以选择继续或在这里返回错误
            }
            Err(e) => {
                info!("批量查询任务执行失败: {:?}", e);
                // Tokio task join error
            }
        }
    }
    Ok(all_datas)
}
