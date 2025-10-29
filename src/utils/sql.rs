use crate::utils::error::MyError;
use sqlx_oldapi::mssql::MssqlPoolOptions;
use sqlx_oldapi::{MssqlPool, Row};

pub async fn client() -> anyhow::Result<MssqlPool, MyError> {
    // let database_url = "mssql://yytest:yytest@localhost:1433/BOSAautotestDB";
    let database_url = "mssql://sa:Wjz142857.@localhost:1433/BOSAautotestDB";
    let pool = MssqlPoolOptions::new()
        // 设置最大连接数：推荐 10-25
        .max_connections(100)
        // 设置最小连接数：推荐 2-5
        .min_connections(10)
        // 连接池健康检查间隔
        .idle_timeout(std::time::Duration::from_secs(3600))
        // 连接超时时间
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn get_tables(pool: &MssqlPool) -> Result<Vec<String>, MyError> {
    let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';";
    let rows = sqlx_oldapi::query(query).fetch_all(pool).await?;

    let mut v = Vec::new();
    for row in rows {
        let table_name: String = row.try_get("TABLE_NAME")?;
        if table_name.starts_with("MAC_") {
            v.push(table_name);
        }
    }
    Ok(v)
}
