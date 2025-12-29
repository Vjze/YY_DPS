use tracing::info;

use crate::{
    box_band::work::query_work::BoxBandData,
    utils::{error::MyError, sql::client},
};
use anyhow::Result;

pub async fn band_work(box_data: Vec<BoxBandData>) -> Result<(), MyError> {
    let mut handles = vec![];
    for i in box_data {
        let data_clone = i; // 假设 BoxBandData 实现了 Clone

        let handle = tokio::spawn(async move {
            info!("Starting DB operation for carton: {}", data_clone.carton_no);

            // 调用 band_box 并处理其可能返回的 MyError
            band_box(&data_clone).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let inner_result: Result<(), MyError> = handle
            .await
            // 使用 map_err 将 JoinError 转换为 MyError
            .map_err(|e| MyError::TaskJoinError(e.to_string()))?;

        // 2. 处理内层的 MyError（这是由 band_box 数据库操作失败导致的）
        inner_result?;
    }

    // 如果所有任务都成功，则返回 Ok(())
    Ok(())
}
pub async fn band_box(box_data: &BoxBandData) -> Result<(), MyError> {
    let client = client().await?;
    let pool = &client;
    let datetime = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let sql_text = format!(
        "INSERT INTO [mes_Factory].[dbo].[jz_box_bind] (carton_No, pkg_No, box_No, [module], p_No, status, createtime) VALUES (
    @p1, @p2, @p3, @p4, @p5, @p6, @p7
);"
    );
    let mut pool = pool.get().await.unwrap();
    pool.execute(
        sql_text,
        &[
            &box_data.carton_no,
            &box_data.new_box_no,
            &box_data.box_no,
            &"0",
            &box_data.pn,
            &"0",
            &datetime,
        ],
    )
    .await?;
    Ok(())
}
