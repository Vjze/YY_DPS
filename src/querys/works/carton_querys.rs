use chrono::NaiveDateTime;
use futures::TryStreamExt;
use sqlx_oldapi::mssql::MssqlArguments;
use sqlx_oldapi::mssql::MssqlRow;
use sqlx_oldapi::{Error as SqlxError, Row, query::QueryAs, query_as};
use std::collections::{HashMap, HashSet};

use crate::{
    structs::{CartonData, Data, Datas, PackData},
    utils::{error::MyError, sql::client},
};

#[derive(Debug)]
struct QueryRow {
    pub sn: String,
    pub box_no: String,
    pub pn: String,
    pub pack_worker: String,
    pub pack_createtime: NaiveDateTime,
    pub carton_worker: String,
    pub carton_createtime: NaiveDateTime,
    pub carton_no: String,
}

// 假设 SQL SELECT 部分已经修正了别名
impl sqlx_oldapi::FromRow<'_, MssqlRow> for QueryRow {
    fn from_row(row: &MssqlRow) -> Result<Self, SqlxError> {
        Ok(QueryRow {
            sn: row.try_get("sn")?,
            box_no: row.try_get("Pack_no")?,
            pn: row.try_get("pn")?,
            pack_worker: row.try_get("pack_creator")?,
            pack_createtime: row.try_get("pack_time")?,
            carton_worker: row.try_get("carton_creator")?,
            carton_createtime: row.try_get("carton_time")?,
            carton_no: row.try_get("CartonNo")?,
        })
    }
}

fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    let all = all_datas
        .into_iter()
        .map(|d| {
            // 展平 Datas 为 HashMap
            let mut map = HashMap::new();
            // CartonData
            map.insert("carton_no".to_string(), d.carton_data.carton_no);
            map.insert("yypn".to_string(), d.carton_data.yypn);
            map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
            map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // Data
            map.insert("sn".to_string(), d.sn_data.sn);

            map
        })
        .collect::<Vec<HashMap<String, String>>>();
    all
}

// ======================================================================
// SQL 常量定义 (使用巨大的字符串字面量，保证 'static 生命周期和字面量要求)
// ======================================================================

// A. 仅按 carton
const SQL_CARTON_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.CartonNo = @P1
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// B. 按 carton 和 pn
const SQL_CARTON_PN: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.CartonNo = @P1 AND b.pn = @P2
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// C. 按 carton 和 时间
const SQL_CARTON_TIME: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.CartonNo = @P1 AND b.createtime BETWEEN @P2 AND @P3
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// D. 仅按 pn 和 时间
const SQL_PN_TIME: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.pn = @P1 AND b.createtime BETWEEN @P2 AND @P3
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// E. 所有条件
const SQL_ALL: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.CartonNo = @P1 AND b.pn = @P2 AND b.createtime BETWEEN @P3 AND @P4
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// F. 仅按 时间
const SQL_TIME_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.createtime BETWEEN @P1 AND @P2
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// G. 仅按 pn
const SQL_PN_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100' AND b.pn = @P1
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";
// H. 兜底 (无条件)
const SQL_DEFAULT: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

pub async fn get_carton_datas(
    carton: String,
    use_time: bool,
    date_time_start: String,
    date_time_end: String,
    pn: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    let pool = &client().await?;
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    if carton.is_empty()
        && pn.is_empty()
        && date_time_start.is_empty()
        && date_time_end.is_empty()
        && !use_time
    {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }
    let carton_cloned = carton.clone();
    let pn_cloned = pn.clone();
    let start_cloned = date_time_start.clone();
    let end_cloned = date_time_end.clone();

    // 2. 使用 if/else 块构建类型安全的查询对象
    let final_query: QueryAs<'static, sqlx_oldapi::Mssql, QueryRow, MssqlArguments>;

    // 注意：这里的 .bind() 直接接收 String (所有权被转移或克隆)，而不是引用 &String。

    // A. 仅按 carton 查 (无 pn, 无时间)
    if !carton_cloned.is_empty() && pn_cloned.is_empty() && !use_time {
        final_query = query_as(SQL_CARTON_ONLY).bind(carton_cloned.clone()); 

    // B. 按 carton 和 pn 查 (无时间)
    } else if !carton_cloned.is_empty() && !pn_cloned.is_empty() && !use_time {
        final_query = query_as(SQL_CARTON_PN)
            .bind(carton_cloned.clone())
            .bind(pn_cloned.clone()); 

    // C. 按 carton 和时间查 (无 pn)
    } else if !carton_cloned.is_empty() && pn_cloned.is_empty() && use_time {
        final_query = query_as(SQL_CARTON_TIME)
            .bind(carton_cloned.clone())
            .bind(start_cloned.clone())
            .bind(end_cloned.clone()); 

    // D. 仅按 pn 和时间查 (无 carton)
    } else if carton_cloned.is_empty() && !pn_cloned.is_empty() && use_time {
        final_query = query_as(SQL_PN_TIME)
            .bind(pn_cloned.clone())
            .bind(start_cloned.clone())
            .bind(end_cloned.clone()); 

    // E. 按 carton, pn, 和时间查 (所有条件)
    } else if !carton_cloned.is_empty() && !pn_cloned.is_empty() && use_time {
        final_query = query_as(SQL_ALL)
            .bind(carton_cloned.clone())
            .bind(pn_cloned.clone())
            .bind(start_cloned.clone())
            .bind(end_cloned.clone()); 

    // F. 仅按 时间查 (无 carton, 无 pn)
    } else if carton_cloned.is_empty() && pn_cloned.is_empty() && use_time {
        final_query = query_as(SQL_TIME_ONLY)
            .bind(start_cloned.clone())
            .bind(end_cloned.clone()); 

    // G. 仅按 pn 查 (无 carton, 无时间)
    } else if carton_cloned.is_empty() && !pn_cloned.is_empty() && !use_time {
        final_query = query_as(SQL_PN_ONLY).bind(pn_cloned.clone()); 
    } else {
        // 兜底查询
        final_query = query_as(SQL_DEFAULT);
    }

    // 3. 执行查询 (逻辑不变)
    let mut rows = final_query.fetch(pool);

    while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
        let sn = row.sn.clone();

        if seen_sns.contains(&sn) {
            continue;
        }

        let carton_data = CartonData {
            carton_no: row.carton_no,
            yypn: row.pn.clone(),
            carton_worker: row.carton_worker,
            carton_packtime: row
                .carton_createtime
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.box_no,
            pack_worker: row.pack_worker,
            pack_packtime: row.pack_createtime.format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let sn_data = Data {
            sn: sn.clone(),
            yypn: row.pn,
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };
        all_datas.push(data);
        seen_sns.insert(sn);
    }

    if all_datas.is_empty() {
        // 由于 final_query 已经接管了参数的所有权，这里的 carton/pn 仍然可用 (因为我们用的是 .clone())
        let query_key = if carton_cloned.is_empty() {
            &pn_cloned
        } else {
            &carton_cloned
        };
        return Err(MyError::Zdyknown(format!(
            "查询条件 '{}' 没有找到数据",
            query_key
        )));
    }

    let datas = format_data(all_datas);
    Ok(datas)
}
