use chrono::NaiveDateTime;
use futures::stream::TryStreamExt;
use sqlx_oldapi::FromRow;
use sqlx_oldapi::query_as;
use std::collections::{HashMap, HashSet};

use crate::{
    structs::{CartonData, Data, Datas, PackData},
    utils::{error::MyError, sql::client},
};

/// 数据库查询结果行映射结构体（使用 FromRow 宏自动生成映射逻辑）
#[derive(Debug, FromRow)]
struct QueryRow {
    pub sn: String,
    #[sqlx(rename = "Pack_no")]
    pub box_no: String,
    pub pn: String,
    #[sqlx(rename = "pack_creator")]
    pub pack_worker: String,
    #[sqlx(rename = "pack_time")]
    pub pack_createtime: NaiveDateTime,
    #[sqlx(rename = "carton_creator")]
    pub carton_worker: String,
    #[sqlx(rename = "carton_time")]
    pub carton_createtime: NaiveDateTime,
    #[sqlx(rename = "CartonNo")]
    pub carton_no: String,
}

/// 格式化日期时间为字符串（yyyy-MM-dd HH:mm:ss）
fn format_datetime(dt: NaiveDateTime) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 将 Datas 结构体转换为 HashMap
fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
    all_datas
        .into_iter()
        .map(|d| {
            let mut map = HashMap::new();
            // 插入 CartonData 字段
            map.insert("carton_no".to_string(), d.carton_data.carton_no);
            map.insert("yypn".to_string(), d.carton_data.yypn);
            map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
            map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
            // 插入 PackData 字段
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // 插入 Data 字段
            map.insert("sn".to_string(), d.sn_data.sn);
            map
        })
        .collect()
}

// SQL 常量：完整字面量，无 concat!
const SQL_CARTON_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.CartonNo = @P1
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_CARTON_PN: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.CartonNo = @P1 AND b.pn = @P2
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_CARTON_TIME: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.CartonNo = @P1 AND b.createtime BETWEEN @P2 AND @P3
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_PN_TIME: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.pn = @P1 AND b.createtime BETWEEN @P2 AND @P3
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_ALL: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.CartonNo = @P1 AND b.pn = @P2 AND b.createtime BETWEEN @P3 AND @P4
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_TIME_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.createtime BETWEEN @P1 AND @P2
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

const SQL_PN_ONLY: &str = "
    SELECT a.sn, a.Pack_no, a.pn, 
           a.creator AS pack_creator, a.createtime AS pack_time, 
           b.creator AS carton_creator, b.createtime AS carton_time, b.CartonNo
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    WHERE b.PnOptionID = '-100'
    AND b.pn = @P1
    ORDER BY b.CreateTime DESC, b.Packing_no DESC, a.Pack_no ASC
";

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
) -> Result<Vec<HashMap<String, String>>, MyError> {
    // 验证时间参数有效性
    if use_time && (date_time_start.is_empty() || date_time_end.is_empty()) {
        return Err(MyError::Zdyknown("时间范围不能为空".to_string()));
    }

    // 检查是否所有条件都为空
    if carton.is_empty() && pn.is_empty() && !use_time {
        return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
    }

    let pool = client().await?;
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new();

    // 选择查询语句 - 显式指定 QueryRow 类型
    let query = match (!carton.is_empty(), !pn.is_empty(), use_time) {
        (true, false, false) => query_as::<_, QueryRow>(SQL_CARTON_ONLY).bind(&carton),
        (true, true, false) => query_as::<_, QueryRow>(SQL_CARTON_PN)
            .bind(&carton)
            .bind(&pn),
        (true, false, true) => query_as::<_, QueryRow>(SQL_CARTON_TIME)
            .bind(&carton)
            .bind(&date_time_start)
            .bind(&date_time_end),
        (false, true, true) => query_as::<_, QueryRow>(SQL_PN_TIME)
            .bind(&pn)
            .bind(&date_time_start)
            .bind(&date_time_end),
        (true, true, true) => query_as::<_, QueryRow>(SQL_ALL)
            .bind(&carton)
            .bind(&pn)
            .bind(&date_time_start)
            .bind(&date_time_end),
        (false, false, true) => query_as::<_, QueryRow>(SQL_TIME_ONLY)
            .bind(&date_time_start)
            .bind(&date_time_end),
        (false, true, false) => query_as::<_, QueryRow>(SQL_PN_ONLY).bind(&pn),
        (false, false, false) => query_as::<_, QueryRow>(SQL_DEFAULT),
    };

    // 执行查询并处理结果
    let mut rows = query.fetch(&pool);
    while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
        let sn = row.sn.clone();
        if seen_sns.contains(&sn) {
            continue;
        }

        let carton_data = CartonData {
            carton_no: row.carton_no,
            yypn: row.pn.clone(),
            carton_worker: row.carton_worker,
            carton_packtime: format_datetime(row.carton_createtime),
            ..Default::default()
        };

        let pack_data = PackData {
            box_no: row.box_no,
            pack_worker: row.pack_worker,
            pack_packtime: format_datetime(row.pack_createtime),
        };

        let sn_data = Data {
            sn: sn.clone(),
            yypn: row.pn,
            ..Default::default()
        };

        all_datas.push(Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        });
        seen_sns.insert(sn);
    }

    // 处理空结果
    if all_datas.is_empty() {
        let query_key = if !carton.is_empty() {
            &carton
        } else if !pn.is_empty() {
            &pn
        } else {
            "时间范围"
        };
        return Err(MyError::Zdyknown(format!(
            "查询条件 '{}' 没有找到数据",
            query_key
        )));
    }

    Ok(format_data(all_datas))
}
