use anyhow::Result;
use chrono::NaiveDateTime;
use tiberius::Query;

use crate::utils::{error::MyError, sql::client};

#[derive(Debug, Clone, Default)]
pub struct BoxBandData {
    pub box_no: String,
    pub pn: String,
    pub carton_no: String,
    pub new_box_no: String,
    pub create_time: String,
}
pub async fn query_carton_info(carton_no: &str) -> Result<Vec<BoxBandData>, MyError> {
    match check_binded(carton_no).await {
        Ok(_) => get_carton_infos(carton_no).await,
        Err(e) => Err(e),
    }
}

async fn get_carton_infos(carton_no: &str) -> Result<Vec<BoxBandData>, MyError> {
    let client = client().await?;
    let pool = &client;
    
    // 使用参数化查询，防止 SQL 注入
    let sql_text = "SELECT a.Pack_no, a.pn, b.CartonNo, a.CreateTime
        FROM [mes_Factory].[dbo].[MaterialPackSn] a
        INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
        WHERE b.CartonNo = @P1 AND b.PnOptionID = '-100'
        ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC";
    
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    
    let mut query = Query::new(sql_text);
    query.bind(carton_no);
    
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::DbConnectionError(e))?;

    let rows = stream.into_results().await?;
    let mut results = Vec::new();
    let mut boxs = vec![];
    for rowset in rows {
        for row in rowset {
            let box_no = row.get::<&str, _>(0).unwrap_or("").to_string();
            if boxs.contains(&box_no) {
                continue;
            }
            let pn = row.get::<&str, _>(1).unwrap_or("").to_string();
            let carton_no = row.get::<&str, _>(2).unwrap_or("").to_string();
            let create_time = row
                .get::<NaiveDateTime, _>(3)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let data = BoxBandData {
                box_no: box_no.clone(),
                pn,
                carton_no,
                create_time,
                ..Default::default()
            };
            boxs.push(box_no.clone());

            results.push(data);
        }
    }
    if results.is_empty() {
        return Err(MyError::Zdyknown("未查询到相关信息!!!".to_string()));
    }
    results.sort_by(|a, b| a.box_no.cmp(&b.box_no));
    Ok(results)
}
async fn check_binded(carton_no: &str) -> Result<(), MyError> {
    let client = client().await?;
    let pool = &client;
    
    // 使用参数化查询，防止 SQL 注入
    let sql_text = "SELECT TOP 1 carton_No 
        FROM [mes_Factory].[dbo].[jz_box_bind]
        WHERE carton_No = @P1 AND status = '0'";
    
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    
    let mut query = Query::new(sql_text);
    query.bind(carton_no);
    
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::DbConnectionError(e))?;

    let rows = stream.into_row().await?;
    if let Some(_e) = rows {
        return Err(MyError::Zdyknown(format!(
            "箱号: {} 已经绑定过。",
            carton_no
        )));
    } else {
        return Ok(());
    }
}
