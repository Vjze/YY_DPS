use std::{collections::HashMap, sync::Arc};
// pub mod export_row;
// pub mod export_tabel;
use makepad_widgets::ScriptVm;
pub mod export_view;
pub mod works;
use async_trait::async_trait;
use bb8_tiberius::ConnectionManager;

use crate::{
    export::works::{carton_query::do_carton_query, export2excel::write_to_excel},
    structs::Datas,
    utils::error::MyError,
};
pub fn script_mod(vm: &mut ScriptVm) {
    export_view::script_mod(vm);
    // export_tabel::script_mod(vm);
    // export_row::script_mod(vm);
}

#[async_trait]
pub trait Exportable: Send + Sync {
    async fn carton_query(
        &self,
        carton: String,
        typeinfos: String,
        is_multi: bool,
        sql_client: &bb8::Pool<ConnectionManager>,
    ) -> anyhow::Result<(), MyError>;
    async fn export(
        &self,
        type_name: &str,
        datas: &Vec<Datas>,
        lock: bool,
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
        sql_client: &bb8::Pool<ConnectionManager>,
    ) -> anyhow::Result<(), MyError> {
        do_carton_query(carton, typeinfos, is_multi, sql_client).await
    }
    async fn export(
        &self,
        type_name: &str,
        datas: &Vec<Datas>,
        lock: bool,
    ) -> anyhow::Result<(), MyError> {
        write_to_excel(type_name, datas, lock).await
        // Ok(())
    }
}

pub fn new_export_processor() -> Arc<dyn Exportable> {
    Arc::new(Exporter)
}
