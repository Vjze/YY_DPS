use bb8_tiberius::ConnectionManager;
use tiberius::{AuthMethod, Config};

use crate::utils::error::MyError;

pub async fn client() -> anyhow::Result<bb8::Pool<ConnectionManager>,MyError> {
    let mut config = Config::new();
    config.host("192.168.3.250");
    config.port(1433);
    config.database("BOSAautotestDB");
    config.authentication(AuthMethod::sql_server("yytest", "yytest"));
    config.trust_cert();
    let manager = ConnectionManager::new(config);
    let pool = bb8::Pool::builder()
        .max_size(10) // 最大连接数，调整根据需要
        .build(manager)
        .await?;
    Ok(pool)
}

pub async fn get_tables(
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<String>, MyError> {
    // let mut client = client().await?;
    let mut client = match pool.get().await {
        Ok(client) => client,
        Err(err) => return Err(MyError::Zdyknown(format!("数据库连接失败！{}", err))),
    };
    let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';";

    let stream = client.query(query, &[]).await?;

    let rowsets = stream.into_results().await?;
    let mut v = vec![];
    for i in 0..rowsets.len() {
        let rows = rowsets.get(i).unwrap();
        for row in rows {
            let r = row.get::<&str, _>(0).unwrap().to_string();
            if &r[0..4] == "MAC_" {
                v.push(r)
            } else {
                continue;
            }
        }
    }

    Ok(v)
}
