use std::{collections::HashMap, path::PathBuf};

use tokio::fs;

// const JSON_FILE_PATH: &str = r"E:\Rust\hyd_datas_export\rust\src\all_column_name.json"; // JSON 文件路径
const JSON_FILE_PATH: &str = r"\\192.168.10.142\Excel_Templates\Configs\all_column_name.json"; // JSON 文件路径
pub async fn get_all_name() -> Result<Vec<String>, String> {
    let data = load_data().await?;
    let mut all_keys: Vec<String> = data.keys().map(|s| s.to_string()).collect();
    // 对键进行排序，以便每次返回的顺序都是一致的
    all_keys.sort();
    Ok(all_keys)
}
pub async fn load_data() -> Result<HashMap<String, String>, String> {
    let path = get_file_path();
    if !path.exists() {
        // 如果文件不存在，则创建空文件并返回空的 HashMap
        fs::write(&path, "{}") // 注意：现在创建的是一个空 JSON 对象 {}
            .await
            .map_err(|e| format!("创建 JSON 文件失败: {}", e))?;
        return Ok(HashMap::new());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("读取 JSON 文件失败: {}", e))?;
    // 直接尝试将内容解析为 HashMap<String, String>
    serde_json::from_str(&contents).map_err(|e| format!("解析 JSON 文件失败: {}", e))
}

/// 获取 JSON 文件路径
fn get_file_path() -> PathBuf {
    PathBuf::from(JSON_FILE_PATH)
}
