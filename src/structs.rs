use serde::{Deserialize, Serialize};
use sqlx_oldapi::mssql::MssqlRow;
use sqlx_oldapi::types::chrono::NaiveDateTime;
use sqlx_oldapi::{Error as SqlxError, FromRow, Row};
use std::result::Result as StdResult; // 假设 MssqlRow 在这里
#[derive(FromRow, Debug)]
pub struct RowData {
    pub sn: String,
    pub pkg_no: String,                   // 对应 c.pkg_no (box_no)
    pub pn: String,                       // 对应 a.pn (yypn)
    pub creator: String,                  // 对应 a.creator (pack_worker)
    pub createtime: NaiveDateTime,        // 对应 a.createtime (pack_time)
    pub carton_creator: String,           // 对应 b.creator (carton_worker)
    pub carton_createtime: NaiveDateTime, // 对应 b.createtime (carton_time)
    #[sqlx(default)]
    pub parameter: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub sn: String,  // 统一别名 sn
    pub ith: String, // 统一别名 ith
    pub po: String,  // 统一别名 po
    pub vf: String,  // 统一别名 vf
    pub im: String,
    pub rs: String,
    pub se: String,
    pub sen: String,
    pub res: String,
    pub icc: String,
    pub vbr: String,
    pub kink: String,
    pub imkink: String,
    pub testdate: String, // 统一别名 testtime (数据库列名 TestDate)
    pub idark: String,
    pub result: String,
    pub tester: String, // 统一别名 tester (数据库列名 ProductBill)
    pub iop: String,
    pub i_xtalk: String,
    pub mdpid: String, // 统一别名 mdpid
    pub yypn: String,  // 统一别名 yypn (数据库列名 testtype)
}
impl sqlx_oldapi::FromRow<'_, MssqlRow> for Data {
    fn from_row(row: &MssqlRow) -> StdResult<Self, SqlxError> {
        // 使用 try_get("column_name") 替代 try_get(index)

        // 获取 NaiveDateTime (数据库列 TestDate AS testtime)
        let testtime_dt: NaiveDateTime = row.try_get("testdate")?;
        let testdate = testtime_dt.format("%Y-%m-%d %H:%M:%S").to_string();

        // 获取 mdpid (数据库列 MDPId/Te AS mdpid) 并应用业务逻辑
        let mdpid_raw: String = row.try_get("mdpid")?;
        let mdpid = if mdpid_raw == "0" {
            "".to_string()
        } else {
            mdpid_raw
        };

        // 构建 Data 结构体
        Ok(Data {
            sn: row.try_get("sn")?,
            ith: row.try_get("ith")?,
            po: row.try_get("po")?,
            vf: row.try_get("vf")?,
            im: row.try_get("im")?,
            rs: row.try_get("rs")?,
            se: row.try_get("se")?,
            sen: row.try_get("sen")?,
            res: row.try_get("res")?,
            icc: row.try_get("icc")?,
            vbr: row.try_get("vbr").unwrap_or_else(|_e| "0.00".to_string()),
            kink: row.try_get("kink")?,
            imkink: row.try_get("imkink")?,
            testdate, // 格式化后的时间
            idark: row.try_get("idark")?,
            result: row.try_get("result")?,
            tester: row.try_get("tester")?,
            iop: row.try_get("iop")?,
            i_xtalk: row.try_get("i_xtalk")?,
            mdpid, // 处理后的 mdpid
            yypn: row.try_get("yypn")?,
        })
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct CartonData {
    pub carton_no: String,
    pub pch: String,
    pub yypn: String,
    pub carton_worker: String,
    pub carton_packtime: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct PackData {
    pub box_no: String,
    pub pack_worker: String,
    pub pack_packtime: String,
}
#[derive(
    Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd, FromRow,
)]
pub struct BandData {
    pub w_sn: String,
    pub b_sn: String,
    pub band_time: String,
    pub band_worker: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Datas {
    pub carton_data: CartonData,
    pub pack_data: PackData,
    pub sn_data: Data,
    pub band_data: BandData,
}
