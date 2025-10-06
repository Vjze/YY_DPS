use makepad_widgets::Cx;
pub mod data_import_db;
pub mod work;
pub mod impot_table;
pub mod import_row;
pub fn live_design(cx: &mut Cx) {
    data_import_db::live_design(cx);
    impot_table::live_design(cx);
    import_row::live_design(cx);
}
