// export2excel.rs

use super::excel_utils::{before_im_num, before_ith_num, before_po, before_vf_num, calculate_tc};
use crate::{
    configs::{
        decimal_config::{InfoDetail, TemplateConfig, get_decimal_config_value},
        type_config::get_type_infos,
    },
    utils::error::MyError,
};
use chrono::{Local, NaiveDateTime};
use directories::UserDirs;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, sync::Arc};
use tracing::info;
use umya_spreadsheet::{
    Border, Font, HorizontalAlignmentValues, Style, VerticalAlignmentValues, reader::xlsx::read,
    writer::xlsx::write,
}; // 引入 info!

// 单元格更新结构
#[derive(Debug)]
struct CellUpdate {
    col: u32,
    row: u32,
    value: String,
}

pub async fn write_to_excel(
    type_name: &str,
    datas: Vec<HashMap<String, String>>,
    lock: bool,
) -> Result<(), MyError> {
    info!(
        "开始执行 write_to_excel, 类型: {}, 数据量: {}",
        type_name,
        datas.len()
    );
    let template_names: Vec<String> = get_type_infos(type_name).await?.0;
    let datas = Arc::new(datas.clone());
    let template_path = Arc::new(String::from(r"\\192.168.10.142\Excel_Templates\"));
    // let template_path = String::from(r"D:\rust\YY_DPS\");

    info!(
        "找到 {} 个关联模板: {:?}",
        template_names.len(),
        template_names
    );

    // 使用 tokio::spawn 创建异步任务，处理每个模板
    let tasks: Vec<_> = template_names
        .into_iter()
        .map(|template_name| {
            let datas = datas.clone();
            let template_path = template_path.clone();
            let tn_clone = template_name.clone(); // 用于日志记录
            tokio::spawn(async move {
                let result: Result<(), MyError> = async {
                    info!("开始处理模板: {}", tn_clone);
                    let decimal_config = get_decimal_config_value(template_name.clone()).await?;
                    let template_path = format!("{}{}.xlsx", template_path, template_name);
                    info!("正在读取模板文件: {}", template_path);

                    let mut book = read(&template_path)
                        .map_err(|e| MyError::Zdyknown(format!("无法读取模板文件: {}", e)))?;
                    info!("模板文件读取成功.");
                    let sheet = book
                        .get_sheet_mut(&0)
                        .ok_or(MyError::Zdyknown("找不到 Sheet1".to_string()))?;
                    let re = Regex::new(r"\r\n|\n|\r").unwrap();
                    // 提取表头
                    let mut headers = Vec::new();

                    // 获取工作表中最高行号
                    let highest_row = sheet.get_highest_row();
                    let highest_col = sheet.get_highest_column(); // 新增：获取最大列数
                    info!("工作表最高行号: {}, 最高列号: {}", highest_row, highest_col);
                    if highest_row == 0 {
                        // 如果工作表为空，直接返回空表头
                        headers = Default::default();
                    }
                    // let rows = if let Some(value) = decimal_config.row.get("rows") {
                    //     value.to_string()
                    // } else {
                    //     "1".to_string() // 默认值
                    // };
                    let rows = if decimal_config.row.clone().is_empty() {
                        "1".to_string()
                    } else {
                        decimal_config.row.clone()
                    };
                    info!("配置的表头行数: {}", rows);
                    // 确定需要扫描的行索引
                    let header_rows_to_scan = if rows == "1" {
                        vec![highest_row]
                    } else {
                        // 如果配置为多行（如 2），扫描最后两行
                        let start_row = if highest_row > 1 { highest_row - 1 } else { 1 };
                        vec![start_row, highest_row]
                    };

                    // // 从第1列开始，按列遍历
                    for col_index in 1..=highest_col {
                        let mut combined_header = String::new();
                        let mut has_value = false; // 标志位，用于判断这一列在表头范围内是否有内容

                        // 在确定的表头行范围内，按行遍历并拼接
                        for &row_index in header_rows_to_scan.iter() {
                            if let Some(cell) = sheet.get_cell((col_index, row_index)) {
                                if !cell.get_value().trim().is_empty() {
                                    combined_header.push_str(&cell.get_value().to_string());
                                    has_value = true; // 这一列有内容
                                }
                            }
                            if has_value {
                                // 清理换行符
                                headers.push(re.replace_all(&combined_header, "").to_string());
                            }
                        }
                    }
                    info!("提取到的原始列名: {:?}", headers);
                    let headers = headers
                        .iter()
                        .map(|h| re.replace_all(h, "").to_string())
                        .collect::<Vec<_>>();
                    info!("格式化后的列名: {:?}", headers);

                    let last_row = sheet.get_highest_row();
                    let decimal_config = Arc::new(decimal_config);
                    // 异步获取 column_name_mapping
                    // let column_name_mapping =
                    //     Arc::new(get_template_map_config(template_name.clone()).await?);
                    // let column_name_mapping = decimal_config.infos;
                    info!("开始并行生成单元格更新...");
                    let table_infos = decimal_config.tables.clone();
                    for (key, value) in table_infos {
                        let cell = sheet.get_cell_mut(key);
                        if value == "Date" {
                            let date = Local::now().date_naive();
                            cell.set_value(date.to_string());
                        } else if value == "DateTime" {
                            let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");
                            cell.set_value(datetime.to_string());
                        } else if value == "Quantity" {
                            let len = datas.len();
                            cell.set_value(len.to_string());
                        } else if value.is_empty() {
                            continue;
                        } else {
                            cell.set_value(value.to_string());
                        }
                    }
                    // 并行生成单元格更新（保留 rayon）
                    let cell_updates: Vec<CellUpdate> = datas
                        .par_iter()
                        .enumerate()
                        .flat_map(|(row_idx, row_data)| {
                            let row = last_row + row_idx as u32 + 1;
                            let raw_po_f64 = row_data
                                .get("po")
                                .unwrap_or(&"".to_string())
                                .parse::<f64>()
                                .unwrap_or(0.0);

                            // 2. 在此处调用一次随机函数，缓存结果
                            let cached_before_po = before_po(raw_po_f64);
                            headers
                                .iter()
                                .enumerate()
                                .map({
                                    let config = decimal_config.clone();
                                    // let column_mapping = column_name_mapping.clone();
                                    let unit = config.unit.clone();
                                    let unit = unit
                                        .to_string()
                                        .replace("\\\"", "")
                                        .replace("\"", "")
                                        .to_lowercase();
                                    move |(col_idx, header)| {
                                        let col = (col_idx + 1) as u32;
                                        // let row_value_header = calculate_row_value(
                                        //     header.to_string(),
                                        //     row_data,
                                        //     config.clone(),
                                        //     // column_mapping.clone(),
                                        //     // &unit,
                                        // );
                                        let (raw_value, mapped_key) = calculate_row_value(
                                            header,
                                            row_data,
                                            config.clone(),
                                            cached_before_po, // 传入缓存的温前值
                                            raw_po_f64,       // 传入温后值
                                        );
                                        let final_raw_value = if mapped_key == "deviceinfo_id"
                                            || mapped_key == "NO."
                                        {
                                            (row_idx + 1).to_string()
                                        } else {
                                            raw_value
                                        };
                                        let (value, _decimals) = format_with_decimals(
                                            &final_raw_value,
                                            config.clone().infos.clone(),
                                            &unit,
                                            &header, // <--- 修正：用 Excel 原始表头去查 decimal 配置
                                            &mapped_key, // <--- 修正：用 映射后的字段名去判断 "po"/"beforeTC" 逻辑
                                        );
                                        CellUpdate { col, row, value }
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect();
                    info!("单元格更新列表生成完毕，共 {} 个更新。", cell_updates.len());

                    // 构造样式一次性复用
                    let mut style = Style::default();
                    apply_cell_style(&mut style);
                    let style = Arc::new(style);

                    info!("开始将数据写入工作表...");
                    // 主线程写入
                    for update in cell_updates {
                        let cell = sheet.get_cell_mut((update.col, update.row));
                        cell.set_value_string(update.value.clone());

                        sheet
                            .get_style_mut((update.col, update.row))
                            .clone_from(&style);
                    }
                    if lock {
                        sheet
                            .get_sheet_protection_mut()
                            .set_password("test") // 设置保护密码
                            .set_sheet(true) // 开启保护
                            .set_objects(true) // 保护对象
                            .set_scenarios(true);
                    }

                    info!("数据写入完毕.");

                    // 保存
                    let user_dirs = UserDirs::new()
                        .ok_or(MyError::Zdyknown("无法获取用户目录.".to_string()))?;
                    let desktop_dir = user_dirs
                        .desktop_dir()
                        .ok_or(MyError::Zdyknown("无法找到桌面路径.".to_string()))?;
                    let target_folder = desktop_dir.join("数据导出");
                    if !target_folder.exists() {
                        std::fs::create_dir_all(&target_folder)?;
                    }

                    let date = Local::now().format("%Y-%m-%d").to_string();
                    let output_folder = target_folder.join(&date);
                    if !output_folder.exists() {
                        std::fs::create_dir_all(&output_folder)?;
                    }
                    let output_name = format!("{}-{}.xlsx", template_name, date);
                    let output_path = output_folder.join(output_name);
                    info!("准备保存文件到: {:?}", output_path);
                    write(&book, &output_path)
                        .map_err(|e| MyError::Zdyknown(format!("无法写入文件: {}", e)))?;
                    info!("模板 '{}' 处理并保存成功.", tn_clone);
                    Ok::<(), MyError>(())
                }
                .await;

                // 返回模板名称和处理结果
                (template_name, result)
            })
        })
        .collect();

    // 收集所有任务的结果
    let results = futures::future::join_all(tasks).await;
    let mut errors = vec![];

    // 检查每个任务的结果，收集错误
    for result in results {
        match result {
            Ok((_template_name, Ok(()))) => {} // 成功，无需操作
            Ok((template_name, Err(e))) => errors.push(format!("{}: {}", template_name, e)),
            Err(e) => errors.push(format!("任务执行失败: {:?}", e)),
        }
    }

    // 如果有错误，返回 Err 包含所有错误信息
    if errors.is_empty() {
        info!("所有模板处理完毕，无错误。");
        Ok(())
    } else {
        let error = errors.join(";");
        info!("处理过程中发现 {} 个错误: {}", errors.len(), error);
        Err(MyError::Zdyknown(error))
    }
}
fn format_with_decimals(
    val: &str,
    infos: HashMap<String, InfoDetail>,
    unit: &str,
    excel_header: &str, // <--- 关键参数1：原始表头，用于在 infos 中查找配置
    mapped_key: &str,   // <--- 关键参数2：映射字段，用于判断是否是 po 等特殊逻辑
) -> (String, Option<usize>) {
    // 1. 使用 excel_header 查找配置
    // 之前这里用的 mapped_key (如 beforeTC-Ith)，导致 infos.get 返回 None
    if let Some(info) = infos.get(excel_header) {
        if info.data_type == "数据" {
            // 解析配置的小数位
            if let Ok(decimals) = info.decimal.parse::<usize>() {
                // 解析数值
                if let Ok(mut num) = val.parse::<f64>() {
                    // 2. 根据 mapped_key 处理单位逻辑
                    // 修正布尔逻辑：((单位是mW 且 是po) 或者 (是beforeTC-Po))
                    if unit == "mw" {
                        if mapped_key == "po" || excel_header == "beforeTC-Po(mW)" {
                            num /= 1000.0;
                        }
                    }

                    // 3. 四舍五入 (Round Half Up)
                    let factor = 10_f64.powi(decimals as i32);
                    let temp_num = num * factor;

                    let rounded_num = if (temp_num.fract() - 0.5).abs() < f64::EPSILON {
                        temp_num.trunc() / factor
                    } else {
                        temp_num.round() / factor
                    };

                    // 4. 格式化输出
                    return (format!("{:.1$}", rounded_num, decimals), Some(decimals));
                }
            }
        } else if info.data_type == "固定文本" {
            return (info.fixed_content.clone(), None);
        }
    }

    // 兜底：如果没有配置或非数字，返回原值
    (val.to_string(), None)
}
// fn format_with_decimals(
//     val: &str,
//     // numble_config: Option<&i64>,
//     // string_config: Option<&String>,
//     infos: HashMap<String, InfoDetail>,
//     unit: &str,
//     header: &str,
// ) -> (String, Option<usize>) {
//     // 优先处理 numble_config
//     if let Some(info) = infos.get(header) {
//         if info.data_type == "数据" {
//             if let Ok(decimals) = info.decimal.parse::<usize>() {
//                 if let Ok(mut num) = val.parse::<f64>() {
//                     // let decimals = *n as usize;

//                     // 根据单位转换
//                     if unit == "mW" && header == "po" || header == "beforeTC-Po(mW)" {
//                         num /= 1000.0;
//                     }

//                     let factor = 10_f64.powi(decimals as i32);
//                     let temp_num = num * factor;

//                     // 自定义四舍五入逻辑
//                     let rounded_num = if (temp_num.fract() - 0.5).abs() < f64::EPSILON {
//                         temp_num.trunc() / factor
//                     } else {
//                         temp_num.round() / factor
//                     };

//                     let formatted_string =
//                         format!("{:.decimals$}", rounded_num, decimals = decimals);
//                     return (formatted_string, Some(decimals));
//                 } else {
//                     return (val.to_string(), None);
//                 }
//             } else {
//                 return (val.to_string(), None);
//             }
//         } else if info.data_type == "固定文本" {
//             return (info.fixed_content.clone(), None);
//         } else {
//             return ("".to_string(), None);
//         }
//     } else {
//         return (val.to_string(), None);
//     }
// }
fn calculate_row_value(
    excel_header: &str,
    row_data: &HashMap<String, String>,
    config: Arc<TemplateConfig>,
    before_po: f64,  // +++ 新增参数：缓存的温前值 +++
    raw_po_f64: f64, // +++ 新增参数：温后值 +++
) -> (String, String) {
    // 1. 获取配置信息
    let info_opt = config.infos.get(excel_header);

    // 2. 获取映射后的字段名 (data_select)
    let mapped_key = info_opt.map_or(excel_header, |info| {
        if info.data_type == "数据" && !info.data_select.is_empty() {
            info.data_select.as_str()
        } else {
            excel_header
        }
    });

    // 3. 优先检查是否是固定文本
    if let Some(info) = info_opt {
        if info.data_type == "固定文本" {
            // 如果是固定文本，直接返回内容和key
            return (info.fixed_content.clone(), mapped_key.to_string());
        }
    }

    let raw_value = match mapped_key {
        "beforeTC-Ith(mA)" => {
            let val = row_data
                .get("ith")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0);
            before_ith_num(val).to_string() // <--- 仅转字符串，不format
        }
        "beforeTC-Im(uA)" => {
            let val = row_data
                .get("im")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0);
            before_im_num(val).to_string()
        }
        "beforeTC-Po(mW)" => before_po.to_string(),
        "beforeTC-Vf(V)" => {
            let val = row_data
                .get("vf")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(0.0);
            before_vf_num(val).to_string()
        }
        "beforeTC-SE(W/A)" => {
            let po = before_po / 1000.0;
            let before_se = po / 20.0;
            before_se.to_string()
        }
        "deltaP（dB）" => {
            let po = raw_po_f64;
            let before_po = before_po;
            let tc = calculate_tc(po, before_po);
            tc.to_string()
        }
        "testtime" => {
            let date_str = row_data.get("testtime").map(|s| s.as_str()).unwrap_or("");
            if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
                dt.date().to_string()
            } else {
                date_str.to_string()
            }
        }
        _ => {
            // 默认直接取值
            row_data
                .get(mapped_key)
                .unwrap_or(&"".to_string())
                .to_string()
        }
    };

    (raw_value, mapped_key.to_string())
}
// 计算 row_value 的独立函数
// fn calculate_row_value(
//     excel_header: String,
//     row_data: &HashMap<String, String>,
//     config: Arc<TemplateConfig>,
//     // column_mapping: Arc<HashMap<String, String>>,
// ) -> (String, String) {
//     // 优先处理 column_name_mapping
//     // if let Some(col_mapping) = column_mapping.get(&header) {
//     //     if !col_mapping.is_empty() {
//     //         if col_mapping != "none" {
//     //             header = col_mapping.to_string();
//     //         }
//     //     }
//     // }
//     let header = config
//         .infos
//         .get(&excel_header)
//         .map_or(excel_header.clone(), |info| {
//             if info.data_type == "数据" {
//                 info.data_select.clone()
//             } else {
//                 excel_header.clone()
//             }
//         });
//     let fixed_text = config
//         .infos
//         .get(&excel_header)
//         .map_or("".to_string(), |info| {
//             if info.data_type == "固定文本" {
//                 info.fixed_content.clone()
//             } else {
//                 "".to_string()
//             }
//         });
//     if fixed_text.is_empty() {
//         let normalized_header = header;
//         if normalized_header == "beforeTC-Ith(mA)" {
//             let before_ith = before_ith_num(
//                 row_data
//                     .get("ith")
//                     .unwrap_or(&"".to_string())
//                     .parse::<f64>()
//                     .unwrap_or(0.0),
//             );
//             let b_ith = format!("{:.2}", before_ith);
//             (b_ith, normalized_header.clone())
//         } else if normalized_header == "beforeTC-Im(uA)" {
//             let before_im = before_im_num(
//                 row_data
//                     .get("im")
//                     .unwrap_or(&"".to_string())
//                     .parse::<f64>()
//                     .unwrap_or(0.0),
//             );
//             let b_im = format!("{:.2}", before_im);
//             (b_im, normalized_header.clone())
//         } else if normalized_header == "beforeTC-Po(mW)" {
//             let po = before_po(
//                 row_data
//                     .get("po")
//                     .unwrap_or(&"".to_string())
//                     .parse::<f64>()
//                     .unwrap_or(0.0),
//             );
//             let before_po = format!("{:.2}", po);
//             (before_po, normalized_header.clone())
//         } else if normalized_header == "po" {
//             let po = row_data
//                 .get("po")
//                 .unwrap_or(&"".to_string())
//                 .parse::<f32>()
//                 .unwrap();
//             (po.to_string(), normalized_header.clone())
//         } else if normalized_header == "beforeTC-Vf(V)" {
//             let before_vf = before_vf_num(
//                 row_data
//                     .get("vf")
//                     .unwrap_or(&"".to_string())
//                     .parse::<f64>()
//                     .unwrap_or(0.0),
//             );
//             let b_vf = format!("{:.2}", before_vf);
//             (b_vf, normalized_header.clone())
//         } else if normalized_header == "beforeTC-SE(W/A)" {
//             let po = before_po(
//                 row_data
//                     .get("po")
//                     .unwrap_or(&"".to_string())
//                     .parse::<f64>()
//                     .unwrap_or(0.0),
//             );
//             let po = po / 1000.0;
//             let before_se = po / 20.0;
//             let b_se = format!("{:.3}", before_se);
//             (b_se, normalized_header.clone())
//         } else if normalized_header == "deltaP（dB）" {
//             let po = row_data
//                 .get("po")
//                 .unwrap_or(&"".to_string())
//                 .parse::<f64>()
//                 .unwrap_or(0.0)
//                 / 1000.0;
//             let before_po = before_po(po);
//             let tc = calculate_tc(po, before_po).to_string();
//             (tc, normalized_header.clone())
//         } else if normalized_header == "testtime" {
//             let date = row_data
//                 .get("testtime")
//                 .unwrap_or(&"".to_string())
//                 .to_string();
//             let datetime = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();

//             let date = datetime.date().to_string();
//             (date, normalized_header.clone())
//         } else {
//             (
//                 row_data
//                     .get(&normalized_header)
//                     .unwrap_or(&"".to_string())
//                     .to_string(),
//                 normalized_header.clone(),
//             )
//         }
//     } else {
//         (fixed_text, excel_header.clone())
//     }
// }

fn apply_cell_style(style: &mut Style) {
    style
        .get_borders_mut()
        .get_bottom_mut()
        .set_border_style(Border::BORDER_THIN);
    style
        .get_borders_mut()
        .get_top_mut()
        .set_border_style(Border::BORDER_THIN);
    style
        .get_borders_mut()
        .get_left_mut()
        .set_border_style(Border::BORDER_THIN);
    style
        .get_borders_mut()
        .get_right_mut()
        .set_border_style(Border::BORDER_THIN);

    style
        .get_alignment_mut()
        .set_horizontal(HorizontalAlignmentValues::Center);
    style
        .get_alignment_mut()
        .set_vertical(VerticalAlignmentValues::Center);

    let mut font = Font::default();
    font.set_size(12.0);
    style.set_font(font);
}
