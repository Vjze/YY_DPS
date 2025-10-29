use std::collections::{HashMap, HashSet};
use sqlx_oldapi::{MssqlPool, query_as, Error as SqlxError, Row};
use sqlx_oldapi::mssql::MssqlRow;
use futures::stream::TryStreamExt;
use chrono::NaiveDateTime;
use crate::{structs::{Data, Datas, PackData}, utils::{error::MyError, sql::client}};

#[derive(Debug)]
struct QueryRow {
    pub sn: String,
    pub box_no: String, 
    pub yypn: String,   
    pub pack_worker: String, 
    pub create_time: NaiveDateTime, 
}

impl sqlx_oldapi::FromRow<'_, MssqlRow> for QueryRow {
    fn from_row(row: &MssqlRow) -> Result<Self, SqlxError> {
        Ok(QueryRow {
            sn: row.try_get("sn")?,
            box_no: row.try_get("Pack_no")?,
            yypn: row.try_get("pn")?,
            pack_worker: row.try_get("creator")?,
            create_time: row.try_get("createtime")?,
        })
    }
}

fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    all_datas
        .into_iter()
        .map(|d| {
            let mut map = HashMap::new();
            // 插入PackData字段
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // 插入Data字段
            map.insert("sn".to_string(), d.sn_data.sn);
            map.insert("yypn".to_string(), d.sn_data.yypn);
            map
        })
        .collect()
}

pub async fn get_box_datas(
    box_no: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    let pool: &MssqlPool = &client().await?;

    // 预检查：所有条件为空且不使用时间查询
    if box_no.is_empty() && pn.is_empty() && date_time_start.is_empty() && date_time_end.is_empty() && !use_time {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }

    // 构建动态SQL查询
    let mut sql = String::from("SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE PnOptionID = '-100'");
    let mut params = Vec::new();
    let mut param_index = 1;

    // 添加box_no条件
    if !box_no.is_empty() {
        sql.push_str(&format!(" AND Pack_no = @P{}", param_index));
        params.push(box_no.clone());
        param_index += 1;
    }

    // 添加pn条件
    if !pn.is_empty() {
        sql.push_str(&format!(" AND pn = @P{}", param_index));
        params.push(pn.clone());
        param_index += 1;
    }

    // 添加时间范围条件
    if use_time && !date_time_start.is_empty() && !date_time_end.is_empty() {
        sql.push_str(&format!(" AND createtime BETWEEN @P{} AND @P{}", param_index, param_index + 1));
        params.push(date_time_start);
        params.push(date_time_end);
        param_index += 2;
    }

    // 添加排序
    sql.push_str(" ORDER BY CreateTime DESC");

    // 创建查询并绑定参数
    let mut query = query_as::<_, QueryRow>(&sql);
    for param in params {
        query = query.bind(param);
    }

    // 执行查询
    let mut rows = query.fetch(pool);
    
    // 处理结果
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
        let sn = row.sn.clone();
        
        // 去重逻辑
        if seen_sns.contains(&sn) {
            continue;
        }

        let pack_data = PackData {
            box_no: row.box_no,
            pack_worker: row.pack_worker,
            pack_packtime: row.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let sn_data = Data {
            sn: sn.clone(),
            yypn: row.yypn,
            ..Default::default()
        };

        all_datas.push(Datas {
            pack_data,
            sn_data,
            ..Default::default()
        });
        seen_sns.insert(sn);
    }

    if all_datas.is_empty() {
        let query_key = match (box_no.is_empty(), pn.is_empty()) {
            (false, _) => &box_no,
            (_, false) => &pn,
            (true, true) => "时间范围",
        };
        return Err(MyError::Zdyknown(format!("查询条件 '{}' 没有找到数据", query_key)));
    }
    
    Ok(format_data(all_datas))
}