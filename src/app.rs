use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    store::Store,
    utils::error::{LoginResult, MyError, MyTip}, widgets::dialog::{ErrorDialogSetWidgetRefExt, ErrorDialogWidgetRefExt, ErrprModalAction},
    // widgets::{
    //     dialog::ErrprModalAction,
    //     // popup_list::{PopupItem, PopupKind, enqueue_popup_notification, set_global_popup_list},
    // },
};

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_CHAT = crate_resource("self://resources/icons/chat.svg")
    let ICON_LOCAL = crate_resource("self://resources/icons/local.svg")
    let ICON_BAND_VIEW = crate_resource("self://resources/icons/cloud.svg")
    let ICON_CLOUD = crate_resource("self://resources/icons/cloud.svg")
    let ICON_MOLYSERVER = crate_resource("self://resources/images/logo.png")

    let ApplicationPages = RoundedShadowView {
        width: Fill, height: Fill
        // margin: {top: 12, right: 12, bottom: 12}
        padding: 3.
        flow: Overlay
        // show_bg: true
        draw_bg +: {
            color: instance(#f9f9f9),
            border_radius: uniform(8.5),
            shadow_color: instance(#0003),
            shadow_radius: uniform(18.0),
            shadow_offset: vec2(0.0,-1.5)
        }
        export_frame := ExportScreen {visible: true}
        // querys_frame = <QueryScreen> {visible: false}
        // box_band_frame = <BoxBandView> {visible: false}
        // data_import_db_frame = <DataImportDb> {visible: false}
        // providers_frame = <ProvidersScreen> {visible: false}
        //
    }
    let SidebarMenu = RoundedView {
        width: 90, height: Fill,
        flow: Down, spacing: 15.0,
        padding: Inset{ top: 40, bottom: 10, left: 10, right: 10 },

        align: Align{x: 0.5, y: 0.0},
        // show_bg: true,
        draw_bg +: {
            color: #f2f2f2,
            border_radius: 0.0,
        }
        View {
            width: Fit, height: Fit
            padding: Inset{bottom:20}
            Image {
                width: 50, height: 50,
                src: ICON_MOLYSERVER,
            }
        }
        seprator := View {
            width: Fill, height: 1.6,
            margin: Inset{left: 15, right: 15, bottom: 10}
            // show_bg: true
            draw_bg +: {
                color: #dadada,
            }
        }

        export_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            // animator: active : {default: @on}
            text: "查询导出",
            draw_icon +: {
                svg: (ICON_CHAT),
            }
        }
        sn_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "数据查询",
            draw_icon +: {
                svg: (ICON_LOCAL),
            }
        }
        box_band_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "盒号绑定",
            draw_icon +: {
                svg: (ICON_BAND_VIEW),
            }
        }
        data_import_db_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "外协数据导入",
            draw_icon +: {
                svg: (ICON_BAND_VIEW),
            }
        }
        Filler{}
        View {
            align: Align{x: 0.5 y: 1.0}
            // visible: false
            providers_tab := SidebarMenuButton {
                text: "设置",
                draw_icon +: {
                    svg: (ICON_CLOUD),
                }
            }
        }

    }

    // mod.gc.set_static(AppUI)

    startup() do #(App::script_component(vm)){
        ui: Root {
            main_window := Window {
                caption_bar +: {
                    margin: Inset{top: 2 left: -190}
                    caption_label +: {
                        label +: {
                            text: "DPS"
                            draw_text +: {
                                color: #000000
                            }
                        }
                    }
                    windows_buttons +: {
                        min +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                        max +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                        close +: { draw_bg +: {color: #0, color_hover: #E81123, color_down: #FF0015} }
                    }
                }
                pass.clear_color: vec4(1.0 1.0 1.0 1.0)
                window.inner_size: vec2(1600 900)

                show_bg: true
                draw_bg +: {
                    color: #F3F3F3
                    // pixel: fn() {
                    //     return theme.color_bg_app
                    // }
                }

                body +: {
                    flow: Overlay
                    width: Fill,
                    height: Fill,
                    padding: 0

                    root := View {
                        width: Fill,
                        height: Fill,
                        // show_bg: true,
                        draw_bg +: {
                            color: #f2f2f2,
                        }


                    root_adaptive_view := View {
                            visible: false

                                sidebar_menu := SidebarMenu {}
                                application_pages := ApplicationPages {}
                        }
                        login_view := View {
                            visible: true
                        login_screen := mod.widgets.LoginScreen {}

                        }
                    }
                    // <PopupList> {}
                    dialog_ui := Modal {
                        can_dismiss: false
                        content +: {
                            dialog_ui_inner := ErrorDialog {
                            }
                        }
                    }
                }
                
            }
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
        // crate::app_ui::script_mod(vm);
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

        if self.ui.radio_button(cx, ids!(export_tab)).clicked(actions) {
            self.navigate_to(cx, ids!(application_pages.export_frame));
        };
        if self
            .ui
            .radio_button(cx, ids!(data_import_db_tab))
            .clicked(actions)
        {
            self.navigate_to(cx, ids!(application_pages.data_import_db_frame));
        }
        if self
            .ui
            .radio_button(cx, ids!(box_band_tab))
            .clicked(actions)
        {
            self.navigate_to(cx, ids!(application_pages.box_band_frame));
        }
        if self.ui.radio_button(cx, ids!(sn_tab)).clicked(actions) {
            self.navigate_to(cx, ids!(application_pages.querys_frame));
        }
        if self
            .ui
            .radio_button(cx, ids!(providers_tab))
            .clicked(actions)
        {
            self.navigate_to(cx, ids!(application_pages.providers_frame));
        }
       
        for action in actions {
            if let Some(err) = action.downcast_ref::<MyError>() {
                let content = err.to_string();
                println!("{content}");
                self.ui.error_dialog(cx, ids!(dialog_ui_inner)).set_err_text(cx, content);
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
            if let Some(ErrprModalAction::Close) = action.downcast_ref() {
                self.ui.modal(cx, ids!(dialog_ui)).close(cx);
            }
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
