use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
// Removed background thread, using in-process handlers instead

// Re-export often-used action types for bus routing
use crate::app_data_bridge::set_query_datas;
use crate::data_import::data_import_db::DbData;
use crate::structs::Datas;
// no direct dependency on MyError here yet

// Bus events carried by the AppBus. These are lightweight wrappers around UI Actions.
#[derive(Clone, Debug)]
pub enum BusEvent {
    QueryResult(Vec<HashMap<String, String>>),
    ExportResult(Vec<Datas>),
    DataExtractedAction(DbData),
}

pub struct AppBus {
    handlers: Arc<Mutex<Vec<Box<dyn Fn(BusEvent) + Send + Sync>>>>,
}

static APP_BUS: OnceLock<AppBus> = OnceLock::new();

impl AppBus {
    pub fn new() -> Self {
        // 简化实现：通过事件处理回调进行分发，不再需要后台线程
        AppBus {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn init() -> &'static AppBus {
        APP_BUS.get_or_init(|| AppBus::new())
    }

    pub fn post_event(&self, event: BusEvent) {
        // 消费事件并分发到 UI 侧的桥接层（若有 UI 订阅即可实时渲染）
        // 当前阶段：日志输出与桥接数据更新
        match &event {
            BusEvent::QueryResult(data) => {
                // 更新数据桥
                set_query_datas(data.clone());
            }
            BusEvent::ExportResult(_data) => {
                // 暂不实现导出结果桥接，后续可扩展
            }
            BusEvent::DataExtractedAction(_db) => {
                // 数据提取完成，后续桥接实现
            }
        }
        for h in self.handlers.lock().unwrap().iter() {
            h(event.clone());
        }
    }

    pub fn register_ui_consumer<F>(&self, f: F)
    where
        F: Fn(BusEvent) + Send + Sync + 'static,
    {
        self.handlers.lock().unwrap().push(Box::new(f));
    }
}

pub fn post(event: BusEvent) {
    AppBus::init().post_event(event);
}

// Convenience helper to publish a QueryAction from data path
// Note: Phase A skeleton - constructing QueryAction is not exposed yet due to private field.
