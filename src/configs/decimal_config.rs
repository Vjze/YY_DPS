use std::{collections::HashMap, path::PathBuf};

use crate::{
    configs::column_map_config::add_new_template_map,
    utils::error::{MyError, MyTip},
};
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx_oldapi::types::chrono::Local;
use tokio::fs;
use toml::Value;
use umya_spreadsheet::reader::xlsx::read;

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
    pub numbers: HashMap<String, i64>,
    pub strings: HashMap<String, String>,
    pub tables: HashMap<String, String>,
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

pub async fn get_decimal_config_value(template_name: String) -> Result<DecimalConfig, MyError> {
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
                        numbers.insert(key, i);
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
                if let Value::String(s) = value {
                    tables.insert(key, s);
                } else {
                    return Err(MyError::Zdyknown(format!(
                        "字段 '{}' 在 string 中不是字符串类型",
                        key
                    )));
                }
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
    mut template_infos: DecimalConfig,
    rows: String,
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let re = Regex::new(r"\r\n|\n|\r").expect("Invalid regex");

    let mut data = load_data().await?;
    // 检查 template_name 是否存在
    if data.iter().any(|item| item.template_name == template_name) {
        return Err(MyError::Haved(format!("模板：{} 已存在", template_name)));
    }
    let file_path = rfd::AsyncFileDialog::new()
        .set_title("选择 Excel 模板")
        .add_filter("XLSX 文件", &["xlsx"])
        .pick_file()
        .await
        .ok_or(MyError::Zdyknown("选择框关闭!".to_string()))?;
    let path = file_path.path();
    let mut book = read(&path)
        .map_err(|_| MyError::Zdyknown(format!("无法读取模板文件: {}", path.display())))?;
    let sheet = book
        .get_sheet_by_name_mut("Sheet1")
        .ok_or(MyError::Zdyknown(format!("找不到 Sheet1")))?;
    let mut headers = vec![];
    // 获取工作表中最高行号
    let highest_row = sheet.get_highest_row();
    if highest_row == 0 {
        // 如果工作表为空，直接返回空表头
        headers = Default::default();
    }
    // 确定表头扫描的行范围
    let header_rows_to_scan = if rows == "1" {
        vec![highest_row]
    } else {
        vec![highest_row - 1, highest_row]
    };
    // 从第1列开始，按列遍历
    for col_index in 1.. {
        let mut combined_header_for_col = String::new();
        let mut found_content_in_col = false; // 标志位，用于判断这一列在表头范围内是否有内容

        // 在确定的表头行范围内，按行遍历并拼接
        for &row_index in header_rows_to_scan.iter() {
            if let Some(cell) = sheet.get_cell((col_index, row_index)) {
                if !cell.get_value().trim().is_empty() {
                    combined_header_for_col.push_str(&cell.get_value().to_string());
                    found_content_in_col = true; // 这一列有内容
                }
            }
        }

        // 如果这一列在表头行范围内完全为空，则认为表头已结束
        if !found_content_in_col {
            // 在遇到第一个完全空列时，停止遍历
            break;
        }

        // 将拼接后的表头添加到 headers 列表中
        headers.push(combined_header_for_col);
    }
    let new_id = if let Some(last) = data.last() {
        last.id + 1
    } else {
        1
    };
    let mut new_fields = HashMap::new();
    for header in headers {
        let result = re.replace_all(&header, "");
        new_fields.insert(result.to_string(), "".to_string());
    }
    add_new_template_map(template_name.clone(), new_fields.clone()).await?;
    new_fields.insert("rows".to_string(), rows.to_string());
    template_infos.strings.extend(new_fields);
    let new_item = Template {
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        int: template_infos
            .numbers
            .into_iter()
            .map(|(k, v)| (k, Value::Integer(v as i64)))
            .collect(),
        string: template_infos
            .strings
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect(),
        infos: template_infos
            .tables
            .into_iter()
            .map(|(k, v)| (k, Value::String(v)))
            .collect(),
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok(MyTip::AddDone(format!("模板: {}", template_name)))
}

pub async fn update_template(
    template_name: String,
    template_infos: DecimalConfig,
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;

    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            item.int = template_infos
                .numbers
                .into_iter()
                .map(|(k, v)| (k, Value::Integer(v as i64)))
                .collect();
            item.string = template_infos
                .strings
                .into_iter()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();
            item.infos = template_infos
                .tables
                .into_iter()
                .map(|(k, v)| (k, Value::String(v)))
                .collect();
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
    let contents = fs::read_to_string(&path).await?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config.templates)
}
