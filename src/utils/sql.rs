// sql.rs
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt as _};

use crate::utils::error::MyError; // 确保路径正确

async fn connect_to_db(config: Config) -> anyhow::Result<Client<Compat<TcpStream>>, MyError> {
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}

pub async fn client() -> anyhow::Result<Client<Compat<TcpStream>>, MyError> {
    println!("开始连接数据库!");
    let mut config = Config::new();
    config.host("192.168.3.250");
    config.port(1433);
    config.database("BOSAautotestDB");
    config.authentication(AuthMethod::sql_server("yytest", "yytest"));
    config.trust_cert();

    // 尝试连接第一个地址
    if let Ok(client_result) = connect_to_db(config.clone()).await {
        return Ok(client_result);
    }

    // 如果第一个连接失败，尝试第二个地址
    config.port(1433); // 端口不变
    config.host("192.168.10.142");
    connect_to_db(config).await
}

pub async fn get_tables() -> anyhow::Result<Vec<String>, MyError> {
    let mut client = client().await?;
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