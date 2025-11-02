use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub yypn: String,
    pub sn: String,
    pub ith: String,
    pub vf: String,
    pub im: String,
    pub po: String,
    pub rs: String,
    pub se: String,
    pub iop: String,
    pub kink: String,
    pub imkink: String,
    pub sen: String,
    pub vbr: String,
    pub res: String,
    pub icc: String,
    pub idark: String,
    pub testtime: String,
    pub result: String,
    pub tester: String,
    pub i_xtalk: String,
    pub mdpid: String,
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
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
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