use crate::data_import::data_import_db::DbData;
use bb8_tiberius::ConnectionManager;
use chrono::Local;

pub async fn write_data_to_db(
    datas: DbData,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<()> {
    let mut client = pool.get().await?;
    let query = "
        INSERT INTO MAC_10GBOSADATA (
            SN, Tester, ProductBill, TestType, TestTemp, Sen, Icc, Ith, SE, Pf, Im, Result, TestDate, TcMode, TeMode, TestMethod
        ) VALUES (
            @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12, @P13, @P14, @P15, @P16
        )
    ";

    for data in datas.data.iter() {
        let test_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        client
            .execute(
                query,
                &[
                    &data.sn,
                    &"hyd_wx",
                    &"hyd_wx",
                    &datas.pn,
                    &data.condition_unit,
                    &data.sen,
                    &data.icc,
                    &data.ith,
                    &data.se,
                    &data.po,
                    &data.im,
                    &"Ok",
                    &test_date,
                    &"0",
                    &"1",
                    &"point",
                ],
            )
            .await?;
    }

    Ok(())
}
