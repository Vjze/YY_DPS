use calamine::{Data, Reader, Xlsx, open_workbook};
use tracing::info;

#[derive(Debug, Default, Clone)]
pub struct ImportDBDatas {
    pub sn: String,
    pub condition_unit: String,
    pub ith: String,
    pub se: String,
    pub po: String,
    pub vf: String,
    pub im: String,
    pub sen: String,
    pub icc: String,
}
fn get_cell_value_as_string(cell: &Data) -> String {
    match cell {
        Data::Int(i) => i.to_string(),
        Data::Float(f) => f.to_string(),
        Data::String(s) => s.clone(),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        _ => String::new(), // 对于 Error 和 Empty 类型，返回空字符串
    }
}
fn get_po_value(cell: &Data) -> String {
    match cell {
        // 如果是浮点数类型，执行乘法运算
        Data::Float(f) => {
            let multiplied_value = f * 1000.0;
            format!("{:.2}", multiplied_value)
        }
        // 如果是整数或其他类型，使用通用函数转换
        _ => get_cell_value_as_string(cell),
    }
}
pub async fn extract_data(file_path: &str) -> anyhow::Result<Vec<ImportDBDatas>> {
    info!("开始提取数据 from file: {}", file_path);
    let mut excel: Xlsx<_> = open_workbook(file_path)?;
    info!("Excel 文件打开成功");
    let mut data_vec: Vec<ImportDBDatas> = Vec::new();
    let sheets = excel.worksheets();
    for (_name, result) in sheets {
        // 遍历每一行，跳过表头
        for row in result.rows().skip(4) {
            // 确保每一行都有足够多的列
            if row.len() >= 9 {
                let data_instance = ImportDBDatas {
                    sn: get_cell_value_as_string(row.get(0).unwrap_or(&calamine::Data::Empty)),
                    condition_unit: get_cell_value_as_string(
                        row.get(1).unwrap_or(&calamine::Data::Empty),
                    ),
                    ith: get_cell_value_as_string(row.get(2).unwrap_or(&calamine::Data::Empty)),
                    se: get_cell_value_as_string(row.get(3).unwrap_or(&calamine::Data::Empty)),
                    po: get_po_value(row.get(4).unwrap_or(&calamine::Data::Empty)),
                    vf: get_cell_value_as_string(row.get(5).unwrap_or(&calamine::Data::Empty)),
                    im: get_cell_value_as_string(row.get(6).unwrap_or(&calamine::Data::Empty)),
                    sen: get_cell_value_as_string(row.get(7).unwrap_or(&calamine::Data::Empty)),
                    icc: get_cell_value_as_string(row.get(8).unwrap_or(&calamine::Data::Empty)),
                };
                data_vec.push(data_instance);
            }
        }
    }
    info!("数据提取完成，一共 {}条", data_vec.len());
    Ok(data_vec)
}