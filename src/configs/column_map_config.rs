use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::Value;
use umya_spreadsheet::reader::xlsx::read;

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
    pub fields: HashMap<String, Value>,
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
                    map_infos.insert(key.to_string(), value.as_str().unwrap().to_string());
                   
                    
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
    // 获取工作表中最高行号
    let highest_row = sheet.get_highest_row();
    if highest_row == 0 {
        // 如果工作表为空，直接返回空表头
        headers = Default::default();
    }
    // 确定表头扫描的行范围
    // 如果最高行小于2，表头最多只有一行，我们只检查这一行
    let header_rows_to_scan = if highest_row >= 2 {
        vec![highest_row - 1, highest_row]
    } else {
        vec![highest_row]
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
        if let Some(captures) = re.captures(&header) {
            let extracted = captures.get(0).map(|m| m.as_str()).unwrap_or("");
            new_fields.insert(extracted.to_string(), Value::String("".to_string()));
        } else {
            new_fields.insert(header, Value::String("".to_string()));
        }
    }
    let new_item = ColumnMapConfig {
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
                if string_value == "None" {
                    item.fields.insert(key, Value::String("".to_string()));
                    continue;
                } else {
                    item.fields.insert(key, Value::String(string_value.to_string()));
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
    PathBuf::from(TOML_FILE_PATH)
}

pub async fn save_data(data: &[ColumnMapConfig]) -> Result<(), String> {
    let path = get_file_path();
    let config_root = ColumnMapConfigRoot {
        column_maps: data.to_vec(),
    };
    let toml_string = toml::to_string_pretty(&config_root)
        .map_err(|e| format!("序列化数据到 TOML 失败: {}", e))?;
    fs::write(&path, toml_string)
        .await
        .map_err(|e| format!("写入 TOML 文件失败: {}", e))
}

pub async fn load_data() -> Result<Vec<ColumnMapConfig>, String> {
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空向量
        fs::write(&path, "[]")
            .await
            .map_err(|e| format!("创建 TOML 文件失败: {}", e))?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取 TOML 文件失败: {}", e))?;
    let config_root: ColumnMapConfigRoot = toml::from_str(&contents)
        .map_err(|e| format!("解析 TOML 文件失败: {}", e))?;
    Ok(config_root.column_maps)
}