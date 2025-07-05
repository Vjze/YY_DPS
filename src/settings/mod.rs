// pub mod add_provider_modal;
// pub mod delete_server_modal;
// pub mod moly_server_screen;
pub mod type_setting_view;
pub mod providers;
pub mod providers_screen;
pub mod template_setting_view;
pub mod map_setting_view;
// pub mod sync_modal;
use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    providers_screen::live_design(cx);
    // moly_server_screen::live_design(cx);
    // delete_server_modal::live_design(cx);
    type_setting_view::live_design(cx);
    providers::live_design(cx);
    template_setting_view::live_design(cx);
    map_setting_view::live_design(cx);
    // add_provider_modal::live_design(cx);
    // sync_modal::live_design(cx);
}
