use crate::{
    configs::type_config::get_type_infos,
    structs::{BandData, CartonData, Data, Datas, PackData, RowData},
    utils::{
        error::MyError,
        sql::{client, get_tables},
    },
};
use futures::TryStreamExt;
use sqlx_oldapi::{MssqlPool, query_as};
use std::collections::{HashMap, HashSet};
use tokio::{
    fs,
    io::{AsyncBufReadExt as _, BufReader},
    task,
};
use tracing::info;
async fn get_info() -> anyhow::Result<Vec<String>, MyError> {
    info!("开始执行文件选择...");
    let pick = rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(MyError::Zdyknown(format!("选择框关闭，查询取消。")));
    let path = if let Ok(path) = pick {
        let p_str = path.path().display().to_string();
        info!("文件选择成功: path={}", p_str);
        p_str
    } else {
        return Err(MyError::Zdyknown(format!("未选择文件.")));
    };
    let op = fs::File::open(&path).await?;
    let mut infos = vec![];
    let mut reader = BufReader::new(op).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        infos.push(line);
    }
    info!("文件内容读取完毕，共 {} 行", infos.len());
    Ok(infos)
}
pub async fn do_carton_query(
    carton: String,
    typeinfos: String,
    is_multi: bool,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    info!(
        "开始执行箱号查询: carton={}, typeinfos={}, is_multi={}",
        carton, typeinfos, is_multi
    );
    let client = client().await?;
    let pool = &client;
    if carton.is_empty() && is_multi {
        info!("执行批量查询模式");
        let cartons = get_info().await?;
        let mut datas = Vec::new();
        for carton in cartons {
            let typeinfos = typeinfos.clone();
            let res = carton_query_datas(carton.clone(), &pool, typeinfos).await?;
            datas.extend(res);
        }
        Ok(datas)
    } else {
        info!("执行单箱查询模式");
        carton_query_datas(carton, pool, typeinfos).await
    }
}
pub async fn carton_query_datas(
    carton: String,
    pool: &MssqlPool,
    typeinfos: String,
) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
    if carton.is_empty() {
        return Err(MyError::CartonNoEmpty);
    }
    let infos = get_type_infos(typeinfos).await?.1;
    info!("类型信息解析完毕: {:?}", infos);
    // 将所有并存条件放入 match 元组中，处理所有组合
    let all_datas = match (
        infos.is_have_pch,
        infos.carton_pch,
        infos.zdy_box,
        infos.jz_band,
    ) {
        (true, true, true, true) => {
            get_data_for_pch_carton_with_zdy_with_jzband(carton, pool).await?
        }
        (true, true, true, false) => {
            get_data_for_pch_carton_with_zdy_no_jzband(carton, pool).await?
        }
        (true, true, false, true) => {
            get_all_data_for_carton_with_pch_with_jzband(carton, pool).await?
        } // Existing
        (true, true, false, false) => get_all_data_for_carton_with_pch(carton, pool).await?, // Existing

        // A.2: box_pch = true (is_have_pch=true, carton_pch=false)
        (true, false, true, true) => {
            get_data_for_pch_box_with_zdy_with_jzband(carton, pool).await?
        }
        (true, false, true, false) => get_data_for_pch_box_with_zdy_no_jzband(carton, pool).await?,
        (true, false, false, true) => {
            get_all_data_for_box_with_pch_with_jzband(carton, pool).await?
        } // Existing
        (true, false, false, false) => get_all_data_for_box_with_pch(carton, pool).await?, // Existing

        // --- 场景 B: is_have_pch = false ---
        // 此时 carton_pch 和 box_pch 均为 false，因此第二个参数用 _ 通配
        (false, _, true, true) => get_data_for_no_pch_with_zdy_with_jzband(carton, pool).await?,
        (false, _, true, false) => get_data_for_no_pch_with_zdy_no_jzband(carton, pool).await?,
        (false, _, false, true) => get_data_no_pch_with_jzband(carton, pool).await?, // Existing
        (false, _, false, false) => get_data_no_pch(carton, pool).await?,            // Existing
    };

    info!("基础数据获取完毕，开始查询最新测试数据");
    // 查询每个 SN 的最新 TestDate
    let sn_placeholders = all_datas
        .iter()
        .map(|s| format!("'{}'", s.sn_data.sn))
        .collect::<Vec<String>>();

    let datas = if sn_placeholders.len() < 1000 {
        let sn_list = sn_placeholders.join(", ");
        let sql_text_s = build_query_sql(&sn_list, pool).await?;
        execute_query(&sql_text_s, pool).await?
    } else {
        // 创建拥有的 String，避免临时值
        let v1 = sn_placeholders[..1000].join(", ");
        let v2 = sn_placeholders[1000..].join(", ");
        info!("SN数量超过1000，将执行并行查询");
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let (data1, data2) = futures::try_join!(
            task::spawn(async move {
                let sql_text_s_1 = build_query_sql(&v1, &pool1).await?;
                execute_query(&sql_text_s_1, &pool1).await
            }),
            task::spawn(async move {
                let sql_text_s_2 = build_query_sql(&v2, &pool2).await?;
                execute_query(&sql_text_s_2, &pool2).await
            })
        )
        .unwrap();

        // 合并结果
        let mut data1 = data1?;
        data1.extend(data2?);
        data1
    };
    info!("测试数据获取完毕，开始整合数据");
    // 合并 TestDate 数据并转换为 HashMap
    let mut all = all_datas
        .into_iter()
        .map(|mut d| {
            let sn_datas = datas
                .iter()
                .find(|x| x.sn == d.sn_data.sn)
                .map(|s| s.clone())
                .unwrap();
            d.sn_data = sn_datas;

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
            map.insert("testtime".to_string(), d.sn_data.testtime);
            map.insert("result".to_string(), d.sn_data.result);
            map.insert("tester".to_string(), d.sn_data.tester);
            map.insert("i_xtalk".to_string(), d.sn_data.i_xtalk);
            map.insert("mdpid".to_string(), d.sn_data.mdpid);
            map
        })
        .collect::<Vec<HashMap<String, String>>>();
    info!("数据整合完毕，开始排序");
    // 按 box_no 排序
    all.sort_by(|a, b| a.get("box_no").unwrap().cmp(b.get("box_no").unwrap()));
    info!("排序完毕，查询结束");
    Ok(all)
}

async fn get_data_for_no_pch_with_zdy_no_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_for_no_pch_with_zdy_no_jzband: select a.sn,c.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
    SELECT 
        a.sn AS sn, 
        c.pkg_no AS pkg_no, 
        a.pn AS pn, 
        a.creator AS creator, 
        a.createtime AS createtime, 
        b.creator AS carton_creator, 
        b.createtime AS carton_createtime
    FROM [mes_Factory].[dbo].[MaterialPackSn] a
    INNER JOIN [mes_Factory].[dbo].[packing_carton] b ON a.Pack_no = b.Packing_no
    INNER JOIN [mes_Factory].[dbo].[jz_carton_bind] c ON a.Pack_no = c.box_no
    WHERE b.CartonNo = @P1 AND b.PnOptionID = '-100' 
    ORDER BY b.CreateTime DESC, a.Pack_no ASC
    ";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);
    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }
        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,                      // 映射到 yypn
            carton_worker: row.carton_creator, // 映射到 carton_worker
            carton_packtime: carton_time,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,       // 映射到 box_no
            pack_worker: row.creator, // 映射到 pack_worker
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }

    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
async fn get_data_for_no_pch_with_zdy_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_for_no_pch_with_zdy_with_jzband: select c.sn,a.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query = "select c.sn AS sn, a.pkg_no AS pkg_no, c.pn AS pn, d.creator AS creator, c.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime
    from [mes_Factory].[dbo].[jz_carton_bind] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
    inner join [mes_Factory].[dbo].[MaterialPackSn] c on c.Pack_no=b.Packing_no
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);
    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }
    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        // if band_data == Default::default() {
        //     break;
        // }
        // data.band_data = band_data;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}
async fn get_data_for_pch_box_with_zdy_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_for_pch_box_with_zdy_with_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select d.sn AS sn, a.pkg_no AS pkg_no, d.pn AS pn, d.creator AS creator, d.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[jz_carton_bind] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on d.Pack_no=c.LABEL_KEY
    inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, d.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}
async fn get_data_for_pch_box_with_zdy_no_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_for_pch_box_with_zdy_no_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select d.sn AS sn, a.pkg_no AS pkg_no, d.pn AS pn, d.creator AS creator, d.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[jz_carton_bind] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on d.Pack_no=c.LABEL_KEY
    inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, d.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);
    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
async fn get_data_for_pch_carton_with_zdy_no_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_for_pch_carton_with_zdy_no_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query =  "
      select d.sn AS sn, a.pkg_no AS pkg_no, d.pn AS pn, d.creator AS creator, d.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[jz_carton_bind] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Box_no=c.LABEL_KEY
    inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, d.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
async fn get_data_for_pch_carton_with_zdy_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行get_data_for_pch_carton_with_zdy_with_jzband: select d.sn,a.pkg_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select d.sn AS sn, a.pkg_no AS pkg_no, d.pn AS pn, d.creator AS creator, d.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[jz_carton_bind] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.box_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.box_no=c.LABEL_KEY
    inner join [mes_Factory].[dbo].[MaterialPackSn] d on d.Pack_no=b.Packing_no
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, d.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定信息获取完毕，开始整合数据");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}
async fn get_all_data_for_box_with_pch_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_all_data_for_box_with_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select a.sn AS sn, a.Pack_no AS pkg_no, a.pn AS pn, a.creator AS creator, a.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}
async fn get_all_data_for_box_with_pch(
    carton: String,
    // verified_pchs_string: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_all_data_for_box_with_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select a.sn AS sn, a.Pack_no AS pkg_no, a.pn AS pn, a.creator AS creator, a.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on a.Pack_no=c.LABEL_KEY
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
// 新增函数：根据已验证的 parameter 字符串获取所有数据
async fn get_all_data_for_carton_with_pch_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_all_data_for_carton_with_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "select a.sn AS sn,a.Pack_no AS pack_no,a.pn AS pn,a.creator AS creator,a.createtime AS createtime,b.creator AS carton_creator,b.createtime AS carton_createtime,c.parameter AS parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}
async fn get_all_data_for_carton_with_pch(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_all_data_for_carton_with_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select a.sn AS sn, a.Pack_no AS pkg_no, a.pn AS pn, a.creator AS creator, a.createtime AS createtime, b.creator AS carton_creator, b.createtime AS carton_createtime, c.parameter AS parameter
    from [mes_Factory].[dbo].[MaterialPackSn] a
    inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
    inner join [mes_Factory].[dbo].[packing_LABEL_PRINT_LOG] c on b.CartonNo=c.LABEL_KEY
    where b.CartonNo=@P1 and b.PnOptionID = '-100'
    order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let p = row.parameter.clone();
        let year = extract_value(&p, "YEAR");
        let week = extract_value(&p, "WEEK");
        let pch = match (year, week) {
            (Some(y), Some(w)) => {
                // 只取 YEAR 的最后两位数并拼接 WEEK
                let year_last_two = &y[y.len() - 2..]; // 提取最后两位
                let pch = format!("{}{}", year_last_two, w);
                pch
            }
            _ => {
                let pch = format!("{}", "None");
                pch
            }
        };
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            pch,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
async fn get_data_no_pch(carton: String, pool: &MssqlPool) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_no_pch: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select a.sn AS sn,a.Pack_no AS pack_no,a.pn AS pn,a.creator AS creator,a.createtime AS createtime,b.creator AS carton_creator,b.createtime AS carton_createtime
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            where b.CartonNo=@P1
            and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    info!("数据处理完毕，准备返回结果");
    Ok(all_datas)
}
async fn get_data_no_pch_with_jzband(
    carton: String,
    pool: &MssqlPool,
) -> anyhow::Result<Vec<Datas>, MyError> {
    let mut all_datas = Vec::new();
    let mut seen_sns = HashSet::new(); // 用于存储已见的 sn
    info!(
        "开始执行 get_data_no_pch_with_jzband: select a.sn,a.Pack_no... where b.CartonNo='{}'",
        carton
    );
    let query = "
      select a.sn AS sn,a.Pack_no AS pack_no,a.pn AS pn,a.creator AS creator,a.createtime AS createtime,b.creator AS carton_creator,b.createtime AS carton_createtime
            from [mes_Factory].[dbo].[MaterialPackSn] a
            inner join [mes_Factory].[dbo].[packing_carton] b on a.Pack_no=b.Packing_no
            where b.CartonNo=@P1
            and b.PnOptionID = '-100' order by b.CreateTime desc, b.Packing_no desc, a.Pack_no asc";
    let mut rows = query_as::<sqlx_oldapi::Mssql, RowData>(query)
        .bind(&carton)
        .fetch(pool);

    info!("SQL查询执行完毕，开始处理结果集");
    while let Some(row) = rows.try_next().await? {
        if seen_sns.contains(&row.sn) {
            continue;
        }

        let pack_time = row.createtime.format("%Y-%m-%d %H:%M:%S").to_string();
        let carton_time = row
            .carton_createtime
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // 4. 构造最终的 Datas 结构体
        let carton_data = CartonData {
            carton_no: carton.clone(),
            yypn: row.pn,
            carton_worker: row.carton_creator,
            carton_packtime: carton_time,
            ..Default::default()
        };
        let pack_data = PackData {
            box_no: row.pkg_no,
            pack_worker: row.creator,
            pack_packtime: pack_time,
        };
        let sn_data = Data {
            sn: row.sn.clone(),
            ..Default::default()
        };

        let data = Datas {
            carton_data,
            pack_data,
            sn_data,
            ..Default::default()
        };

        all_datas.push(data);
        seen_sns.insert(row.sn);
    }

    info!("基础数据处理完毕，开始获取绑定数据");
    if all_datas.is_empty() {
        return Err(MyError::NoResult(format!("箱号:{carton}")));
    }
    let mut band_datas = vec![];
    for data in &mut all_datas {
        // 获取 band_data
        let band_data = get_band_data(data.sn_data.sn.clone(), pool).await?;
        band_datas.push(band_data);
    }
    info!("绑定数据获取完毕，开始整合");
    let a_datas = all_datas
        .iter_mut()
        .filter_map(|d| {
            // 尝试在 band_datas 中找到匹配的元素
            let b_data = band_datas.iter().find(|b| d.sn_data.sn == b.w_sn);

            // 使用 if let 来安全地处理 Option
            if let Some(band_data) = b_data {
                d.band_data = band_data.clone();
                Some(d.clone())
            } else {
                // 如果没有找到匹配的 band_data，就返回 None，filter_map 会自动过滤掉这个元素
                println!(
                    "Warning: No matching band_data found for SN: {}",
                    d.sn_data.sn
                );
                d.band_data.w_sn = d.sn_data.sn.clone();
                Some(d.clone())
            }
        })
        .collect::<Vec<Datas>>();
    info!("数据整合完毕，准备返回结果");
    Ok(a_datas)
}

async fn get_band_data(sn: String, pool: &MssqlPool) -> Result<BandData, MyError> {
    info!("开始为SN: {} 查询绑定数据", sn);
    let query = "
    SELECT 
        SN_ShipMent AS b_sn,  
        Sn AS w_sn,             
        userno AS band_worker,   
        Relationtime AS band_time 
    FROM [mes_Factory].[dbo].[QA_snRelation]
    WHERE sn=@P1
    ";
    let rows = query_as::<sqlx_oldapi::Mssql, BandData>(query)
        .bind(&sn)
        .fetch_optional(pool)
        .await?;
    info!("SN: {} 绑定数据查询完毕", sn);
    let band_data = match rows {
        Some(band_data) => band_data,
        None => {
            info!("SN: {} 未找到绑定数据，返回默认空值。", sn);
            BandData {
                b_sn: "".to_string(),
                w_sn: "".to_string(),
                band_time: "".to_string(),
                band_worker: "".to_string(),
            }
        }
    };
    Ok(band_data)
}

pub async fn execute_query(sql_text_s: &str, pool: &MssqlPool) -> Result<Vec<Data>, MyError> {
    info!("开始执行 carton_query 查询: {}", sql_text_s);
    let mut rows = query_as::<sqlx_oldapi::Mssql, Data>(sql_text_s).fetch(pool);
    info!("查询执行完毕，开始处理结果集...");
    let mut sn_map: HashMap<String, Data> = HashMap::new();
    let mut row_count = 0;
    while let Some(data) = rows.try_next().await.map_err(MyError::from)? {
        row_count += 1;
        let sn = data.sn.clone();

        // 3. 只保留最新的测试数据 (去重逻辑不变)
        if let Some(existing_data) = sn_map.get(&sn) {
            if existing_data.testtime < data.testtime {
                sn_map.insert(sn, data);
            }
        } else {
            sn_map.insert(sn, data);
        }
    }
    let datas: Vec<Data> = sn_map.into_iter().map(|(_, v)| v).collect();
    info!(
        "共处理 {} 行原始数据，去重后得到 {} 条最新SN数据。",
        row_count,
        datas.len()
    );
    if datas.is_empty() {
        return Err(MyError::NoResult(format!("")));
    }
    Ok(datas)
}
// 导入 MssqlPool，MyError 和 anyhow::Result
pub async fn build_query_sql(sn_list: &str, pool: &MssqlPool) -> anyhow::Result<String, MyError> {
    let aliased_cols_10g = "SN AS sn, Ith AS ith, Pf AS po, Vop AS vf, Im AS im, Rs AS rs, Se AS se, Sen AS sen, Res AS res, ICC AS icc, Vbr AS vbr, Kink AS kink, imkink AS imkink, TestDate AS testtime, Idark AS idark, Result AS result, ProductBill AS tester, iop AS iop, ixtalk AS i_xtalk, MDPId AS mdpid, testtype AS yypn";

    let aliased_cols_others = "SN AS sn, Ith AS ith, Po AS po, Vf AS vf, Im AS im, Rs AS rs, Pslop AS se, Sen AS sen, Res AS res, ICC AS icc, Vbr AS vbr, Kink_I AS kink, kinkim_i AS imkink, TestDate AS testtime, Idark AS idark, Result AS result, ProductBill AS tester, io AS iop, xtalk AS i_xtalk, Te AS mdpid, testtype AS yypn";

    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        aliased_cols_10g
    );
    let mut sql_text = String::from(&sql_10);

    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", aliased_cols_others, i);
        sql_text.push_str(&s);
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE result = 'OK' ORDER BY testtime DESC")
    } else {
        format!(
            "WHERE sn IN ({}) AND result = 'OK' ORDER BY testtime DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}
pub fn extract_value(input: &str, key: &str) -> Option<String> {
    let search_key = format!("{}=", key);
    if let Some(start_index) = input.find(&search_key) {
        let start = start_index + search_key.len();
        if let Some(end_index) = input[start..].find(';') {
            let end = start + end_index;
            return Some(input[start..end].to_string());
        } else {
            // 如果没有分号，则取到字符串末尾
            return Some(input[start..].to_string());
        }
    }
    None
}
