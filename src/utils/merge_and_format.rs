use std::collections::HashMap;

use crate::structs::{Data, Datas};

pub fn merge_and_format_results(
    all_datas: Vec<Datas>,
    test_datas: &Vec<Data>,
) -> Vec<HashMap<String, String>> {
    all_datas
        .into_iter()
        .map(|mut d| {
            // 找到对应的最新测试数据
            if let Some(sn_datas) = test_datas.iter().find(|x| x.sn == d.sn_data.sn) {
                d.sn_data = sn_datas.clone();
            }

            // 展平 Datas 为 HashMap
            let mut map = HashMap::new();
            // CartonData
            map.insert("carton_no".to_string(), d.carton_data.carton_no);
            map.insert("pch".to_string(), d.carton_data.pch);
            map.insert("yypn".to_string(), d.carton_data.yypn);
            map.insert("carton_worker".to_string(), d.carton_data.carton_worker);
            map.insert("carton_packtime".to_string(), d.carton_data.carton_packtime);
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no);
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
            map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
            // BandData
            map.insert("w_sn".to_string(), d.band_data.w_sn);
            map.insert("b_sn".to_string(), d.band_data.b_sn);
            map.insert("bandtime".to_string(), d.band_data.band_time);
            map.insert("band_worker".to_string(), d.band_data.band_worker);
            // Data
            map.insert("sn".to_string(), d.sn_data.sn);
            map.insert("ith".to_string(), d.sn_data.ith);
            map.insert("vf".to_string(), d.sn_data.vf);
            map.insert("im".to_string(), d.sn_data.im);
            map.insert("po".to_string(), d.sn_data.po);
            map.insert("rs".to_string(), d.sn_data.rs);
            map.insert("se".to_string(), d.sn_data.se);
            map.insert("iop".to_string(), d.sn_data.iop);
            map.insert("kink".to_string(), d.sn_data.kink);
            map.insert("imkink".to_string(), d.sn_data.imkink);
            map.insert("sen".to_string(), d.sn_data.sen);
            map.insert("vbr".to_string(), d.sn_data.vbr);
            map.insert("res".to_string(), d.sn_data.res);
            map.insert("icc".to_string(), d.sn_data.icc);
            map.insert("idark".to_string(), d.sn_data.idark);
            map.insert(
                "testdate".to_string(),
                d.sn_data.testdate.format("%Y-%m-%d %H:%M:%S").to_string(),
            );
            map.insert("result".to_string(), d.sn_data.result);
            map.insert("tester".to_string(), d.sn_data.tester);
            map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk);
            map.insert("mdpid".to_string(), d.sn_data.mdpid);
            map
        })
        .collect::<Vec<HashMap<String, String>>>()
}
