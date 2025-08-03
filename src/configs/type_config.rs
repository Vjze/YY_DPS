use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use tokio::fs;

use crate::utils::error::{MyError, MyTip};

// 定义 TOML 文件的路径
const TOML_FILE_PATH: &str = "././configs/type_config.toml";
// const JSON_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\type_config.toml"; // TOML 文件路径

// 定义 infos 结构体，用于匹配 TOML 文件中的 [ConfigType.infos]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Infos {
    pub is_have_pch: bool,
    pub carton_pch: bool,
    pub box_pch: bool,
    pub jz_band: bool,
}
// 定义根结构体，用于匹配 TOML 文件中的 [[ConfigType]]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeConfigRoot {
    #[serde(rename = "ConfigType")] // 映射 TOML 中的 [[ConfigType]]
    pub types: Vec<ConfigType>,
}

// ConfigType 结构体，用于 TOML 序列化和反序列化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigType {
    pub id: u32,
    pub type_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    pub infos: Infos,
    pub template: HashMap<String, String>,
}

/// 获取指定类型名称的类型信息列表
pub async fn get_type_infos(type_name: String) -> Result<(Vec<String>, Infos), MyError> {
    // 使用 ? 运算符替代 unwrap()，以传播可能的错误
    let data = load_data().await?;
    let mut type_infos = Vec::new();
    let mut infos = Infos::default();

    if data.is_empty() {
        return Err(MyError::NoResult(format!("型号: {}", type_name)));
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
            infos = data_item.infos;
        }
    }
    Ok((type_infos, infos))
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
pub async fn add_new_type(
    type_name: String,
    templates: Vec<String>,
    infos: Infos,
) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    // 检查 type_name 是否已存在
    if data.iter().any(|item| item.type_name == type_name) {
        return Err(MyError::Haved(format!("型号: {}", type_name)));
    }
    // 将 template 转换为 HashMap<String, String>
    let template: HashMap<String, String> = templates
        .into_iter()
        .enumerate()
        .map(|(i, value)| (format!("{}", i + 1), value))
        .collect();
    // 添加新类型
    let new_type = ConfigType {
        id: data.len() as u32 + 1,
        type_name: type_name.clone(),
        create_time: now.clone(),
        update_time: None,
        template,
        infos,
    };
    data.push(new_type);

    save_data(&data).await?;
    Ok(MyTip::AddDone(format!("型号: {}", type_name)))
}

/// 更新现有类型配置
pub async fn update_type(
    type_name: String,
    template: Vec<String>,
    infos: Infos,
) -> Result<MyTip, MyError> {
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
        item.infos = infos;
        item.update_time = Some(now);
        save_data(&data).await?;
        Ok(MyTip::UpdateDone(format!("型号: {}", type_name)))
    } else {
        Err(MyError::None(format!("型号: {}", type_name)))
    }
}

/// 删除类型配置
pub async fn delete_type(type_name: String) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.type_name != type_name);

    if data.len() == initial_length {
        return Err(MyError::None(format!("型号: {}", type_name)));
    }

    save_data(&data).await?;
    Ok(MyTip::DeleteDone(format!("型号: {}", type_name)))
}

/// 保存数据到 TOML 文件
pub async fn save_data(data: &[ConfigType]) -> Result<(), MyError> {
    let path = get_file_path();
    let config_root = TypeConfigRoot {
        types: data.to_vec(),
    };
    let toml_string = toml::to_string_pretty(&config_root)?;
    fs::write(&path, toml_string).await?;
    Ok(())
}

/// 从 TOML 文件加载数据
pub async fn load_data() -> Result<Vec<ConfigType>, MyError> {
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空向量
        fs::write(&path, "[]").await?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path).await?;
    println!("con = {}", contents);
    let config_root: TypeConfigRoot = toml::from_str(&contents)?;
    Ok(config_root.types)
}

/// 获取 TOML 文件路径
fn get_file_path() -> PathBuf {
    PathBuf::from(TOML_FILE_PATH)
}
