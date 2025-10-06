use std::path::PathBuf;
use crate::utils::error::MyError;

pub async fn select_file() -> anyhow::Result<PathBuf> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("Excel", &["xlsx", "xls"])
        .set_title("选择Excel文件")
        .pick_file().await.ok_or(MyError::DialogClosed)?;
    let path = file.path().to_path_buf();
    Ok(path)
}