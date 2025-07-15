use std::collections::HashMap;

use makepad_widgets::*;

use crate::{configs::type_config::get_type_names, utils::sql::get_tables};
#[derive(Debug, Default, Clone)]
pub struct Store {
    pub types: Vec<String>,
    pub sql_tables: Vec<String>,
    pub datas: Vec<HashMap<String,String>>,
    pub type_infos: Vec<String>
}   

impl Store {
    pub async fn init() -> Self {
        let types = match get_type_names().await {
            Ok(res) => {
                res
            },
            Err(e) => {
                Cx::post_action(e);
                Vec::default()
            },
        };
        // let sql_tables = match get_tables().await{
        //     Ok(res) => {
        //         res
        //     },
        //     Err(e) => {
        //         Cx::post_action(e);
        //         Vec::default()
        //     },
        // };
        Self{
            types,
            // sql_tables,
            ..Default::default()
        }
    }
    pub fn set_type_infos(&mut self, mut infos: Vec<String>) {
        infos.sort_by(|a,b| a.cmp(b));
        self.type_infos = infos
    }
}