use tracing::info;

use crate::{box_band::work::query_work::BoxBandData, utils::{error::MyError, sql::client}};
use anyhow::Result;

pub async fn band_work(box_data: Vec<BoxBandData>) -> Result<(), MyError> {
    for i in box_data.iter() {
        info!("绑定盒号: {}", i.new_box_no);
        // let res = band_box(i).await;
        // match res {
        //     Ok(_) => {
        //         info!("盒号: {} 绑定成功", i.new_box_no);
        //     }
        //     Err(e) => {
        //         info!("盒号: {} 绑定失败, 错误信息: {}", i.new_box_no, e);
        //     }
        // }
    }
    Ok(())
}
pub async fn band_box(box_data: &BoxBandData) -> Result<(), MyError> {
    let client = client().await?;
    let pool = &client;
    let datetime = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let sql_text = format!(
        "exec [mes_Factory].[dbo].[jz_carton_bind] '{}', '{}', '{}', '{}'",
        box_data.box_no, box_data.pn, box_data.carton_no, box_data.new_box_no
    );
    let mut pool = pool.get().await.unwrap();
    let stream = pool.simple_query(sql_text).await?;

    let rows = stream.into_results().await?;
    for rowset in rows {
        for row in rowset {
            let msg = row.get::<&str, _>(0).unwrap_or("").to_string();
            if msg != "OK" {
                return Err(MyError::Zdyknown(msg));
            }
        }
    }
    Ok(())
}   