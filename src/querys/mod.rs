use async_trait::async_trait;
use bb8_tiberius::ConnectionManager;
use makepad_widgets::Cx;
use std::{collections::HashMap, sync::Arc};
pub mod infos_modal;
pub mod row;
pub mod tabel;
use crate::{
    querys::works::{
        // batch_query::batch_query, box_querys::get_box_datas,
        carton_querys::get_carton_datas,
        export2excel::sn_export,
        // sn_query::sn_query_datas,
    },
    structs::Datas,
    utils::error::MyError,
};
pub mod carton_row;
pub mod carton_tabel;
pub mod querys_view;
pub mod sn_row;
pub mod sn_tabel;
pub mod works;
pub fn live_design(cx: &mut Cx) {
    querys_view::live_design(cx);
    row::live_design(cx);
    tabel::live_design(cx);
    carton_row::live_design(cx);
    carton_tabel::live_design(cx);
    infos_modal::live_design(cx);
    sn_row::live_design(cx);
    sn_tabel::live_design(cx);
}

#[async_trait]
pub trait DatasQuery: Send + Sync {
    // async fn box_query(
    //     &self,
    //     box_no: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     pn: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> anyhow::Result<Vec<HashMap<String, String>>, MyError>;
    async fn get_carton_datas(
        &self,
        carton: String,
        use_time: bool,
        date_time_start: String,
        date_time_end: String,
        pn: String,
        pool: &bb8::Pool<ConnectionManager>,
    ) -> anyhow::Result<Vec<Datas>, MyError>;
    // async fn sn_query_datas(
    //     &self,
    //     sns: Vec<String>,
    //     pn: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     test_result: String,
    //     test_devices: String,
    //     worker: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> Result<Vec<HashMap<String, String>>, MyError>;
    // async fn batch_query(
    //     &self,
    //     q_type: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     pn: String,
    //     test_result: String,
    //     test_devices: String,
    //     worker: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> Result<Vec<HashMap<String, String>>, MyError>;
    async fn data_export(
        &self,
        datas: Vec<HashMap<String, String>>,
    ) -> anyhow::Result<String, MyError>;
}

pub struct DatasQueryer;

#[async_trait]
impl DatasQuery for DatasQueryer {
    // async fn box_query(
    //     &self,
    //     box_no: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     pn: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    //     get_box_datas(box_no, use_time, date_time_start, date_time_end, pn, pool).await
    // }
    async fn get_carton_datas(
        &self,
        carton: String,
        use_time: bool,
        date_time_start: String,
        date_time_end: String,
        pn: String,
        pool: &bb8::Pool<ConnectionManager>,
    ) -> anyhow::Result<Vec<Datas>, MyError> {
        get_carton_datas(carton, use_time, date_time_start, date_time_end, pn, pool).await
    }
    // async fn sn_query_datas(
    //     &self,
    //     sns: Vec<String>,
    //     pn: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     test_result: String,
    //     test_devices: String,
    //     worker: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> Result<Vec<HashMap<String, String>>, MyError> {
    //     sn_query_datas(
    //         sns,
    //         pn,
    //         use_time,
    //         date_time_start,
    //         date_time_end,
    //         test_result,
    //         test_devices,
    //         worker,
    //         pool,
    //     )
    //     .await
    // }
    // async fn batch_query(
    //     &self,
    //     q_type: String,
    //     use_time: bool,
    //     date_time_start: String,
    //     date_time_end: String,
    //     pn: String,
    //     test_result: String,
    //     test_devices: String,
    //     worker: String,
    //     pool: &bb8::Pool<ConnectionManager>,
    // ) -> Result<Vec<HashMap<String, String>>, MyError> {
    //     batch_query(
    //         q_type,
    //         use_time,
    //         date_time_start,
    //         date_time_end,
    //         pn,
    //         test_result,
    //         test_devices,
    //         worker,
    //         pool,
    //     )
    //     .await
    // }
    async fn data_export(
        &self,
        datas: Vec<HashMap<String, String>>,
    ) -> anyhow::Result<String, MyError> {
        sn_export(datas).await
    }
}

pub fn new_query_processor() -> Arc<dyn DatasQuery> {
    Arc::new(DatasQueryer)
}
