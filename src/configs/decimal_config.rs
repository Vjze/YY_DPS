use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::Value;

const TOML_FILE_PATH: &str ="././decimal_config.toml"; // TOML 文件路径
// const TOML_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\decimal_config.toml"; // TOML 文件路径
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "Template")] // 对应 TOML 文件中的 [[ExportType]]
    pub templates: Vec<Template>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template {
    pub id: u32,
    pub template_name: String,
    pub create_time: String,
    pub update_time: Option<String>,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
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


pub async fn get_decimal_config_value(
    template_name: String,
) -> Result<HashMap<String, Value>, String> {
    let data = load_data().await?;
    let excluded_columns: HashSet<&str> = ["template_name", "id", "create_time", "update_time"]
        .iter()
        .cloned()
        .collect();
    if data.is_empty() {
        return Err("没有找到任何配置数据".to_string());
    };
    let datas: HashMap<String, Value> = data
        .into_iter()
        .filter(|item| item.template_name == template_name)
        .flat_map(|item| {
            item.fields
                .into_iter()
                .filter(|(key, _)| !excluded_columns.contains(key.as_str()))
        })
        .collect();
    Ok(datas)
}

pub async fn add_new_template(
    template_name: String,
    datas: HashMap<String,Value>
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
    
    let new_item = Template {
        id: new_id,
        template_name: template_name.clone(),
        create_time: now,
        update_time: None,
        fields: datas,
    };
    data.push(new_item);
    save_data(&data).await?;
    Ok("模板添加成功".to_string())
}

pub async fn update_template(
    template_name: String,
    datas: HashMap<String,Value>
) -> Result<String, String> {
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
    let mut data = load_data().await?;
    let mut found = false;
    for item in &mut data {
        if item.template_name == template_name {
            found = true;
            item.update_time = Some(now.clone());
                item.fields = datas;
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
    PathBuf::from(TOML_FILE_PATH)
}

pub async fn save_data(data: &[Template]) -> Result<(), String> {
    let path = get_file_path();
    // 使用 toml::to_string_pretty 进行序列化
    let config = Config {
        templates: data.to_vec(),
    };
    let toml_string =
        toml::to_string_pretty(&config).map_err(|e| format!("序列化数据到 TOML 失败: {}", e))?;
    fs::write(&path, toml_string)
        .await
        .map_err(|e| format!("写入 TOML 文件失败: {}", e))
}

pub async fn load_data() -> Result<Vec<Template>, String> {
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空向量
        fs::write(&path, "") // TOML 空数组通常表示为空文件
            .await
            .map_err(|e| format!("创建 TOML 文件失败: {}", e))?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取 TOML 文件失败: {}", e))?;
    // 使用 toml::from_str 进行反序列化
    let config: Config =
        toml::from_str(&contents).map_err(|e| format!("解析 TOML 文件失败: {}", e))?;
    Ok(config.templates) // 返回 Config 中的 templates 字段
}