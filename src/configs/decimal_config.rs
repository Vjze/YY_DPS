use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::Value;

use crate::utils::error::{MyError, MyTip};

const TOML_FILE_PATH: &str = "././configs/decimal_config.toml";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "Template")]
    pub templates: Vec<Template>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    pub id: u32,
    pub template_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    #[serde(default)]
    pub int: HashMap<String, toml::Value>, // 对应 [Template.int]
    #[serde(default)]
    pub string: HashMap<String, toml::Value>, // 对应 [Template.string]
    #[serde(default)]
    pub infos: HashMap<String, toml::Value>, // 对应 [Template.infos]
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecimalConfig {
    pub numbers: HashMap<String, f64>,
    pub strings: HashMap<String, String>,
    pub tables: HashMap<String, toml::Value>, // 修改为 toml::Value 以支持混合类型
}

pub async fn get_templates() -> Result<Vec<String>, MyError> {
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

pub async fn get_decimal_config_value(
    template_name: String,
) -> Result<DecimalConfig, MyError> {
    let data = load_data().await?;

    if data.is_empty() {
        return Err(MyError::UnLoadedTemplates);
    }

    let mut numbers = HashMap::new();
    let mut strings = HashMap::new();
    let mut tables = HashMap::new();

    for item in data {
        if item.template_name == template_name {
            // 处理 int 字段（数字）
            for (key, value) in item.int {
                match value {
                    Value::Integer(i) => {
                        numbers.insert(key, i as f64);
                    }
                    Value::Float(f) => {
                        numbers.insert(key, f);
                    }
                    _ => {
                        return Err(MyError::Zdyknown(format!(
                            "字段 '{}' 在 int 中不是数字类型",
                            key
                        )));
                    }
                }
            }
            // 处理 string 字段（字符串）
            for (key, value) in item.string {
                if let Value::String(s) = value {
                    strings.insert(key, s);
                } else {
                    return Err(MyError::Zdyknown(format!(
                        "字段 '{}' 在 string 中不是字符串类型",
                        key
                    )));
                }
            }
            // 处理 infos 字段（任意类型）
            for (key, value) in item.infos {
                tables.insert(key, value); // 直接存储 toml::Value
            }
            break; // 找到匹配的模板后退出
        }
    }

    if numbers.is_empty() && strings.is_empty() && tables.is_empty() {
        return Err(MyError::None(format!("模板: {}", template_name)));
    }

    Ok(DecimalConfig {
        numbers,
        strings,
        tables,
    })
}

pub async fn add_new_template(
    template_name: String,
    numbers: HashMap<String, f64>,
    strings: HashMap<String, String>,
    tables: HashMap<String, toml::Value>, // 修改为 toml::Value
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;

    // 检查 template_name 是否存在
    if data.iter().any(|item| item.template_name == template_name) {
        return Err(MyError::Haved(format!("模板: {}", template_name)));
    }

    let new_id = if let Some(last) = data.last() {
        last.id + 1
    } else {
        1
    };

    let new_item = Template {
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        int: numbers
            .into_iter()
            .map(|(k, v)| (k, Value::Float(v)))
            .collect(),
        string: strings
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect(),
        infos: tables, // 直接使用 tables
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok(MyTip::AddDone(format!("模板: {}", template_name)))
}

pub async fn update_template(
    template_name: String,
    numbers: HashMap<String, f64>,
    strings: HashMap<String, String>,
    tables: HashMap<String, toml::Value>, // 修改为 toml::Value
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;

    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            item.int = numbers
                .into_iter()
                .map(|(k, v)| (k, Value::Float(v)))
                .collect();
            item.string = strings
                .into_iter()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();
            item.infos = tables; // 直接使用 tables
            break;
        }
    }

    if found {
        save_data(&data).await?;
        Ok(MyTip::UpdateDone(format!("模板: {}", template_name)))
    } else {
        Err(MyError::None(format!("模板: {}", template_name)))
    }
}

pub async fn delete_decimal_field(
    template_name: String,
    field_key: String,
    section: &str,
) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let mut template_found = false;
    let mut field_deleted = false;

    for item in &mut data {
        if item.template_name == template_name {
            template_found = true;
            let removed = match section {
                "int" => item.int.remove(&field_key),
                "string" => item.string.remove(&field_key),
                "infos" => item.infos.remove(&field_key),
                _ => return Err(MyError::Zdyknown(format!("无效的部分: {}", section))),
            };
            if removed.is_some() {
                field_deleted = true;
                item.update_time = Some(Local::now().format("%Y/%m/%d %H:%M:%S").to_string());
                break;
            }
        }
    }

    if !template_found {
        return Err(MyError::None(format!("模板: {}", template_name)));
    }
    if !field_deleted {
        return Err(MyError::Zdyknown(format!(
            "字段 '{}' 在模板 '{}' 的 '{}' 部分中不存在",
            field_key, template_name, section
        )));
    }

    save_data(&data).await?;
    Ok(MyTip::Zdyknown(format!(
        "字段 '{}' 已从模板 '{}' 的 '{}' 部分中删除成功",
        field_key, template_name, section
    )))
}

pub async fn delete_template(template_name: String) -> Result<MyTip, MyError> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.template_name != template_name);
    if data.len() == initial_length {
        return Err(MyError::None(format!("模板: {}", template_name)));
    }
    save_data(&data).await?;
    Ok(MyTip::DeleteDone(format!("模板: {}", template_name)))
}

pub async fn get_decimal_column_names(
    template_name: String,
    section: &str,
) -> Result<Vec<String>, MyError> {
    let data = load_data().await?;

    if data.is_empty() {
        return Err(MyError::UnLoadedTemplates);
    }

    let mut column_names = Vec::new();
    for item in data {
        if item.template_name == template_name {
            let keys = match section {
                "int" => item.int.keys(),
                "string" => item.string.keys(),
                "infos" => item.infos.keys(),
                _ => return Err(MyError::Zdyknown(format!("无效的部分: {}", section))),
            };
            column_names.extend(keys.map(|k| k.to_string()));
            break;
        }
    }

    if column_names.is_empty() {
        return Err(MyError::None(format!(
            "模板 '{}' 不存在或 '{}' 部分没有字段",
            template_name, section
        )));
    }
    Ok(column_names)
}

fn get_file_path() -> PathBuf {
    PathBuf::from(TOML_FILE_PATH)
}

pub async fn save_data(data: &[Template]) -> Result<(), MyError> {
    let path = get_file_path();
    let config = Config {
        templates: data.to_vec(),
    };
    let toml_string = toml::to_string_pretty(&config)?;
    fs::write(&path, toml_string).await?;
    Ok(())
}

pub async fn load_data() -> Result<Vec<Template>, MyError> {
    let path = get_file_path();
    if !path.exists() {
        fs::write(&path, "").await?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path).await?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config.templates)
}