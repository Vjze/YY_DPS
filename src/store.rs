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
pub struct Store {
    pub types: Vec<String>,
    pub templates: Vec<String>,
    pub all_column_name: Vec<String>,
    pub datas: Option<Vec<HashMap<String, String>>>,
    pub type_infos: (Vec<String>, Infos),
    pub template_infos: DecimalConfig,
    pub map_infos: HashMap<String, String>,
    pub grid_area: Area, // 存储 DecimalGrid 的 Area
    pub logined: bool,
    pub free_login: bool,
    pub box_data: Vec<BoxBandData>,
    pub import_datas: DbData,
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

        Self {
            types,
            templates,
            all_column_name,
            ..Default::default()
        }
    }
}
