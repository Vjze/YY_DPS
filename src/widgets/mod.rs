use makepad_widgets::Cx;
pub mod dialog;
pub mod widget;
pub mod popup_list;

pub fn live_design(cx: &mut Cx) {
    widget::live_design(cx);
    dialog::live_design(cx);
    popup_list::live_design(cx);
}
