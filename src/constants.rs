//! 应用程序常量定义
//! 统一管理所有魔法值、颜色和默认配置

/// UI 尺寸常量
pub mod dimensions {
    pub const BTN_BORDER_RADIUS: f64 = 5.0;
    pub const BTN_BORDER_SIZE: f64 = 1.0;
    pub const BTN_HEIGHT: f64 = 40.0;
    pub const INPUT_BORDER_RADIUS: f64 = 5.0;
    pub const INPUT_BORDER_SIZE: f64 = 1.0;
    pub const INPUT_HEIGHT: f64 = 40.0;
    pub const CARD_BORDER_RADIUS: f64 = 8.5;
    pub const SHADOW_RADIUS: f64 = 18.0;
}

/// 颜色常量 (使用十六进制字符串，避免依赖 makepad_widgets)
pub mod colors {
    pub const BTN_PRIMARY: &str = "#00FFFF";
    pub const BTN_SECONDARY: &str = "#FF7F50";
    pub const BTN_DANGER: &str = "#FF4500";
    pub const BTN_SUCCESS: &str = "#AFEEEE";
    pub const BTN_HOVER: &str = "#FFB6C1";
    pub const BTN_DISABLED: &str = "#A9A9A9";

    pub const TEXT_PRIMARY: &str = "#000000";
    pub const TEXT_SECONDARY: &str = "#808080";

    pub const BG_PRIMARY: &str = "#F3F3F3";
    pub const BG_SECONDARY: &str = "#F2F4F7";

    pub const SHADOW_COLOR: &str = "#00000033";
    pub const BORDER_COLOR: &str = "#333333";
    pub const BORDER_COLOR_HOVER: &str = "#555555";
}

/// 消息文本常量
pub mod messages {
    pub const DB_CONNECT_SUCCESS: &str = "数据库连接成功";
    pub const DB_CONNECT_FAILED: &str = "数据库连接失败";
    pub const DB_NOT_CONNECTED: &str = "数据库未连接";
    pub const QUERYING: &str = "查询中";
    pub const QUERY_SUCCESS: &str = "查询成功";
    pub const QUERY_FAILED: &str = "查询失败";
    pub const EXPORT_COMPLETE: &str = "数据导出完成";
    pub const LOGIN_SUCCESS: &str = "登录成功，解锁设置页面";
    pub const DATA_IMPORT_SUCCESS: &str = "数据写入成功";
    pub const BAND_SUCCESS: &str = "绑定成功";
    pub const UNBAND_SUCCESS: &str = "解绑成功";
    pub const CARTON_QUERY: &str = "箱号查询";
    pub const BATCH_QUERY: &str = "批量查询";
}

/// 时间常量
pub mod time {
    pub const POPUP_DURATION_SHORT: f64 = 2.5;
    pub const POPUP_DURATION_LONG: f64 = 5.0;
    pub const RETRY_DELAY_MS: u64 = 100;
    pub const RETRY_MAX_DELAY_MS: u64 = 5000;
}

/// 数据库常量
pub mod db {
    pub const DEFAULT_POOL_SIZE: u32 = 5;
    pub const DEFAULT_PORT: u16 = 1433;
    pub const DEFAULT_DATABASE: &str = "BOSAautotestDB";
    pub const TABLE_PREFIX: &str = "MAC_";
}

/// 文件操作常量
pub mod file {
    pub const DEFAULT_PN_LENGTH: usize = 8;
}

/// 窗口尺寸
pub mod window {
    pub const DEFAULT_WIDTH: f64 = 1600.0;
    pub const DEFAULT_HEIGHT: f64 = 900.0;
}

/// 分页常量
pub mod pagination {
    pub const DEFAULT_PAGE_SIZE: usize = 20;
    pub const MAX_PAGE_SIZE: usize = 100;
}
