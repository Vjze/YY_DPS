use makepad_widgets::Cx;
pub mod band_table;
pub mod box_band_view;
pub mod unband_dialog;
pub mod work;
pub fn live_design(cx: &mut Cx) {
    box_band_view::live_design(cx);
    band_table::live_design(cx);
    unband_dialog::live_design(cx);
}
