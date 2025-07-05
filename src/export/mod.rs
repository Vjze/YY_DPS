use makepad_widgets::Cx;
pub mod export_view;
pub mod works;
pub fn live_design(cx: &mut Cx) {
    export_view::live_design(cx);
}