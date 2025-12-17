use std::{collections::HashMap, path::PathBuf};

use crate::{
    configs::column_map_config::add_new_template_map,
    utils::error::{MyError, MyTip},
};
use anyhow::Result;
use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::info;
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
    pub row: String,
    pub infos: HashMap<String, InfoDetail>,
    pub tables: HashMap<String, String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfoDetail {
    pub roudan: bool,
    #[serde(rename = "roudan_min")]
    pub roudan_min: String,
    #[serde(rename = "roudan_max")]
    pub roudan_max: String,
    #[serde(rename = "roudan_size")]
    pub roudan_size: String,
    #[serde(rename = "data_type")]
    pub data_type: String,
    #[serde(rename = "data_select")]
    pub data_select: String,
    #[serde(rename = "fixed_content")]
    pub fixed_content: String,
    #[serde(rename = "decimal")]
    pub decimal: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub row: String,
    pub infos: HashMap<String, InfoDetail>,
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

pub async fn get_decimal_config_value(template_name: String) -> Result<TemplateConfig, MyError> {
    let data = load_data().await?; // 加载所有模板数据
    if data.is_empty() {
        return Err(MyError::UnLoadedTemplates);
    }

    // 查找匹配的模板
    if let Some(item) = data
        .into_iter()
        .find(|item| item.template_name == template_name)
    {
        // 匹配成功，直接返回 infos 和 tables
        let template_config = TemplateConfig {
            row: item.row,
            infos: item.infos,   // 直接使用 HashMap<String, InfoDetail>
            tables: item.tables, // 直接使用 HashMap<String, String>
        };

        // 检查是否为空，如果 infos 和 tables 都为空，则报错
        if template_config.infos.is_empty() && template_config.tables.is_empty() {
            return Err(MyError::None(format!(
                "模板: {} 的 infos 和 tables 字段均为空",
                template_name
            )));
        }

        Ok(template_config)
    } else {
        // 未找到匹配的模板
        Err(MyError::None(format!("模板: {}", template_name)))
    }
}

pub async fn add_new_template(
    template_name: String,
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
    let mut excel_column_keys = HashMap::new();
    for header in headers {
        let result = re.replace_all(&header, "");
        excel_column_keys.insert(result.to_string(), "".to_string());
    }
    let mut final_infos = HashMap::new(); // 使用空的 HashMap 作为默认值

    for column_name in excel_column_keys.keys() {
        // 对于每一个 Excel 列名，插入一个默认的 InfoDetail，确保配置完整
        final_infos.entry(column_name.clone()).or_insert_with(InfoDetail::default);
    }
    // let mut row_map = HashMap::new();
    // row_map.insert("rows".to_string(), rows.to_string());
    let new_item = Template {
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        row: rows,
        infos: final_infos,
        tables: HashMap::new(),
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok(MyTip::AddDone(format!("模板: {}", template_name)))
}

pub async fn update_template(
    template_name: String,
    template_config: TemplateConfig,
) -> Result<MyTip, MyError> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;

    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
            item.row = template_config.row;
            item.infos = template_config.infos;
            item.tables = template_config.tables;
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
