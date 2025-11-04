use makepad_widgets::*;
use std::collections::HashMap;

use crate::{
    box_band::work::query_work::BoxBandData,
    configs::{
        decimal_config::{DecimalConfig, get_templates},
        get_all_column_name::load_all_column_names,
        type_config::{Infos, get_type_names},
    },
    data_import::data_import_db::DbData,
};

#[derive(Debug, Default, Clone)]
pub struct DatasStore {
    pub export_datas: Vec<HashMap<String, String>>,
    pub query_datas: Vec<HashMap<String, String>>,
}

#[derive(Debug, Default, Clone)]
pub struct SettingStore {
    pub types: Vec<String>,
    pub templates: Vec<String>,
    pub all_column_name: Vec<String>,
    pub type_infos: (Vec<String>, Infos),
    pub template_infos: DecimalConfig,
    pub map_infos: HashMap<String, String>,
}
#[derive(Debug, Default, Clone)]
pub struct LoginStore {
    pub logined: bool,
    pub free_login: bool,
}
#[derive(Debug, Default, Clone)]
pub struct ImportStore {
    pub import_datas: DbData,
}
#[derive(Debug, Default, Clone)]
pub struct BoxBandStore {
    pub box_data: Vec<BoxBandData>,
}
#[derive(Debug, Default, Clone)]
pub struct Store {
    pub datas_store: DatasStore,
    pub setting_store: SettingStore,
    pub login_store: LoginStore,
    pub import_store: ImportStore,
    pub box_band_store: BoxBandStore,
}

impl Store {
    pub async fn init() -> Self {
        let types = match get_type_names().await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::default()
            }
        };
        let templates = match get_templates().await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::default()
            }
        };
        let all_column_name = match load_all_column_names().await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::default()
            }
        };
        let setting_store = SettingStore {
            types: types.clone(),
            templates: templates.clone(),
            all_column_name: all_column_name.clone(),
            ..Default::default()
        };
        Self {
            setting_store,
            ..Default::default()
        }
    }
}