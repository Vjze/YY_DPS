use crate::{
    box_band::work::query_work::BoxBandData,
    configs::{
        decimal_config::{TemplateConfig, get_templates},
        get_all_column_name::load_all_column_names,
        type_config::{Infos, get_type_names},
    },
    data_import::data_import_db::DbData,
    utils::{retry::retry_default, sql::client},
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};
use bb8_tiberius::ConnectionManager;
use makepad_widgets::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct DatasStore {
    pub export_datas: Arc<Vec<HashMap<String, String>>>,
    pub query_datas: Arc<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Default)]
pub struct SettingStore {
    pub types: Arc<Vec<String>>,
    pub templates: Arc<Vec<String>>,
    pub all_column_name: Arc<Vec<String>>,
    pub type_infos: (Arc<Vec<String>>, Infos),
    pub template_infos: TemplateConfig,
    pub map_infos: Arc<HashMap<String, String>>,
}
#[derive(Debug, Default, Clone)]
pub struct LoginStore {
    pub logined: bool,
    pub free_login: bool,
}
#[derive(Debug, Default)]
pub struct ImportStore {
    pub import_datas: Arc<DbData>,
}
#[derive(Debug, Default)]
pub struct BoxBandStore {
    pub box_data: Arc<Vec<BoxBandData>>,
}
#[derive(Debug, Default)]
pub struct Store {
    pub datas_store: DatasStore,
    pub setting_store: SettingStore,
    pub login_store: LoginStore,
    pub import_store: ImportStore,
    pub box_band_store: BoxBandStore,
    pub pool: Option<bb8::Pool<ConnectionManager>>,
}

impl DatasStore {
    pub fn new() -> Self {
        Self {
            export_datas: Arc::new(Vec::new()),
            query_datas: Arc::new(Vec::new()),
        }
    }

    pub fn set_export_datas(&mut self, data: Vec<HashMap<String, String>>) {
        self.export_datas = Arc::new(data);
    }

    pub fn set_query_datas(&mut self, data: Vec<HashMap<String, String>>) {
        self.query_datas = Arc::new(data);
    }

    pub fn clear_export_datas(&mut self) {
        self.export_datas = Arc::new(Vec::new());
    }

    pub fn clear_query_datas(&mut self) {
        self.query_datas = Arc::new(Vec::new());
    }
}

impl SettingStore {
    pub fn new() -> Self {
        Self {
            types: Arc::new(Vec::new()),
            templates: Arc::new(Vec::new()),
            all_column_name: Arc::new(Vec::new()),
            type_infos: (Arc::new(Vec::new()), Infos::default()),
            template_infos: TemplateConfig::default(),
            map_infos: Arc::new(HashMap::new()),
        }
    }

    pub fn set_types(&mut self, types: Vec<String>) {
        self.types = Arc::new(types);
    }

    pub fn set_templates(&mut self, templates: Vec<String>) {
        self.templates = Arc::new(templates);
    }

    pub fn set_all_column_name(&mut self, names: Vec<String>) {
        self.all_column_name = Arc::new(names);
    }

    pub fn set_type_infos(&mut self, templates: Vec<String>, infos: Infos) {
        self.type_infos = (Arc::new(templates), infos);
    }

    pub fn add_template_to_type(&mut self, template_name: String) {
        let mut templates = self.type_infos.0.as_ref().clone();
        templates.push(template_name);
        self.type_infos.0 = Arc::new(templates);
    }

    pub fn remove_template_from_type(&mut self, index: usize) {
        let mut templates = self.type_infos.0.as_ref().clone();
        if index < templates.len() {
            templates.remove(index);
            self.type_infos.0 = Arc::new(templates);
        }
    }
}

impl ImportStore {
    pub fn new() -> Self {
        Self {
            import_datas: Arc::new(DbData::default()),
        }
    }

    pub fn set_import_datas(&mut self, data: DbData) {
        self.import_datas = Arc::new(data);
    }

    pub fn clear(&mut self) {
        self.import_datas = Arc::new(DbData::default());
    }
}

impl BoxBandStore {
    pub fn new() -> Self {
        Self {
            box_data: Arc::new(Vec::new()),
        }
    }

    pub fn set_box_data(&mut self, data: Vec<BoxBandData>) {
        self.box_data = Arc::new(data);
    }

    pub fn clear(&mut self) {
        self.box_data = Arc::new(Vec::new());
    }
}

impl Store {
    pub async fn init() -> Self {
        let types = match retry_default(|| get_type_names()).await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::new()
            }
        };
        let templates = match retry_default(|| get_templates()).await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::new()
            }
        };
        let all_column_name = match retry_default(|| load_all_column_names()).await {
            Ok(res) => res,
            Err(e) => {
                Cx::post_action(e);
                Vec::new()
            }
        };

        let mut setting_store = SettingStore::new();
        setting_store.set_types(types);
        setting_store.set_templates(templates);
        setting_store.set_all_column_name(all_column_name);

        let pool = match client().await {
            Ok(client) => {
                enqueue_popup_notification(PopupItem {
                    kind: PopupKind::Success,
                    auto_dismissal_duration: Some(5.0),
                    message: "数据库连接成功".to_string(),
                });
                Some(client)
            }
            Err(e) => {
                Cx::post_action(e);
                None
            }
        };
        let mut store = Store::default();
        store.setting_store = setting_store;
        store.pool = pool;
        store
    }

    pub fn update_types(&mut self, types: Vec<String>) {
        self.setting_store.set_types(types);
    }

    pub fn update_templates(&mut self, templates: Vec<String>) {
        self.setting_store.set_templates(templates);
    }

    pub fn update_column_names(&mut self, names: Vec<String>) {
        self.setting_store.set_all_column_name(names);
    }
}
