use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    store::Store,
    utils::error::{LoginResult, MyError, MyTip},
    // widgets::{
    //     dialog::ErrprModalAction,
    //     // popup_list::{PopupItem, PopupKind, enqueue_popup_notification, set_global_popup_list},
    // },
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.gc.set_static(AppUI)

    startup() do #(App::script_component(vm)){
        ui: Root {
            AppUI{}
        }
    }
}

app_main!(App);

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    pub ui: WidgetRef,
    #[rust]
    pub store: Store,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        makepad_widgets::script_mod(vm);
        crate::script_mod(vm);
        crate::app_ui::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);

        // set_global_popup_list(cx, &self.ui);
        let store = &mut self.store;
        let scope = &mut Scope::with_data(store);
        self.ui.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let rt = self.rt.handle().clone();
        self.ui.view(cx, ids!(body)).set_visible(cx, false);
        let _guard = rt.enter();
        let store = rt.block_on(async move { Store::init().await });
        self.store = store;
    }
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigate_to_export = false;
        let mut navigate_to_sn = false;
        let mut navigate_to_box_band = false;
        let mut navigate_to_data_import_db = false;
        let mut navigate_to_providers = false;

        // TODO: Replace this with a proper navigation widget.
        if let Some(selected_tab) = self
            .ui
            .radio_button_set(
                cx,
                ids_array!(
                    sidebar_menu.export_tab,
                    sidebar_menu.sn_tab,
                    sidebar_menu.box_band_tab,
                    sidebar_menu.data_import_db_tab,
                    sidebar_menu.providers_tab,
                ),
            )
            .selected(cx, actions)
        {
            match selected_tab {
                0 => navigate_to_export = true,
                1 => navigate_to_sn = true,
                2 => navigate_to_box_band = true,

                3 => navigate_to_data_import_db = true,
                4 => navigate_to_providers = true,
                _ => {}
            }
        }
        // Handle navigation after processing all actions
        if navigate_to_providers {
            self.navigate_to(cx, ids!(application_pages.providers_frame));
        } else if navigate_to_export {
            self.navigate_to(cx, ids!(application_pages.export_frame));
        } else if navigate_to_box_band {
            self.navigate_to(cx, ids!(application_pages.box_band_frame));
        } else if navigate_to_data_import_db {
            self.navigate_to(cx, ids!(application_pages.data_import_db_frame));
        } else if navigate_to_sn {
            self.navigate_to(cx, ids!(application_pages.querys_frame));
        }
        for action in actions {
            if let Some(err) = action.downcast_ref::<MyError>() {
                let content = err.to_string();
                println!("{content}");
                self.ui
                    .modal(cx, ids!(dialog_ui.dialog_ui_inner))
                    .label(cx, ids!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(cx, ids!(dialog_ui)).open(cx);
            }
            if let Some(tip) = action.downcast_ref::<MyTip>() {
                let content = tip.to_string();
                println!("{content}");
                self.ui
                    .modal(cx, ids!(dialog_ui.dialog_ui_inner))
                    .label(cx, ids!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(cx, ids!(dialog_ui)).open(cx);
            }
            // if let Some(ErrprModalAction::Close) = action.downcast_ref() {
            //     self.ui.modal(cx, ids!(dialog_ui)).close(cx);
            // }
            if let Some(LoginResult::Logined) = action.downcast_ref() {
                let store = self.store.clone();
                // store.logined = true;
                // store.free_login = false;
                let show_login = !store.login_store.logined;
                self.ui
                    .view(cx, ids!(login_view))
                    .set_visible(cx, show_login);
                self.ui
                    .view(cx, ids!(root_adaptive_view))
                    .set_visible(cx, !show_login);
                self.ui.view(cx, ids!(set_btn)).set_visible(cx, true);

                // enqueue_popup_notification(PopupItem {
                //     kind: PopupKind::Success,
                //     auto_dismissal_duration: Some(2.5),
                //     message: "登录成功，解锁设置页面.".to_string(),
                // });
            }
            if let Some(LoginResult::FreeLogin) = action.downcast_ref() {
                let store = self.store.clone();
                // store.logined = true;
                // store.free_login = true;
                let show_login = !store.login_store.logined;
                self.ui
                    .view(cx, ids!(login_view))
                    .set_visible(cx, show_login);
                self.ui
                    .view(cx, ids!(root_adaptive_view))
                    .set_visible(cx, !show_login);
                self.ui
                    .button(cx, ids!(providers_tab))
                    .set_visible(cx, false);
                self.ui.view(cx, ids!(set_btn)).set_visible(cx, false);

                // enqueue_popup_notification(PopupItem {
                //     kind: PopupKind::Warning,
                //     auto_dismissal_duration: Some(2.5),
                //     message: "未使用账号密码登录，隐藏设置页面.".to_string(),
                // });
            }
        }
        cx.redraw_all();
    }
}

impl App {
    fn navigate_to(&mut self, cx: &mut Cx, id: &[LiveId]) {
        let providers_id = ids!(application_pages.providers_frame);
        let export_id = ids!(application_pages.export_frame);
        let box_band_id = ids!(application_pages.box_band_frame);
        let sn_id = ids!(application_pages.querys_frame);
        let data_import_db_id = ids!(application_pages.data_import_db_frame);

        if id != providers_id {
            self.ui.widget(cx, providers_id).set_visible(cx, false);
        }

        if id != export_id {
            self.ui.widget(cx, export_id).set_visible(cx, false);
        }

        if id != sn_id {
            self.ui.widget(cx, sn_id).set_visible(cx, false);
        }

        if id != box_band_id {
            self.ui.widget(cx, box_band_id).set_visible(cx, false);
        }

        if id != data_import_db_id {
            self.ui.widget(cx, data_import_db_id).set_visible(cx, false);
        }
        self.ui.widget(cx, id).set_visible(cx, true);
    }
}
