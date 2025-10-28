

use sqlx_oldapi::{MssqlPool, Row};

use crate::utils::error::MyError;


pub async fn client() -> anyhow::Result<MssqlPool,MyError> {
    let database_url = "mssql://yytest:yytest@localhost:1433/BOSAautotestDB";
    let pool = MssqlPool::connect(database_url).await?;
    Ok(pool)
}

pub async fn get_tables(
    pool: &MssqlPool,
) -> Result<Vec<String>, MyError> {
    let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';";
    let rows = sqlx_oldapi::query(query)
        .fetch_all(pool)
        .await?;

    let mut v = Vec::new();
    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?; 
        if table_name.starts_with("MAC_") {
            v.push(table_name);
        }
    }
    Ok(v)
}
