use makepad_widgets::ScriptVm;
// pub mod clean_input;
pub mod dialog;
// pub mod popup_list;
// pub mod progress;
pub mod widget;

pub fn script_mod(vm: &mut ScriptVm) {
    widget::script_mod(vm);
    dialog::script_mod(vm);
    // popup_list::script_mod(vm);
    // clean_input::script_mod(vm);
    // progress::script_mod(vm);
}
