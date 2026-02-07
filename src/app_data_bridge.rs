use crate::data_import::work::extract_data::ImportDBDatas;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

// Simple data bridge to share query results with UI without touching AppStore directly
pub type SharedQueryData = Arc<Mutex<Vec<HashMap<String, String>>>>;

static QUERY_DATA_BRIDGE: OnceLock<SharedQueryData> = OnceLock::new();
static IMPORT_DATA_BRIDGE: OnceLock<SharedImportData> = OnceLock::new();

pub fn init_bridge() -> SharedQueryData {
    QUERY_DATA_BRIDGE
        .get_or_init(|| Arc::new(Mutex::new(Vec::new())))
        .clone()
}

pub fn set_query_datas(data: Vec<HashMap<String, String>>) {
    let bridge = init_bridge();
    let mut guard = bridge.lock().unwrap();
    *guard = data;
}

pub fn get_query_datas() -> SharedQueryData {
    init_bridge()
}

pub fn get_query_datas_vec() -> Vec<HashMap<String, String>> {
    let bridge = init_bridge();
    bridge.lock().unwrap().clone()
}

pub fn query_data_len() -> usize {
    let bridge = init_bridge();
    bridge.lock().unwrap().len()
}

pub type SharedImportData = Arc<Mutex<Vec<ImportDBDatas>>>;

pub fn init_import_bridge() -> SharedImportData {
    IMPORT_DATA_BRIDGE
        .get_or_init(|| Arc::new(Mutex::new(Vec::new())))
        .clone()
}

pub fn set_import_datas(datas: Vec<ImportDBDatas>) {
    let bridge = init_import_bridge();
    let mut guard = bridge.lock().unwrap();
    *guard = datas;
}

pub fn get_import_datas() -> Vec<ImportDBDatas> {
    let bridge = init_import_bridge();
    bridge.lock().unwrap().clone()
}
