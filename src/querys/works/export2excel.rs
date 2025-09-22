use std::{collections::HashMap, sync::Arc};

use chrono::{Local, NaiveDateTime};
// use hashbrown::HashMap;
use rayon::prelude::*;
use regex::Regex;
use umya_spreadsheet::{
    Border, Font, HorizontalAlignmentValues, Style, VerticalAlignmentValues, reader::xlsx::read,
    writer::xlsx::write,
};

use super::excel_utils::{before_im_num, before_ith_num, before_po, before_vf_num, calculate_tc};
use crate::{
    configs::{
        column_map_config::get_template_map_config,
        decimal_config::{DecimalConfig, get_decimal_config_value},
        type_config::get_type_infos,
    },
    utils::error::{MyError, MyTip},
};

// 单元格更新结构
#[derive(Debug)]
struct CellUpdate {
    col: u32,
    row: u32,
    value: String,
}

pub async fn write_to_excel(
    type_name: String,
    datas: Vec<HashMap<String, String>>,
) -> Result<MyTip, MyError> {
    let template_names: Vec<String> = get_type_infos(type_name.clone()).await?.0;
    let datas = Arc::new(datas.clone());
    let template_path = Arc::new(String::from(r"\\192.168.10.142\Excel_Templates\"));

    // 使用 tokio::spawn 创建异步任务，处理每个模板
    let tasks: Vec<_> = template_names
        .into_iter()
        .map(|template_name| {
            let datas = datas.clone();
            let template_path = template_path.clone();
            tokio::spawn(async move {
                let result = async {
                    let decimal_config = get_decimal_config_value(template_name.clone())
                        .await
                        .unwrap();
                    let template_path = format!("{}{}.xlsx", template_path, template_name);
                    let mut book = read(&template_path)
                        .map_err(|_| format!("无法读取模板文件: {}", template_path))?;
                    let sheet = book
                        .get_sheet_by_name_mut("Sheet1")
                        .ok_or("找不到 Sheet1".to_string())?;
                    let re = Regex::new(r"^[A-Za-z0-9_]+").unwrap();
                    // 提取表头
                    let mut headers = Vec::new();

                    // 获取工作表中最高行号
                    let highest_row = sheet.get_highest_row();
                    if highest_row == 0 {
                        // 如果工作表为空，直接返回空表头
                        headers = Default::default();
                    }
                    // 获取 rows 字段，确保是整数
                    let rows = decimal_config.strings.get("rows").unwrap();
                    let header_rows_to_scan = if rows == "1" {
                        vec![highest_row]
                    } else {
                        vec![highest_row - 1, highest_row]
                    };
                    // 处理 table_infos
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
                        } else {
                            cell.set_value(value.to_string().trim_matches('"').to_string());
                        }
                    }
                    // 从第1列开始，按列遍历
                    for col_index in 1.. {
                        let mut combined_header_for_col = String::new();
                        let mut found_content_in_col = false; // 标志位，用于判断这一列在表头范围内是否有内容

                        // 在确定的表头行范围内，按行遍历并拼接
                        for &row_index in header_rows_to_scan.iter() {
                            if let Some(cell) = sheet.get_cell((col_index, row_index)) {
                                if !cell.get_value().trim().is_empty() {
                                    combined_header_for_col.push_str(&cell.get_value().to_string());
                                    found_content_in_col = true; // 这一列有内容
                                }
                            }
                        }

                        // 如果这一列在表头行范围内完全为空，则认为表头已结束
                        if !found_content_in_col {
                            // 在遇到第一个完全空列时，停止遍历
                            break;
                        }

                        // 将拼接后的表头添加到 headers 列表中
                        headers.push(combined_header_for_col);
                    }
                    let headers = headers
                        .iter()
                        .map(|h| {
                            if let Some(captures) = re.captures(&h) {
                                let extracted = captures.get(0).map(|m| m.as_str()).unwrap_or("");
                                extracted.to_string()
                            } else {
                                h.to_string()
                            }
                        })
                        .collect::<Vec<_>>();
                    let last_row = sheet.get_highest_row();
                    let decimal_config = Arc::new(decimal_config);
                    // 异步获取 column_name_mapping
                    let column_name_mapping = Arc::new(
                        get_template_map_config(template_name.clone())
                            .await
                            .map_err(|_| format!("无法获取列映射: {}", template_name))?,
                    );
                    // 并行生成单元格更新（保留 rayon）
                    let cell_updates: Vec<CellUpdate> = datas
                        .par_iter()
                        .enumerate()
                        .flat_map(|(row_idx, row_data)| {
                            let row = last_row + row_idx as u32 + 1;
                            headers
                                .iter()
                                .enumerate()
                                .map({
                                    let config = decimal_config.clone();
                                    let column_mapping = column_name_mapping.clone();
                                    // let deviceinfo_id = deviceinfo_id.clone();
                                    move |(col_idx, header)| {
                                        let col = (col_idx + 1) as u32;
                                        let row_value_header = calculate_row_value(
                                            header.to_string(),
                                            row_data,
                                            column_mapping.clone(),
                                            config.clone(),
                                        );
                                        let new_header = row_value_header.1;
                                        let row_value =
                                            if new_header == "deviceinf" || new_header == "no" {
                                                let row = row_idx + 1;
                                                row.to_string()
                                            } else {
                                                row_value_header.0
                                            };
                                        let (value, _decimals) =
                                            format_with_decimals(&row_value, &config, &new_header);

                                        CellUpdate { col, row, value }
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect();

                    // 构造样式一次性复用
                    let mut style = Style::default();
                    apply_cell_style(&mut style);
                    let style = Arc::new(style);

                    // 主线程写入
                    for update in cell_updates {
                        let cell = sheet.get_cell_mut((update.col, update.row));
                        cell.set_value_string(update.value.clone());

                        sheet
                            .get_style_mut((update.col, update.row))
                            .clone_from(&style);
                    }

                    // 保存
                    let date = Local::now().format("%Y-%m-%d").to_string();
                    let output_path = format!("{}-{}.xlsx", template_name, date);
                    write(&book, &output_path)
                        .map_err(|_| format!("无法写入文件: {}", output_path))?;

                    Ok::<(), String>(())
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
        Ok(MyTip::ExportDone(type_name.clone()))
    } else {
        let error = errors.join(";");
        Err(MyError::WriteToExcelErr(type_name.clone(), error))
    }
}

fn format_with_decimals(
    val: &str,
    config: &Arc<DecimalConfig>,
    header: &str,
) -> (String, Option<usize>) {
    // 先检查 numbers 是否包含 header（小数位数配置）
    if let Some(decimals) = config.numbers.get(header) {
        if let Ok(num) = val.parse::<f64>() {
            let decimals = decimals.clone() as usize; // 将 f64 转换为 usize
            let formatted_string = format!("{:.decimals$}", num, decimals = decimals);
            return (formatted_string, Some(decimals));
        }
    }

    // 再检查 strings 是否包含 header（字符串或单位）
    if let Some(s) = config.strings.get(header) {
        if !s.is_empty() {
            if s == "mw" || s == "uw" {
                (val.to_string(), None)
            } else {
                (s.to_string(), None) // 直接使用配置中的字符串
            }
        } else {
            (val.to_string(), None)
        }
    } else {
        // 如果 numbers 和 strings 都没有匹配，直接返回原值
        (val.to_string(), None)
    }
}

// 计算 row_value 的独立函数
fn calculate_row_value(
    mut header: String,
    row_data: &HashMap<String, String>,
    column_mapping: Arc<HashMap<String, String>>,
    config: Arc<DecimalConfig>, // 修改为接收整个 DecimalConfig
) -> (String, String) {
    // 优先处理 column_name_mapping
    if let Some(col_mapping) = column_mapping.get(&header) {
        if !col_mapping.is_empty() {
            if col_mapping != "none" {
                header = col_mapping.to_string();
            }
        }
    }

    // 规范化表头：转为小写，移除空格和括号
    // let normalized_header = header.clone().to_lowercase();
    // .replace(" ", "")
    // .replace("(", "")
    // .replace(")", "");
    let normalized_header = header;
    // 模糊匹配逻辑，统一处理 beforeTC 和非 beforeTC 情况
    // if normalized_header.contains("ith") {
    if normalized_header == "beforeTC-Ith(mA)" {
        let before_ith = before_ith_num(
            row_data
                .get("ith")
                .unwrap_or(&"".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
        );
        let b_ith = format!("{:.2}", before_ith);
        (b_ith, normalized_header.clone())
    } else if normalized_header == "beforeTC-Im(uA)" {
        let before_im = before_im_num(
            row_data
                .get("im")
                .unwrap_or(&"".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
        );
        let b_im = format!("{:.2}", before_im);
        (b_im, normalized_header.clone())
    } else if normalized_header == "beforeTC-Po(mW)" {
        if let Some(unit) = config.strings.get("unit") {
            let unit = unit.to_string().replace("\\\"", "").replace("\"", "");
            if unit.to_string().to_lowercase() == "uw" {
                let po = before_po(
                    row_data
                        .get("po")
                        .unwrap_or(&"".to_string())
                        .parse::<f64>()
                        .unwrap_or(0.0),
                );
                let before_po = format!("{:.2}", po);
                (before_po, normalized_header.clone())
            } else {
                let po = before_po(
                    row_data
                        .get("po")
                        .unwrap_or(&"".to_string())
                        .parse::<f64>()
                        .unwrap_or(0.0),
                );
                let po = po / 1000.0;
                let before_po = format!("{:.2}", po);
                (before_po, normalized_header.clone())
            }
        } else {
            let po = before_po(
                row_data
                    .get("po")
                    .unwrap_or(&"".to_string())
                    .parse::<f64>()
                    .unwrap_or(0.0),
            );
            let before_po = format!("{:.2}", po);
            (before_po, normalized_header.clone())
        }
    } else if normalized_header == "po" {
        if let Some(unit) = config.strings.get("unit") {
            let unit = unit.to_string().replace("\\\"", "").replace("\"", "");
            if unit.to_string().to_lowercase() == "uw" {
                let po = row_data
                    .get("po")
                    .unwrap_or(&"".to_string())
                    .parse::<f32>()
                    .unwrap();
                (po.to_string(), normalized_header.clone())
            } else {
                let po = row_data
                    .get("po")
                    .unwrap_or(&"".to_string())
                    .parse::<f32>()
                    .unwrap();
                let po = po / 1000.0;
                (po.to_string(), normalized_header.clone())
            }
        } else {
            let po = row_data
                .get("po")
                .unwrap_or(&"".to_string())
                .parse::<f32>()
                .unwrap();
            let po = po / 1000.0;
            (
                // row_data.get("po").unwrap_or(&"".to_string()).to_string(),
                po.to_string(),
                normalized_header.clone(),
            )
        }
    } else if normalized_header == "beforeTC-Vf(V)" {
        let before_vf = before_vf_num(
            row_data
                .get("vf")
                .unwrap_or(&"".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
        );
        let b_vf = format!("{:.2}", before_vf);
        (b_vf, normalized_header.clone())
    } else if normalized_header == "beforeTC-SE(W/A)" {
        let po = before_po(
            row_data
                .get("po")
                .unwrap_or(&"".to_string())
                .parse::<f64>()
                .unwrap_or(0.0),
        );
        let po = po / 1000.0;
        let before_se = po / 20.0;
        let b_se = format!("{:.3}", before_se);
        (b_se, normalized_header.clone())
    } else if normalized_header == "deltaP（dB）" {
        let po = row_data
            .get("po")
            .unwrap_or(&"".to_string())
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1000.0;
        let before_po = before_po(po);
        let tc = calculate_tc(po, before_po).to_string();
        // let delta = format!("{:.3}", tc);

        (tc, normalized_header.clone())
    } else if normalized_header == "testtime" {
        let date = row_data
            .get("testtime")
            .unwrap_or(&"".to_string())
            .to_string();
        let datetime = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();

        let date = datetime.date().to_string();
        (date, normalized_header.clone())
    } else if normalized_header == "time" {
        let date = row_data
            .get("carton_packtime")
            .unwrap_or(&"".to_string())
            .to_string();
        (date, normalized_header.clone())
    } else {
        // 在没有特殊匹配规则的情况下，使用规范化后的 header 来获取 row_data 中的值
        (
            row_data
                .get(&normalized_header)
                .unwrap_or(&"".to_string())
                .to_string(),
            normalized_header.clone(),
        )
    }
}

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
