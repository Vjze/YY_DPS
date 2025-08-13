use makepad_widgets::Cx;
pub mod dialog;
pub mod row;
pub mod table;
pub fn live_design(cx: &mut Cx) {
    row::live_design(cx);
    table::live_design(cx);
}
