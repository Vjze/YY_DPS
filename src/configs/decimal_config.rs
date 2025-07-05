use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;

// const JSON_FILE_PATH: &str = r"E:\Rust\hyd_datas_export\rust\src\decimal_config.json"; // JSON 文件路径
const JSON_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\decimal_config.json"; // JSON 文件路径
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportType {
    // 注意：这里使用了 Decimal 作为结构体名称，根据用户提供的文件片段，我将其统一为 ExportType
    pub id: u32,
    pub template_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/// 将 Vec<ExportType> 序列化为 JSON 字符串
pub fn to_json_string(data: &Vec<ExportType>) -> Result<String, String> {
    serde_json::to_string(data).map_err(|e| format!("序列化数据到 JSON 失败: {}", e))
}

pub async fn get_templates() -> Result<Vec<String>, String> {
    let data = load_data().await?;
    let mut templates = Vec::new();
    if data.is_empty() {
        return Err("没有找到任何配置数据".to_string());
    };
    for item in data {
        if !templates.contains(&item.template_name) {
            templates.push(item.template_name);
        }
    }
    Ok(templates)
}

pub async fn get_decimal_config(
    template_name: String,
) -> Result<(HashMap<String, String>, HashMap<String, String>), String> {
    let data = load_data().await?;
    let mut ints = HashMap::new();
    let mut srings = HashMap::new();
    let excluded_columns: HashSet<&str> = ["template_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    if data.is_empty() {
        return Err("没有找到任何配置数据".to_string());
    };
    for item in data {
        if item.template_name == template_name {
            for (key, value) in item.fields.iter() {
                if excluded_columns.contains(key.as_str()) {
                    continue;
                }
                if value.is_number() {
                    ints.insert(key.to_string(), value.to_string());
                } else {
                    // 包含 String 和其他类型
                    srings.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    Ok((ints, srings))
}
pub async fn get_decimal_config_value(
    template_name: String,
) -> Result<HashMap<String, Value>, String> {
    let data = load_data().await?;
    let mut decimal_config = HashMap::new();
    let excluded_columns: HashSet<&str> = ["template_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    if data.is_empty() {
        return Err("没有找到任何配置数据".to_string());
    };
    for item in data {
        if item.template_name == template_name {
            for (key, value) in item.fields.iter() {
                if excluded_columns.contains(key.as_str()) {
                    continue;
                }
                decimal_config.insert(key.to_string(), value.clone());
                // if value.is_number() {
                //     decimal_config.insert(key.to_string(), Value::Number(serde_json::Number::from_f64(value).unwrap()));
                // } else {
                //     // 包含 String 和其他类型
                //     decimal_config.insert(key.to_string(), Value::String(value.to_string()));
                // }
            }
        }
    }

    Ok(decimal_config)
}

pub async fn add_new_template(
    template_name: String,
    data_int: HashMap<String, String>,
    data_string: HashMap<String, String>,
) -> Result<String, String> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    let mut data = load_data().await?;
    // 检查 template_name 是否存在
    if data.iter().any(|item| item.template_name == template_name) {
        return Err("模板已存在".to_string());
    }
    let new_id = if let Some(last) = data.last() {
        last.id + 1
    } else {
        1
    };
    let mut new_fields = HashMap::new();
    for (key, string_value) in data_int {
        // 尝试解析为数字、布尔值，否则默认为字符串
        if let Ok(num) = string_value.parse::<i64>() {
            new_fields.insert(key, Value::Number(num.into()));
        } else if let Ok(fnum) = string_value.parse::<f64>() {
            let num = fnum as u32;
            new_fields.insert(
                key,
                Value::Number(num.into()),
            );
        } else {
            new_fields.insert(key, Value::String(string_value));
        }
    }
    for (key, string_value) in data_string {
        // 尝试解析为数字、布尔值，否则默认为字符串
        if let Ok(num) = string_value.parse::<i64>() {
            new_fields.insert(key, Value::Number(num.into()));
        } else if let Ok(fnum) = string_value.parse::<f64>() {
            new_fields.insert(
                key,
                Value::Number(serde_json::Number::from_f64(fnum).unwrap()),
            );
        } else if let Ok(b) = string_value.parse::<bool>() {
            new_fields.insert(key, Value::Bool(b));
        } else {
            new_fields.insert(key, Value::String(string_value));
        }
    }
    let new_item = ExportType {
        // 统一为 ExportType
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        fields: new_fields,
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok("模板添加成功".to_string())
}

pub async fn update_template(
    template_name: String,
    data_int: HashMap<String, String>,
    data_string: HashMap<String, String>,
) -> Result<String, String> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;
    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            for (key, string_value) in data_int {
                if let Ok(fnum) = string_value.parse::<f64>() {
                    let num = fnum as u32;
                    item.fields.insert(
                        key,
                        Value::Number(num.into()),
                    );
                } else if string_value.is_empty() {
                    // For number fields, if empty, default to 0
                    item.fields.insert(key, Value::Number(0.into()));
                }
            }
            for (key, new_string_value) in data_string {
                if new_string_value.starts_with('"')
                    && new_string_value.ends_with('"')
                    && new_string_value.len() >= 2
                {
                    let unquoted_string =
                        new_string_value[1..new_string_value.len() - 1].to_string();

                    item.fields.insert(key, Value::String(unquoted_string));
                } else {
                    item.fields.insert(key, Value::String(new_string_value));
                }
            }
            break;
        }
    }
    if found {
        save_data(&data).await?;
        Ok("模板更新成功".to_string())
    } else {
        Err("模板不存在".to_string())
    }
}
pub async fn delete_decimal_field(
    template_name: String,
    field_key: String,
) -> Result<String, String> {
    let mut data = load_data().await?;
    let mut template_found = false;
    let mut field_deleted = false;

    for item in &mut data {
        if item.template_name == template_name {
            template_found = true;
            if item.fields.remove(&field_key).is_some() {
                field_deleted = true;
                item.update_time = Some(Local::now().format("%Y/%m/%d %H:%M:%S").to_string());
                break; // 找到模板并删除字段后即可退出循环
            }
        }
    }

    if !template_found {
        return Err(format!("模板 '{}' 不存在", template_name));
    }
    if !field_deleted {
        return Err(format!(
            "字段 '{}' 在模板 '{}' 中不存在",
            field_key, template_name
        ));
    }

    save_data(&data).await?;
    Ok(format!(
        "字段 '{}' 已从模板 '{}' 中删除成功",
        field_key, template_name
    ))
}
pub async fn delete_template(template_name: String) -> Result<String, String> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.template_name != template_name);
    if data.len() == initial_length {
        return Err("模板不存在".to_string());
    }
    save_data(&data).await?;
    Ok("模板删除成功".to_string())
}

pub async fn get_decimal_column_names(template_name: String) -> Result<Vec<String>, String> {
    let data = load_data().await?;
    let mut column_names = Vec::new();
    let excluded_columns: HashSet<&str> = ["template_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    if data.is_empty() {
        return Err("没有找到任何配置数据".to_string());
    };
    for item in data {
        if item.template_name == template_name {
            for key in item.fields.keys() {
                if !excluded_columns.contains(key.as_str()) {
                    column_names.push(key.to_string());
                }
            }
        }
    }

    Ok(column_names)
}

fn get_file_path() -> PathBuf {
    PathBuf::from(JSON_FILE_PATH)
}

pub async fn save_data(data: &[ExportType]) -> Result<(), String> {
    // 统一为 ExportType
    let path = get_file_path();
    let json_string =
        serde_json::to_string_pretty(data).map_err(|e| format!("序列化数据到 JSON 失败: {}", e))?;
    fs::write(&path, json_string)
        .await
        .map_err(|e| format!("写入 JSON 文件失败: {}", e))
}
pub async fn load_data() -> Result<Vec<ExportType>, String> {
    // 统一为 ExportType
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空向量
        fs::write(&path, "[]")
            .await
            .map_err(|e| format!("创建 JSON 文件失败: {}", e))?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取 JSON 文件失败: {}", e))?;
    serde_json::from_str(&contents).map_err(|e| format!("解析 JSON 文件失败: {}", e))
}
