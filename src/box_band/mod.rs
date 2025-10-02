use makepad_widgets::Cx;
pub mod work;
pub mod box_band_view;
pub mod band_table;
pub fn live_design(cx: &mut Cx) {
    box_band_view::live_design(cx);
    band_table::live_design(cx);
}
