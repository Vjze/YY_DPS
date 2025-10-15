use async_trait::async_trait;
use makepad_widgets::Cx;
use std::{path::PathBuf, sync::Arc};

use crate::{
    data_import::{
        data_import_db::DbData,
        work::{
            extract_data::{extract_data, ImportDBDatas}, select_file::select_file, write_data::write_data_to_db,
        },
    },
};
pub mod data_import_db;
pub mod import_row;
pub mod import_table;
pub mod work;
pub fn live_design(cx: &mut Cx) {
    data_import_db::live_design(cx);
    import_table::live_design(cx);
    import_row::live_design(cx);
}

#[async_trait]
pub trait DataImport: Send + Sync {
    async fn select_file(&self) -> anyhow::Result<PathBuf>;
    async fn extract(&self, path: &str) -> anyhow::Result<Vec<ImportDBDatas>>;
    async fn write(&self, datas: DbData) -> anyhow::Result<()>;
}

pub struct DataImporter;

#[async_trait]
impl DataImport for DataImporter {
    async fn select_file(&self) -> anyhow::Result<PathBuf> {
        select_file().await
    }
    async fn extract(&self, path: &str) -> anyhow::Result<Vec<ImportDBDatas>> {
        extract_data(path).await
    }
    async fn write(&self, datas: DbData) -> anyhow::Result<()> {
        write_data_to_db(datas).await
    }
}

pub fn new_import_processor() -> Arc<dyn DataImport> {
    Arc::new(DataImporter)
}
