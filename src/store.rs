use std::collections::HashMap;

use bb8_tiberius::ConnectionManager;
use makepad_widgets::*;

use crate::{
    configs::{
        decimal_config::{DecimalConfig, get_templates},
        get_all_column_name::load_all_column_names,
        type_config::{Infos, get_type_names},
    },
    utils::sql::{client, get_tables},
};
#[derive(Debug, Default, Clone)]
pub struct Store {
    pub types: Vec<String>,
    pub templates: Vec<String>,
    pub all_column_name: Vec<String>,
    pub sql_tables: Vec<String>,
    pub datas: Option<Vec<HashMap<String, String>>>,
    pub type_infos: (Vec<String>, Infos),
    pub template_infos: DecimalConfig,
    pub map_infos: HashMap<String, String>,
    pub sql_pool: Option<bb8::Pool<ConnectionManager>>,
    pub grid_area: Area, // 存储 DecimalGrid 的 Area
    pub logined: bool,
    pub free_login: bool
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
        // let sql_pool = match client().await {
        //     Ok(res) => Some(res),
        //     Err(e) => {
        //         Cx::post_action(e);
        //         None
        //     }
        // };
        // let pool = sql_pool.clone().unwrap();
        // let sql_tables = match get_tables(&pool).await{
        //     Ok(res) => {
        //         res
        //     },
        //     Err(e) => {
        //         Cx::post_action(e);
        //         Vec::default()
        //     },
        // };
        Self {
            types,
            templates,
            // sql_pool,
            // sql_tables,
            all_column_name,
            ..Default::default()
        }
    }
}
