use bb8_tiberius::ConnectionManager;

use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::utils::error::MyError;

// async fn connect_to_db(config: Config) -> anyhow::Result<Client<Compat<TcpStream>>, String> {
//     let tcp = TcpStream::connect(config.get_addr()).await
//        .map_err(|e| format!("数据库连接失败:{}",e.to_string()))?;
//     tcp.set_nodelay(true).map_err(|e| format!("数据库连接失败:{}",e.to_string()))?;
//     let client = tiberius::Client::connect(config, tcp.compat_write()).await
//        .map_err(|e| format!("数据库连接失败:{}",e.to_string()))?;
//     Ok(client)
// }

pub async fn client() -> anyhow::Result<bb8::Pool<ConnectionManager>> {
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
    // if let Ok(client) = connect_to_db(config.clone()).await {
    //     let manager = ConnectionManager::new(config);
    //     let pool = bb8::Pool::builder()
    //         .max_size(10) // 最大连接数，调整根据需要
    //         .build(manager)
    //         .await.unwrap();
    //     return Ok(client);
    // }
    
    // config.port(1433);
    // config.host("192.168.10.142");
    // connect_to_db(config).await
}

pub async fn get_tables(pool: &bb8::Pool<ConnectionManager>) -> anyhow::Result<Vec<String>, MyError> {
    // let mut client = client().await?;
    let mut client = pool.get().await.unwrap();
    let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';";

    let stream = client.query(query, &[]).await.unwrap();

    let rowsets = stream.into_results().await.unwrap();
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