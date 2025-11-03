use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook};
use sqlx_oldapi::types::chrono::Local;
use std::collections::HashMap;

use crate::utils::error::MyError;

pub async fn sn_export(datas: Vec<HashMap<String, String>>) -> anyhow::Result<String, MyError> {
    if datas.is_empty() {
        return Err(MyError::Zdyknown("警告,没有数据.".to_string()));
    }

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(0, 16).unwrap();

    let blue_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_font_size(10)
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0x00B0F0))
        .set_bold();
    let str_format = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);

    // 1. 确定所有列头并设置列宽
    let mut headers: Vec<String> = Vec::new();
    // 从第一个 HashMap 中获取所有键作为列头
    // 假设所有 HashMap 的键集相同，或者至少第一个包含所有可能的键
    if let Some(first_data) = datas.first() {
        for (key, _) in first_data.iter() {
            headers.push(key.clone());
        }
    } else {
        // 如果 datas 为空，提前返回
        return Err(MyError::Zdyknown("警告,没有数据.".to_string()));
    }

    // 写入列头并设置列宽
    for (y, key) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, y as u16, key, &blue_format)?;

        match key.as_str() {
            "sn" | "testtime" | "carton_no" | "carton_packtime" | "box_no" | "pack_packtime" => {
                worksheet.set_column_width_pixels(y as u16, 160)?;
            }
            _ => {
                worksheet.set_column_width_pixels(y as u16, 80)?;
            }
        }
    }

    // 2. 遍历 datas 一次，写入所有数据行
    for (n, data) in datas.iter().enumerate() {
        let x: u32 = (n + 1).try_into().unwrap(); // 从第1行开始写数据

        for (y, header) in headers.iter().enumerate() {
            if let Some(value) = data.get(header) {
                worksheet.write_string_with_format(x, y as u16, value, &str_format)?;
            } else {
                // 如果某个 HashMap 缺少某个键，可以写入空字符串或者其他默认值
                worksheet.write_string_with_format(x, y as u16, "", &str_format)?;
            }
        }
    }

    let datetime = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let file_name = "sn_export".to_string();
    let path = format!("{}-{}.xlsx", file_name, datetime);

    match workbook.save(path.clone()) {
        Ok(_) => Ok(path),
        Err(_e) => Err(MyError::Zdyknown(
            "数据导出错误，请注意软件目录下是否存在同名文件以及是否有读写权限!!!".to_string(),
        )),
    }
}
