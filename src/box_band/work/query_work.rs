use crate::utils::{error::MyError, sql::client};
use futures::TryStreamExt;
use sqlx_oldapi::mssql::MssqlRow;
use sqlx_oldapi::types::chrono::NaiveDateTime;
use sqlx_oldapi::{Error as SqlxError, MssqlPool, Row};
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct BoxBandData {
    pub box_no: String,
    pub pn: String,
    pub carton_no: String,
    pub new_box_no: String,
    pub create_time: String,
}
impl sqlx_oldapi::FromRow<'_, MssqlRow> for BoxBandData {
    fn from_row(row: &MssqlRow) -> Result<Self, SqlxError> {
        let create_time_dt: NaiveDateTime = row.try_get("create_time")?;
        let create_time = create_time_dt.format("%Y-%m-%d %H:%M:%S").to_string();

        Ok(BoxBandData {
            box_no: row.try_get("box_no")?,
            pn: row.try_get("pn")?,
            carton_no: row.try_get("carton_no")?,
            new_box_no: String::default(), // 字段在 SQL 中不存在，使用默认值
            create_time,
        })
    }
}
pub async fn query_carton_info(carton_no: String) -> Result<Vec<BoxBandData>, MyError> {
    match check_binded(carton_no.clone()).await {
        Ok(_) => get_carton_infos(carton_no).await,
        Err(e) => Err(e),
    }
}

async fn get_carton_infos(carton_no: String) -> Result<Vec<BoxBandData>, MyError> {
    let pool: &MssqlPool = &client().await?;
    let sql_text =
        "SELECT a.Pack_no AS box_no, a.pn AS pn, b.CartonNo AS carton_no, a.CreateTime AS create_time
            FROM [mes_Factory].[dbo].[MaterialPackSn] a
            INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
            WHERE b.CartonNo = @P1 AND b.PnOptionID = '-100'
            ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC";
    let mut rows = sqlx_oldapi::query_as::<sqlx_oldapi::Mssql, BoxBandData>(&sql_text)
        .bind(carton_no)
        .fetch(pool);

    let mut results = Vec::new();
    let mut boxs: HashSet<String> = HashSet::new();

    // 3. 循环处理数据流
    while let Some(data) = rows.try_next().await.map_err(MyError::from)? {
        if boxs.insert(data.box_no.clone()) {
            results.push(data);
        }
    }

    if results.is_empty() {
        return Err(MyError::Zdyknown("未查询到相关信息!!!".to_string()));
    }

    Ok(results)
}
async fn check_binded(carton_no: String) -> Result<(), MyError> {
    let pool: &MssqlPool = &client().await?;
    let sql_text = format!(
        "SELECT TOP 1 carton_No
            FROM [mes_Factory].[dbo].[jz_carton_bind]
            WHERE carton_No = '{}' AND status = '0'",
        carton_no
    );

    let row = sqlx_oldapi::query(&sql_text)
        .fetch_optional(pool)
        .await
        .map_err(MyError::from)?;
    if row.is_some() {
        return Err(MyError::Zdyknown(format!(
            "箱号: {} 已经绑定过。",
            carton_no
        )));
    } else {
        return Ok(());
    }
}
