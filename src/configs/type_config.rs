use crate::utils::error::{MyError, MyTip};
use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use tokio::fs;

// 定义 JSON 文件的路径
// const JSON_FILE_PATH: &str = r"E:\Rust\hyd_datas_export\rust\src\type_config.json";
const JSON_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\type_config.json"; // JSON 文件路径
// TypeConfig 结构体，用于 JSON 序列化和反序列化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConfig {
    pub id: u32,
    pub type_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    pub template: HashMap<String, String>,
}

/// 获取指定类型名称的类型信息列表
pub async fn get_type_infos(type_name: String) -> Result<Vec<String>, MyError> {
    // 使用 ? 运算符替代 unwrap()，以传播可能的错误
    let data = load_data().await?;
    let mut type_infos = Vec::new();
    if data.is_empty() {
        return Err(MyError::NoResult(format!("型号: {}",type_name)));
    };
    let excluded_columns: HashSet<&str> = ["type_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    for data_item in data {
        if data_item.type_name == type_name {
            for (_key, value) in &data_item.template {
                if excluded_columns.contains(_key.as_str()) {
                    continue; // 跳过排除的列
                }
                type_infos.push(value.to_string());
            }
        }
    }
    Ok(type_infos)
}

/// 获取所有类型名称的列表
pub async fn get_type_names() -> Result<Vec<String>, MyError> {
    // 使用 ? 运算符替代 unwrap()，以传播可能的错误
    let data = load_data().await?;
    let mut type_names = Vec::new();
    if data.is_empty() {
        return Err(MyError::UnLoadedTypes);
    };
    for data_item in data {
        if !type_names.contains(&data_item.type_name) {
            type_names.push(data_item.type_name);
        }
    }
    Ok(type_names)
}

/// 添加新类型配置
pub async fn add_new_type(type_name: String, templates: Vec<String>) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    // 检查 type_name 是否已存在
    if data.iter().any(|item| item.type_name == type_name) {
        return Err(MyError::Haved(format!("型号: {}",type_name)));
    }
    // 将 template 转换为 HashMap<String, String>
    let template: HashMap<String, String> = templates
        .into_iter()
        .enumerate()
        .map(|(i, value)| (format!("{}", i + 1), value))
        .collect();
    // 添加新类型
    let new_type = TypeConfig {
        id: data.len() as u32 + 1,
        type_name: type_name.clone(),
        create_time: now.clone(),
        update_time: None,
        template,
    };
    data.push(new_type);

    save_data(&data).await?;
    Ok(MyTip::AddDone(format!("型号: {}",type_name)))
}

/// 更新现有类型配置
pub async fn update_type(type_name: String, template: Vec<String>) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    // 计算当前模板的最后一个索引，用于新模板的键
    let last_template_index = data
        .iter()
        .filter_map(|item| {
            if item.type_name == type_name {
                item.template
                    .keys()
                    .filter_map(|key| key.parse::<usize>().ok())
                    .max()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    // 将传入的 Vec<String> 转换为 HashMap<String, String>
    let template: HashMap<String, String> = template
        .into_iter()
        .enumerate() // 使用 enumerate 来给每个值一个递增的索引
        .map(|(i, value)| (format!("{}", last_template_index + 1 + i), value))
        .collect();

    // 查找并更新类型
    if let Some(item) = data.iter_mut().find(|item| item.type_name == type_name) {
        item.template = template;
        item.update_time = Some(now);
        save_data(&data).await?;
        Ok(MyTip::UpdateDone(format!("型号: {}",type_name)))
    } else {
        Err(MyError::None(format!("型号: {}",type_name)))
    }
}

/// 删除类型配置
pub async fn delete_type(type_name: String) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.type_name != type_name);

    if data.len() == initial_length {
        return Err(MyError::None(format!("型号: {}",type_name)));
    }

    save_data(&data).await?;
    Ok(MyTip::DeleteDone(format!("型号: {}",type_name)))
}

/// 保存数据到 JSON 文件
pub async fn save_data(data: &[TypeConfig]) -> Result<(), MyError> {
    let path = get_file_path();
    let json_string = serde_json::to_string_pretty(data)?;
    fs::write(&path, json_string).await?;
    Ok(())
}

/// 从 JSON 文件加载数据
pub async fn load_data() -> Result<Vec<TypeConfig>, MyError> {
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空向量
        fs::write(&path, "[]")
            .await
            ?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .await?;
    // 尝试从 JSON 字符串反序列化数据
    let tc = serde_json::from_str(&contents)?;
    Ok(tc)

}

/// 获取 JSON 文件路径
fn get_file_path() -> PathBuf {
    PathBuf::from(JSON_FILE_PATH)
}
