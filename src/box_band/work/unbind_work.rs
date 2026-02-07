use crate::utils::error::MyError;
use bb8_tiberius::ConnectionManager;
use tiberius::Query;

pub async fn unbind_box(
    box_no: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<(), MyError> {
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    
    // 使用参数化查询，防止 SQL 注入
    let sql = "UPDATE [mes_Factory].[dbo].[jz_box_bind] 
               SET status = '1' 
               WHERE pkg_No = @P1 OR box_No = @P1";
    
    let mut query = Query::new(sql);
    query.bind(box_no);
    
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::DbConnectionError(e))?;
    
    // 消费结果流
    let _ = stream.into_results().await
        .map_err(|e| MyError::DbConnectionError(e))?;
    
    Ok(())
}

pub async fn unbind_carton(
    carton_no: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<(), MyError> {
    let mut client = pool.get().await
        .map_err(|_| MyError::DatabaseNotConnected)?;
    
    // 使用参数化查询，防止 SQL 注入
    let sql = "UPDATE [mes_Factory].[dbo].[jz_box_bind] 
               SET status = '1' 
               WHERE carton_No = @P1";
    
    let mut query = Query::new(sql);
    query.bind(carton_no);
    
    let stream = query.query(&mut client).await
        .map_err(|e| MyError::DbConnectionError(e))?;
    
    // 消费结果流
    let _ = stream.into_results().await
        .map_err(|e| MyError::DbConnectionError(e))?;
    
    Ok(())
}
