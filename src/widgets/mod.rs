use makepad_widgets::Cx;
pub mod clean_input;
pub mod dialog;
pub mod popup_list;
pub mod progress;
pub mod widget;

pub fn live_design(cx: &mut Cx) {
    widget::live_design(cx);
    dialog::live_design(cx);
    popup_list::live_design(cx);
    clean_input::live_design(cx);
    progress::live_design(cx);
}
