use anyhow::Result;
use chrono::NaiveDateTime;

use crate::utils::{error::MyError, sql::client};

#[derive(Debug, Clone, Default)]
pub struct BoxBandData {
    pub box_no: String,
    pub pn: String,
    pub carton_no: String,
    pub new_box_no: String,
    pub create_time: String,
}
pub async fn query_carton_info(carton_no: String) -> Result<Vec<BoxBandData>, MyError> {
    let client = client().await?;
    let pool = &client;
    let sql_text = format!(
        "select a.Pack_no, a.pn, b.CartonNo, a.CreateTime
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no = b.Packing_no
            where b.CartonNo = '{}' and b.PnOptionID = '-100'
            order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc",
        carton_no
    );
    let mut pool = pool.get().await.unwrap();
    let stream = pool.simple_query(sql_text).await?;

    let rows = stream.into_results().await?;
    let mut results = Vec::new();
    for rowset in rows {
        for row in rowset {
            let box_no = row.get::<&str, _>(0).unwrap_or("").to_string();
            let pn = row.get::<&str, _>(1).unwrap_or("").to_string();
            let carton_no = row.get::<&str, _>(2).unwrap_or("").to_string();
            let create_time = row
                .get::<NaiveDateTime, _>(3)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let data = BoxBandData {
                box_no,
                pn,
                carton_no,
                create_time,
                ..Default::default()
            };
            results.push(data);
        }
    }
    if results.is_empty() {
        return Err(MyError::Zdyknown("未查询到相关信息!!!".to_string()));
    }
    Ok(results)
}

