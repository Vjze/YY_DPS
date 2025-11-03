use sqlx_oldapi::types::chrono::Local;
use sqlx_oldapi::{MssqlPool, Transaction, query};
use tracing::{error, info};

use crate::{
    data_import::data_import_db::DbData,
    utils::{error::MyError, sql::client},
};

pub async fn write_data_to_db(datas: DbData) -> anyhow::Result<()> {
    // 假设 client().await? 返回 sqlx_oldapi::MssqlPool
    let pool: &MssqlPool = &client().await?;
    let mut tx: Transaction<'_, sqlx_oldapi::Mssql> = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to begin transaction: {}", e);
            return Err(e.into());
        }
    };

    let query_text = "
        INSERT INTO MAC_10GBOSADATA (
            SN, Tester, ProductBill, TestType, TestTemp, Sen, Icc, Ith, SE, Pf, Im, Result, TestDate, TcMode, TeMode, TestMethod
        ) VALUES (
            @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12, @P13, @P14, @P15, @P16
        )
    ";

    let mut rows_inserted = 0;

    // 2. 循环插入数据，在事务上执行查询
    for data in datas.data.iter() {
        let test_date = Local::now().naive_local();

        let result = query(query_text)
            .bind(&data.sn)
            .bind("hyd_wx")
            .bind("hyd_wx")
            .bind(&datas.pn)
            .bind(&data.condition_unit)
            .bind(&data.sen)
            .bind(&data.icc)
            .bind(&data.ith)
            .bind(&data.se)
            .bind(&data.po)
            .bind(&data.im)
            .bind("Ok")
            .bind(test_date)
            .bind("0")
            .bind("1")
            .bind("point")
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Transaction insert failed for SN {}: {}", data.sn, e);
                MyError::from(e)
            })?;

        rows_inserted += result.rows_affected();
    }

    match tx.commit().await {
        Ok(_) => {
            info!(
                "Successfully committed transaction. Total rows inserted: {}",
                rows_inserted
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to commit transaction: {}", e);
            Err(e.into())
        }
    }
}
