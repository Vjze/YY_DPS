use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("数据库连接失败: {0}")]
    DbConnectionError(#[from] tiberius::error::Error),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("并发查询错误: {0}")]
    TokioError(#[from] tokio::task::JoinError),
    #[error("表格写入异常: {0}")]
    Write2ExcelErr(#[from] rust_xlsxwriter::XlsxError),
    #[error("表格写入异常: {0}")]
    SqlError(#[from] bb8_tiberius::Error),
    #[error("没有找到任何型号!!!")]
    UnLoadedTypes,
    #[error("没有找到任何模板!!!")]
    UnLoadedTemplates,
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
    WriteErr(String, String),
    #[error("型号 {0}导出错误!!错误原因:{1}")]
    WriteToExcelErr(String, String),
    #[error("箱号不能为空!!!")]
    CartonNoEmpty,
    #[error("数据查询错误!!")]
    QueryErr,
    #[error("{0}")]
    Zdyknown(String),
    #[error("所有查询条件不能为空!!!/n最少输入一个查询条件")]
    AllNone,
    #[error("{0}")]
    TaskJoinError(String),
    #[error("{0}")]
    LoginError(String),
    #[error("选择文件对话框被关闭")]
    DialogClosed,
    #[error("盒号解绑错误：{0}")]
    UnbindBoxErr(String),

    // 数据查询相关错误
    #[error("箱号 '{0}' 没有找到数据")]
    CartonNoData(String),
    #[error("SN查询错误: {0}")]
    SnQueryError(String),
    #[error("批次号查询错误: {0}")]
    BatchQueryError(String),

    // 配置相关错误
    #[error("型号名称不能为空")]
    TypeNameEmpty,
    #[error("模板名称不能为空")]
    TemplateNameEmpty,
    #[error("文件操作错误: {0}")]
    FileOperationError(String),

    // 数据处理相关错误
    #[error("没有数据，无法操作")]
    NoDataAvailable,
    #[error("数据转换错误: {0}")]
    DataConversionError(String),
    #[error("导出数据为空")]
    ExportDataEmpty,

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
    #[error("{0}")]
    Zdyknown(String),
    #[error("未知错误")]
    Unknown,
}
#[derive(Debug)]
pub enum LoginResult {
    Logined,
    FreeLogin,
}
