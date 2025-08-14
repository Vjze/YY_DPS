use makepad_widgets::Cx;
pub mod dialog;
pub mod row;
pub mod table;
pub mod widget;
pub mod login_view;
pub fn live_design(cx: &mut Cx) {
    widget::live_design(cx);
    dialog::live_design(cx);
    row::live_design(cx);
    table::live_design(cx);
    login_view::live_design(cx);
}
