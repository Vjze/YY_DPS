use makepad_widgets::Cx;
pub mod styles;
pub mod widgets;

pub fn script_mod(vm: &mut ScriptVm) {
    styles::live_design(cx);
    widgets::live_design(cx);
}
