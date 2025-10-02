use makepad_widgets::Cx;
pub mod work;
pub mod box_band_view;
pub fn live_design(cx: &mut Cx) {
    box_band_view::live_design(cx);
}
