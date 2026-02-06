use bb8_tiberius::ConnectionManager;
use tiberius::{AuthMethod, Config};
use std::env;

use crate::utils::error::MyError;
use crate::utils::retry::retry_default;

fn _local_ip() -> Config {
    let mut config = Config::new();
    config.host("127.0.0.1");
    config.port(1433);
    config.database("BOSAautotestDB");

    // 优先使用环境变量，回退到默认值（临时方案）
    let username = env::var("DB_USER_LOCAL").unwrap_or_else(|_| "sa".to_string());
    let password = env::var("DB_PASSWORD_LOCAL").unwrap_or_else(|_| "Wjz142857.".to_string());

    config.authentication(AuthMethod::sql_server(&username, &password));
    config.trust_cert();
    config
}

fn _server_ip() -> Config {
    let mut config = Config::new();
    config.host("192.168.3.250");
    config.port(1433);
    config.database("BOSAautotestDB");

    // 优先使用环境变量，回退到默认值（临时方案）
    let username = env::var("DB_USER_SERVER").unwrap_or_else(|_| "yytest".to_string());
    let password = env::var("DB_PASSWORD_SERVER").unwrap_or_else(|_| "yytest".to_string());

    config.authentication(AuthMethod::sql_server(&username, &password));
    config.trust_cert();
    config
}

fn get_pool_size() -> u32 {
    env::var("DB_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5) // 从25降到5，更适合单用户桌面应用
}

pub async fn client() -> anyhow::Result<bb8::Pool<ConnectionManager>, MyError> {
    let use_server = env::var("USE_SERVER_DB")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    let config = if use_server {
        _server_ip()
    } else {
        _local_ip()
    };
    let manager = ConnectionManager::new(config);

    let pool_size = get_pool_size();

    let pool = bb8::Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .await
        .map_err(|e| MyError::SqlError(e.into()))?;

    Ok(pool)
}

pub async fn get_tables(
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<Vec<String>, MyError> {
    let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE';";
    
    // 使用重试机制执行查询
    let result = retry_default(|| async {
        let mut client = pool.get().await
            .map_err(|e| MyError::Zdyknown(format!("数据库连接池错误: {}", e)))?;
            
        let stream = client.query(query, &[]).await
            .map_err(|e| MyError::DbConnectionError(e))?;
            
        let rowsets = stream.into_results().await
            .map_err(|e| MyError::DbConnectionError(e))?;
            
        let mut v = vec![];
        for i in 0..rowsets.len() {
            let rows = rowsets.get(i).unwrap();
            for row in rows {
                let r = row.get::<&str, _>(0).unwrap().to_string();
                if &r[0..4] == "MAC_" {
                    v.push(r)
                }
            }
        }
        
        Ok::<Vec<String>, MyError>(v)
    }).await?;
    
    Ok(result)
}
