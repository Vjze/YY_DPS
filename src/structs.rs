use chrono::NaiveDateTime;
use tiberius_mappers::TryFromRow;
#[derive(Debug, Clone, Default, TryFromRow)]
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
    pub testdate: NaiveDateTime, // 统一别名 testtime (数据库列名 TestDate)
    pub idark: String,
    pub result: String,
    pub tester: String, // 统一别名 tester (数据库列名 ProductBill)
    pub iop: String,
    pub i_xtalk: String,
    pub mdpid: String, // 统一别名 mdpid
    pub yypn: String,  // 统一别名 yypn (数据库列名 testtype)
    pub box_no: Option<String>,
    pub carton_no: Option<String>,
}
#[derive(Debug, Clone, Default )]
pub struct CartonData {
    pub carton_no: String,
    pub pch: String,
    pub yypn: String,
    pub carton_worker: String,
    pub carton_packtime: String,
}
#[derive(Debug, Clone, Default )]
pub struct PackData {
    pub box_no: String,
    pub pack_worker: String,
    pub pack_packtime: String,
}
#[derive(Debug, Clone, Default )]
pub struct BandData {
    pub w_sn: String,
    pub b_sn: String,
    pub band_time: String,
    pub band_worker: String,
}
#[derive(Debug, Clone, Default )]
pub struct Datas {
    pub carton_data: CartonData,
    pub pack_data: PackData,
    pub sn_data: Data,
    pub band_data: BandData,
}
