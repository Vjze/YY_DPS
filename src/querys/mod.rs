use makepad_widgets::Cx;
pub mod querys_view;
pub mod works;
pub mod widgets;
pub fn live_design(cx: &mut Cx) {
    querys_view::live_design(cx);
    // works::live_design(cx);
    widgets::live_design(cx);
}
