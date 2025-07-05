
use super::query_utils::{build_query_sql, execute_query};

pub async fn sn_query_datas(sns: Vec<String>) -> Result<Vec<Data>, String> {
    if sns.is_empty() {
        return Err("SN为空".to_string());
    }
    let v: Vec<String> = sns.iter().map(|sn| format!("'{}'", sn)).collect();
    let sn_list = v.join(", ");
    let sql_text_s = build_query_sql(&sn_list).await?;
    execute_query(&sql_text_s).await
}