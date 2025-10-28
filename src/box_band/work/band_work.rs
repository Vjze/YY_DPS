use crate::{
    box_band::work::query_work::BoxBandData,
    utils::{error::MyError, sql::client},
};
use anyhow::Result;
use sqlx_oldapi::{MssqlPool, query};
use tracing::info;

pub async fn band_work(box_data: Vec<BoxBandData>) -> Result<(), MyError> {
    let mut handles = vec![];
    for i in box_data {
        let data_clone = i; // 假设 BoxBandData 实现了 Clone

        let handle = tokio::spawn(async move {
            info!("Starting DB operation for carton: {}", data_clone.carton_no);
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
    Ok(())
}

pub async fn band_box(box_data: &BoxBandData) -> Result<(), MyError> {
    let pool: &MssqlPool = &client().await?;
    let datetime = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let sql_text = "INSERT INTO [mes_Factory].[dbo].[jz_carton_bind] 
            (carton_No, pkg_No, box_No, [module], p_No, status, bindtime) 
        VALUES (
            @P1, @P2, @P3, @P4, @P5, @P6, @P7
        );";
    let result = query(sql_text)
        .bind(&box_data.carton_no)
        .bind(&box_data.new_box_no)
        .bind(&box_data.box_no)
        .bind("0")
        .bind(&box_data.pn)
        .bind("0")
        .bind(datetime)
        .execute(pool)
        .await
        .map_err(MyError::from)?;

    if result.rows_affected() == 0 {
        info!("INSERT操作未插入任何行，数据: {:?}", box_data);
    }

    Ok(())
}
