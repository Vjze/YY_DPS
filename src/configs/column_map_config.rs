use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::fs;
use umya_spreadsheet::reader::xlsx::read;

// const JSON_FILE_PATH: &str = r"E:\Rust\hyd_datas_export\rust\src\column_map_config.json"; // JSON 文件路径
const JSON_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\column_map_config.json"; // JSON 文件路径

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapConfig {
    pub id: u32,
    pub template_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    #[serde(flatten)]
    /// 使用 `flatten` 属性将字段映射到 JSON 对象的顶层
    pub fields: HashMap<String, String>,
}

pub async fn get_map_templates() -> Result<Vec<String>, String> {
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

pub async fn get_template_map_config(
    template_name: String,
) -> Result<HashMap<String, String>, String> {
    let data = load_data().await?;
    let mut map_infos = HashMap::new();
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
                if !excluded_columns.contains(key.as_str()) {
                    map_infos.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    Ok(map_infos)
}

pub async fn add_new_template_map(
    template_name: String,
) -> Result<String, String> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let re = Regex::new(r"^[A-Za-z0-9_]+").unwrap();
    let mut data = load_data().await?;
    // 检查 template_name 是否存在
    if data.iter().any(|item| item.template_name == template_name) {
        return Err("映射已存在".to_string());
    }
    let file_path = rfd::AsyncFileDialog::new()
        .set_title("选择 Excel 模板")
        .add_filter("XLSX 文件", &["xlsx"])
        .pick_file()
        .await
        .ok_or("选择框关闭")?;
    let path = file_path.path();
    let mut book = read(&path).map_err(|_| format!("无法读取模板文件: {}", path.display()))?;
    let sheet = book
        .get_sheet_by_name_mut("Sheet1")
        .ok_or("找不到 Sheet1".to_string())?;
    let mut headers = vec![];
    for col in 1.. {
        match sheet.get_cell((col, 1)) {
            Some(cell) if !cell.get_value().trim().is_empty() => {
                headers.push(cell.get_value().to_string());
            }
            _ => break,
        }
    }
    let new_id = if let Some(last) = data.last() {
        last.id + 1
    } else {
        1
    };
    let mut new_fields = HashMap::new();
    for header in headers {
        if let Some(captures) = re.captures(&header) {
            let extracted = captures.get(0).map(|m| m.as_str()).unwrap_or("");
            new_fields.insert(extracted.to_string(), "".to_string());
        } else {
            new_fields.insert(header, "".to_string());
        }
    }
    let new_item = MapConfig {
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

pub async fn update_template_map(
    template_name: String,
    map_infos: HashMap<String, String>,
) -> Result<String, String> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;
    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            for (key, string_value) in map_infos {
                if string_value == "空" {
                    item.fields.insert(key, "".to_string());
                    continue;
                } else {
                    item.fields.insert(key, string_value);
                }
            }
            break;
        }
    }
    if found {
        save_data(&data).await?;
        Ok("映射更新成功".to_string())
    } else {
        Err("映射不存在".to_string())
    }
}

pub async fn delete_map_field(template_name: String, field_key: String) -> Result<String, String> {
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

pub async fn delete_template_map(template_name: String) -> Result<String, String> {
    let mut data = load_data().await?;
    let initial_length = data.len();
    data.retain(|item| item.template_name != template_name);
    if data.len() == initial_length {
        return Err("映射不存在".to_string());
    }
    save_data(&data).await?;
    Ok("映射删除成功".to_string())
}

pub async fn get_column_map_names(template_name: String) -> Result<Vec<String>, String> {
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

pub async fn save_data(data: &[MapConfig]) -> Result<(), String> {
    let path = get_file_path();
    let json_string =
        serde_json::to_string_pretty(data).map_err(|e| format!("序列化数据到 JSON 失败: {}", e))?;
    fs::write(&path, json_string)
        .await
        .map_err(|e| format!("写入 JSON 文件失败: {}", e))
}

pub async fn load_data() -> Result<Vec<MapConfig>, String> {
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
