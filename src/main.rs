// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 加载环境变量（如果.env文件存在）
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::DEBUG)
        // display source code file paths
        .with_file(true)
        // display source code line numbers
        .with_line_number(true)
        // disable targets
        .with_target(false)
        // sets this to be the default, global subscriber for this application.
        .init();
    yy_dps::app::app_main();
}
