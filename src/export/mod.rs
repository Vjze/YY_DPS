use std::{collections::HashMap, sync::Arc};
pub mod export_row;
pub mod export_tabel;
use makepad_widgets::Cx;
pub mod export_view;
pub mod works;
use async_trait::async_trait;

use crate::{
    export::works::{carton_query::do_carton_query, export2excel::write_to_excel},
    utils::error::MyError,
};
pub fn live_design(cx: &mut Cx) {
    export_view::live_design(cx);
    export_tabel::live_design(cx);
    export_row::live_design(cx);
}

#[async_trait]
pub trait Exportable: Send + Sync {
    async fn carton_query(
        &self,
        carton: String,
        typeinfos: String,
        is_multi: bool,
    ) -> anyhow::Result<Vec<HashMap<String, String>>, MyError>;
    async fn export(
        &self,
        type_name: String,
        datas: Vec<HashMap<String, String>>,
    ) -> anyhow::Result<(), MyError>;
}

pub struct Exporter;

#[async_trait]
impl Exportable for Exporter {
    async fn carton_query(
        &self,
        carton: String,
        typeinfos: String,
        is_multi: bool,
    ) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
        do_carton_query(carton, typeinfos, is_multi).await
    }
    async fn export(
        &self,
        type_name: String,
        datas: Vec<HashMap<String, String>>,
    ) -> anyhow::Result<(), MyError> {
        write_to_excel(type_name, datas).await
    }
}

pub fn new_export_processor() -> Arc<dyn Exportable> {
    Arc::new(Exporter)
}