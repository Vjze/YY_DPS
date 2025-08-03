use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;

const TOML_FILE_PATH: &str = "././configs/all_column_name.toml"; // TOML 文件路径
// const TOML_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\all_column_name.toml"; // TOML 文件路径
// 定义结构体来匹配 TOML 文件中的键值对
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllColumnNames {
    // 使用 HashMap 来灵活地存储所有列名和对应的中文描述
    #[serde(flatten)] // 将所有字段平铺到 HashMap 中
    pub columns: HashMap<String, String>,
}

fn get_file_path() -> PathBuf {
    PathBuf::from(TOML_FILE_PATH)
}/// 加载所有列名配置数据
pub async fn load_all_column_names() -> Result<Vec<String>, String> {
    let path = get_file_path();
    if !path.exists() {
        fs::write(&path, "")
            .await
            .map_err(|e| format!("创建 TOML 文件失败: {}", e))?;
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取 TOML 文件失败: {}", e))?;

    let all_names: AllColumnNames = toml::from_str(&contents)
        .map_err(|e| format!("解析 TOML 文件失败: {}", e))?;
    let mut all_name = vec![];
    for (key,_value) in all_names.columns {
        all_name.push(key);
    }
    Ok(all_name)
}

/// 保存所有列名配置数据
pub async fn save_all_column_names(data: &HashMap<String, String>) -> Result<(), String> {
    let path = get_file_path();
    let all_names = AllColumnNames {
        columns: data.clone(),
    };
    let toml_string = toml::to_string_pretty(&all_names)
        .map_err(|e| format!("序列化数据到 TOML 失败: {}", e))?;
    fs::write(&path, toml_string)
        .await
        .map_err(|e| format!("写入 TOML 文件失败: {}", e))
}