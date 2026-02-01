use crate::utils::error::MyError;
use bb8_tiberius::ConnectionManager;
pub async fn unbind_box(
    box_no: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<(), MyError> {
    let mut client = pool.get().await.unwrap();
    let stream = client
        .execute(
            format!(
                "UPDATE [mes_Factory].[dbo].[jz_box_bind] SET status = '1' WHERE pkg_No = '{0}' or box_No = '{0}",
                box_no
            ),
            &[&1i32],
        )
        .await;
    match stream {
        Ok(_) => Ok(()),
        Err(e) => Err(MyError::UnbindBoxErr(e.to_string())),
    }
}
pub async fn unbind_carton(
    carton_no: &str,
    pool: &bb8::Pool<ConnectionManager>,
) -> anyhow::Result<(), MyError> {
    let mut client = pool.get().await.unwrap();

    let stream = client
        .execute(
            format!(
                "UPDATE [mes_Factory].[dbo].[jz_box_bind] SET status = '1' WHERE carton_No = '{0}'",
                carton_no
            ),
            &[&1i32],
        )
        .await;
    match stream {
        Ok(_) => Ok(()),
        Err(e) => Err(MyError::UnbindBoxErr(e.to_string())),
    }
}
