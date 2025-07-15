use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("数据库连接失败: {0}")]
    DbConnectionError(#[from] tiberius::error::Error),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("没有找到任何型号!!!")]
    UnLoadedTypes,
    #[error("TOML序列化失败!{0}")]
    ReadToTomlErr(#[from] toml::de::Error),
    #[error("TOML反序列化失败!{0}")]
    WriteToTomlErr(#[from] toml::ser::Error),
    #[error("{0}不存在!!")]
    None(String),
    #[error("{0}已存在!!")]
    Haved(String),
    #[error("{0}没有数据!!")]
    NoResult(String),
    #[error("{0}写入错误,{1}!!")]
    WriteErr(String,String),
    #[error("型号 {0}导出错误!!错误原因:{1}")]
    WriteToExcelErr(String,String),
    #[error("未知错误")]
    Unknown,
}
#[derive(Error, Debug)]
pub enum MyTip {
    #[error("{0}删除完成!!")]
    DeleteDone(String),
    #[error("{0}更新完成!!")]
    UpdateDone(String),
    #[error("{0}添加完成!!")]
    AddDone(String),
    #[error("型号 {0}导出完成!!")]
    ExportDone(String),
    #[error("未知错误")]
    Unknown,
}