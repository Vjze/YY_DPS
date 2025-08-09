use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::utils::error::{MyError, MyTip};

const TOML_FILE_PATH: &str = "././configs/column_map_config.toml"; // TOML 文件路径
// const TOML_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\column_map_config.toml"; // TOML 文件路径

// 定义根结构体，用于匹配 TOML 文件中的 [[ColumnMapConfig]]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnMapConfigRoot {
    #[serde(rename = "ColumnMapConfig")] // 映射 TOML 中的 [[ColumnMapConfig]]
    pub column_maps: Vec<ColumnMapConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMapConfig {
    pub id: u32,
    pub template_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    #[serde(flatten)]
    /// 使用 `flatten` 属性将字段映射到 TOML 对象的顶层
    pub fields: HashMap<String, String>,
}

pub async fn get_map_templates() -> Result<Vec<String>, MyError> {
    let data = load_data().await?;
    let mut templates = Vec::new();
    if data.is_empty() {
        return Err(MyError::UnLoadedTemplates);
    }
    for item in data {
        if !templates.contains(&item.template_name) {
            templates.push(item.template_name);
        }
    }
    Ok(templates)
}

pub async fn get_template_map_config(
    template_name: String,
) -> Result<HashMap<String, String>, MyError> {
    let data = load_data().await?;
    let mut map_infos = HashMap::new();
    let excluded_columns: HashSet<&str> = ["template_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    if data.is_empty() {
        return Err(MyError::UnLoadedTemplates);
    }
    for item in data {
        if item.template_name == template_name {
            for (key, value) in item.fields.iter() {
                if !excluded_columns.contains(key.as_str()) {
                    map_infos.insert(key.to_string(), value.clone());
                }
            }
        }
    }

    Ok(map_infos)
}

pub async fn add_new_template_map(
    template_name: String,
    new_fields: HashMap<String, String>,
) -> Result<(), MyError> {
    let mut data = load_data().await?;
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    let new_id = if let Some(last) = data.last() {
        last.id + 1
    } else {
        1
    };
    let new_item = ColumnMapConfig {
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        fields: new_fields,
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok(())
}

pub async fn update_template_map(
    template_name: String,
    map_infos: HashMap<String, String>,
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;
    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            for (key, string_value) in map_infos {
                if string_value == "None" || string_value == "none" {
                    item.fields.insert(key, "".to_string());
                    continue;
                } else {
                    item.fields.insert(key, string_value.to_string());
                }
            }
            break;
        }
    }
    if found {
        save_data(&data).await?;
        Ok(MyTip::UpdateDone(format!("映射模板: {}", template_name)))
    } else {
        Err(MyError::None(format!("映射模板: {}", template_name)))
    }
}

pub async fn delete_template_map(template_name: String) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.template_name != template_name);
    if data.len() == initial_length {
        return Err(MyError::None(format!("映射模板: {}", template_name)));
    }
    save_data(&data).await?;
    Ok(MyTip::DeleteDone(format!("映射模板: {}", template_name)))
}

fn get_file_path() -> PathBuf {
    PathBuf::from(TOML_FILE_PATH)
}

pub async fn save_data(data: &[ColumnMapConfig]) -> Result<(), MyError> {
    let path = get_file_path();
    let config_root = ColumnMapConfigRoot {
        column_maps: data.to_vec(),
    };
    let toml_string = toml::to_string_pretty(&config_root)?;
    fs::write(&path, toml_string).await?;
    Ok(())
}

pub async fn load_data() -> Result<Vec<ColumnMapConfig>, MyError> {
    let path = get_file_path();

    let contents = fs::read_to_string(&path).await?;
    let config_root: ColumnMapConfigRoot = toml::from_str(&contents)?;
    Ok(config_root.column_maps)
}
