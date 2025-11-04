use makepad_widgets::Cx;
pub mod styles;
pub mod widgets;

pub fn live_design(cx: &mut Cx) {
    styles::live_design(cx);
    widgets::live_design(cx);
}
