pub mod add_template_modal;
pub mod delete_modal;
// pub mod moly_server_screen;
pub mod decimal_grid;
pub mod fixed_content_grid;
pub mod map_grid;
pub mod map_setting_view;
pub mod providers;
pub mod providers_screen;
pub mod tabel_grid;
pub mod template_setting_view;
pub mod type_setting_view;
use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    providers_screen::live_design(cx);
    // moly_server_screen::live_design(cx);
    delete_modal::live_design(cx);
    type_setting_view::live_design(cx);
    providers::live_design(cx);
    template_setting_view::live_design(cx);
    decimal_grid::live_design(cx);
    fixed_content_grid::live_design(cx);
    tabel_grid::live_design(cx);
    map_setting_view::live_design(cx);
    map_grid::live_design(cx);
    add_template_modal::live_design(cx);
    // sync_modal::live_design(cx);
}
