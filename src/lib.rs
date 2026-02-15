pub use makepad_widgets::*;
pub mod app;
pub mod configs;
pub mod export;
// pub mod settings;
// pub mod shared;
pub mod store;
pub mod structs;
pub mod utils;

pub mod widgets;
// pub mod querys;
pub mod app_ui;
pub mod login_view;
// pub mod box_band;
// pub mod data_import;
pub fn script_mod(vm: &mut ScriptVm) {
    crate::login_view::script_mod(vm);
    crate::widgets::script_mod(vm);
    crate::export::script_mod(vm);
}
