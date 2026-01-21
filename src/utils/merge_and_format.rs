use std::collections::HashMap;

use crate::structs::{Data, Datas};

pub fn merge_and_format_results(
    all_datas: &mut Vec<Datas>,
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
            map.insert("carton_no".to_string(), d.carton_data.carton_no.clone());
            map.insert("pch".to_string(), d.carton_data.pch.clone());
            map.insert("yypn".to_string(), d.carton_data.yypn.clone());
            map.insert(
                "carton_worker".to_string(),
                d.carton_data.carton_worker.clone(),
            );
            map.insert(
                "carton_packtime".to_string(),
                d.carton_data.carton_packtime.clone(),
            );
            // PackData
            map.insert("box_no".to_string(), d.pack_data.box_no.clone());
            map.insert("pack_worker".to_string(), d.pack_data.pack_worker.clone());
            map.insert(
                "pack_packtime".to_string(),
                d.pack_data.pack_packtime.clone(),
            );
            // BandData
            map.insert("w_sn".to_string(), d.band_data.w_sn.clone());
            map.insert("b_sn".to_string(), d.band_data.b_sn.clone());
            map.insert("bandtime".to_string(), d.band_data.band_time.clone());
            map.insert("band_worker".to_string(), d.band_data.band_worker.clone());
            // Data
            map.insert("sn".to_string(), d.sn_data.sn.clone());
            map.insert("ith".to_string(), d.sn_data.ith.clone());
            map.insert("vf".to_string(), d.sn_data.vf.clone());
            map.insert("im".to_string(), d.sn_data.im.clone());
            map.insert("po".to_string(), d.sn_data.po.clone());
            map.insert("rs".to_string(), d.sn_data.rs.clone());
            map.insert("se".to_string(), d.sn_data.se.clone());
            map.insert("iop".to_string(), d.sn_data.iop.clone());
            map.insert("kink".to_string(), d.sn_data.kink.clone());
            map.insert("imkink".to_string(), d.sn_data.imkink.clone());
            map.insert("sen".to_string(), d.sn_data.sen.clone());
            map.insert("vbr".to_string(), d.sn_data.vbr.clone());
            map.insert("res".to_string(), d.sn_data.res.clone());
            map.insert("icc".to_string(), d.sn_data.icc.clone());
            map.insert("idark".to_string(), d.sn_data.idark.clone());
            map.insert(
                "testdate".to_string(),
                d.sn_data
                    .testdate
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .clone(),
            );
            map.insert("result".to_string(), d.sn_data.result.clone());
            map.insert("tester".to_string(), d.sn_data.tester.clone());
            map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk.clone());
            map.insert("mdpid".to_string(), d.sn_data.mdpid.clone());
            map
        })
        .collect::<Vec<HashMap<String, String>>>()
}
