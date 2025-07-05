use std::collections::HashMap;
use chrono::NaiveDateTime;
use crate::{structs::Data, utils::{error::MyError, sql::client}};


pub async fn build_query_sql(sn_list: &str, tables: Vec<String>) -> anyhow::Result<String, MyError> {
    let testtype =      "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk";
    let testtype_12 =   "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );

    let mut sql_text = String::from(&sql_10);
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }

    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }

    let query_ty = format!(
        "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
        sn_list
    );

    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn execute_query(sql_text_s: &str) -> Result<Vec<Data>, MyError> {
    let mut client = client().await?;
    let stream = client.query(sql_text_s, &[&1i32]).await?;
    let rowsets = stream.into_results().await
       ?;

    let mut sn_map: HashMap<String, Data> = HashMap::new();
    for rows in rowsets {
        for row in rows {
            let sn = row.get::<&str, _>(0).unwrap().to_string();
            let kink = row.get::<&str, _>(11).unwrap_or_default();
            let imkink = row.get::<&str, _>(12).unwrap_or_default();
            let data = Data {
                sn: sn.clone(),
                ith: row.get::<&str, _>(1).unwrap_or_default().to_string(),
                vf: row.get::<&str, _>(3).unwrap_or_default().to_string(),
                im: row.get::<&str, _>(4).unwrap_or_default().to_string(),
                po: row.get::<&str, _>(2).unwrap_or_default().to_string(),
                rs: row.get::<&str, _>(5).unwrap_or_default().to_string(),
                se: row.get::<&str, _>(6).unwrap_or_default().to_string(),
                sen: row.get::<&str, _>(7).unwrap_or_default().to_string(),
                res: row.get::<&str, _>(8).unwrap_or_default().to_string(),
                icc: row.get::<&str, _>(9).unwrap_or_default().to_string(),
                vbr: row.get::<&str, _>(10).unwrap_or("0.00").to_string(),
                kink: kink.to_string(),
                imkink: imkink.to_string(),
                testtime: row
                   .get::<NaiveDateTime, _>(13)
                   .unwrap()
                   .format("%Y-%m-%d %H:%M:%S")
                   .to_string(),
                tester: row.get::<&str, _>(16).unwrap_or_default().to_string(),
                iop: row.get::<&str, _>(17).unwrap_or_default().to_string(),
                idark: row.get::<&str, _>(14).unwrap_or_default().to_string(),
                result: row.get::<&str, _>(15).unwrap_or_default().to_string(),
                i_xtalk: row.get::<&str, _>(18).unwrap_or_default().to_string(),
            };
            if let Some(existing_data) = sn_map.get(&sn) {
                if existing_data.testtime < data.testtime {
                    sn_map.insert(sn.clone(), data);
                }
            } else {
                sn_map.insert(sn.clone(), data);
            }
        }
    }

    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();
    if datas.is_empty() {
        return Err(MyError::NoResult("".to_string()));
    }
    Ok(datas)
}