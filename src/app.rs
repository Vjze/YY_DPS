
use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    store::Store,
    utils::error::{LoginResult, MyError, MyTip},
    widgets::{
        dialog::ErrprModalAction,
        popup_list::{PopupItem, PopupKind, enqueue_popup_notification, set_global_popup_list},
    },
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::widgets::SidebarMenuButton;
    use crate::querys::querys_view::QueryScreen;
    use crate::export::export_view::ExportScreen;
    use crate::settings::providers_screen::ProvidersScreen;
    use crate::widgets::dialog::*;
    use crate::login_view::LoginScreen;
    use crate::box_band::box_band_view::BoxBandView;
    use crate::data_import::data_import_db::DataImportDb;
    use crate::widgets::popup_list::*;


    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_LOCAL = dep("crate://self/resources/icons/local.svg")
    ICON_BAND_VIEW = dep("crate://self/resources/icons/cloud.svg")
    ICON_CLOUD = dep("crate://self/resources/icons/cloud.svg")
    ICON_MOLYSERVER = dep("crate://self/resources/images/logo.png")

    ApplicationPages = <RoundedShadowView> {
        width: Fill, height: Fill
        margin: {top: 12, right: 12, bottom: 12}
        padding: 3
        flow: Overlay
        show_bg: true
        draw_bg: {
            color: (MAIN_BG_COLOR),
            border_radius: 8.5,
            uniform shadow_color: #0003
            shadow_radius: 18.0,
            shadow_offset: vec2(0.0,-1.5)
        }
        export_frame = <ExportScreen> {visible: true}
        querys_frame = <QueryScreen> {visible: false}
        box_band_frame = <BoxBandView> {visible: false}
        data_import_db_frame = <DataImportDb> {visible: false}
        providers_frame = <ProvidersScreen> {visible: false}
    }

    SidebarMenu = <RoundedView> {
        width: 90, height: Fill,
        flow: Down, spacing: 15.0,
        padding: { top: 40, bottom: 10, left: 10, right: 10 },

        align: {x: 0.5, y: 0.0},
        show_bg: true,
        draw_bg: {
            color: (SIDEBAR_BG_COLOR),
            instance border_radius: 0.0,
        }
        logo = <View> {
            width: Fit, height: Fit
            padding: {bottom:20}
            <Image> {
                width: 50, height: 50,
                source: (ICON_MOLYSERVER),
            }
        }
        seprator = <View> {
            width: Fill, height: 1.6,
            margin: {left: 15, right: 15, bottom: 10}
            show_bg: true
            draw_bg: {
                color: #dadada,
            }
        }

        export_tab = <SidebarMenuButton> {
            animator: {active = {default: on}}
            text: "查询导出",
            draw_icon: {
                svg_file: (ICON_CHAT),
            }
        }
        sn_tab = <SidebarMenuButton> {
            text: "数据查询",
            draw_icon: {
                svg_file: (ICON_LOCAL),
            }
        }
        box_band_tab = <SidebarMenuButton> {
            text: "盒号绑定",
            draw_icon: {
                svg_file: (ICON_BAND_VIEW),
            }
        }
        data_import_db_tab = <SidebarMenuButton> {
            text: "外协数据导入",
            draw_icon: {
                svg_file: (ICON_BAND_VIEW),
            }
        }
        <HorizontalFiller> {}
        set_btn = <View> {
            align: {y: 1.0}
            visible: false
            providers_tab = <SidebarMenuButton> {
                text: "设置",
                draw_icon: {
                    svg_file: (ICON_CLOUD),
                }
            }
        }

    }

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(1600, 900), title: "DPS"},
            pass: {clear_color: #FFFFFF00}
            caption_bar = {
                    caption_label = {
                        label = {
                            margin: {left: 65},
                            align: {x: 0.5},
                            text: "DPS",
                            draw_text: {color: #000000}
                        }
                    }
                    windows_buttons = {
                        min   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        max   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        close = { draw_bg: {color: #0, color_hover: #E81123, color_down: #FF0015} }
                    }
                    draw_bg: {color: #F3F3F3},
            }
            body = {
                flow: Overlay
                width: Fill,
                height: Fill,
                padding: 0


                root = {{MyRoot}} {
                    width: Fill,
                    height: Fill,
                    show_bg: true,
                    draw_bg: {
                        color: (MAIN_BG_COLOR_DARK),
                    }

                    root_adaptive_view = <View> {
                        visible: false

                            sidebar_menu = <SidebarMenu> {}
                            application_pages = <ApplicationPages> {}
                    }
                    login_view = <View> {
                        visible: true
                        login_screen = <LoginScreen> {}

                    }
                }
                <PopupList> {}
                dialog_ui = <Modal> {
                    content : {
                        dialog_ui_inner = <ErrorDialog> {
                        }
                    }
                }

            }


        }

    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    pub ui: WidgetRef,
    #[rust]
    pub store: Store,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::shared::live_design(cx);
        crate::export::live_design(cx);
        crate::settings::live_design(cx);
        crate::widgets::live_design(cx);
        crate::querys::live_design(cx);
        crate::login_view::live_design(cx);
        crate::box_band::live_design(cx);
        crate::data_import::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);

        set_global_popup_list(cx, &self.ui);
        let store = &mut self.store;
        let scope = &mut Scope::with_data(store);
        self.ui.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let rt = self.rt.handle().clone();
        self.ui.view(ids!(body)).set_visible(cx, false);
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
            .radio_button_set(ids_array!(
                sidebar_menu.export_tab,
                sidebar_menu.sn_tab,
                sidebar_menu.box_band_tab,
                sidebar_menu.data_import_db_tab,
                sidebar_menu.providers_tab,
            ))
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
                    .modal(ids!(dialog_ui.dialog_ui_inner))
                    .label(ids!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(ids!(dialog_ui)).open(cx);
            }
            if let Some(tip) = action.downcast_ref::<MyTip>() {
                let content = tip.to_string();
                println!("{content}");
                self.ui
                    .modal(ids!(dialog_ui.dialog_ui_inner))
                    .label(ids!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(ids!(dialog_ui)).open(cx);
            }
            if let Some(ErrprModalAction::Close) = action.downcast_ref() {
                self.ui.modal(ids!(dialog_ui)).close(cx);
            }
            if let Some(LoginResult::Logined) = action.downcast_ref() {
                let store = self.store.clone();
                // store.logined = true;
                // store.free_login = false;
                let show_login = !store.login_store.logined;
                self.ui.view(ids!(login_view)).set_visible(cx, show_login);
                self.ui
                    .view(ids!(root_adaptive_view))
                    .set_visible(cx, !show_login);
                self.ui.view(ids!(set_btn)).set_visible(cx, true);

                enqueue_popup_notification(PopupItem {
                    kind: PopupKind::Success,
                    auto_dismissal_duration: Some(2.5),
                    message: "登录成功，解锁设置页面.".to_string(),
                });
            }
            if let Some(LoginResult::FreeLogin) = action.downcast_ref() {
                let store = self.store.clone();
                // store.logined = true;
                // store.free_login = true;
                let show_login = !store.login_store.logined;
                self.ui.view(ids!(login_view)).set_visible(cx, show_login);
                self.ui
                    .view(ids!(root_adaptive_view))
                    .set_visible(cx, !show_login);
                self.ui.button(ids!(providers_tab)).set_visible(cx, false);
                self.ui.view(ids!(set_btn)).set_visible(cx, false);

                enqueue_popup_notification(PopupItem {
                    kind: PopupKind::Warning,
                    auto_dismissal_duration: Some(2.5),
                    message: "未使用账号密码登录，隐藏设置页面.".to_string(),
                });
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
            self.ui.widget(providers_id).set_visible(cx, false);
        }

        if id != export_id {
            self.ui.widget(export_id).set_visible(cx, false);
        }

        if id != sn_id {
            self.ui.widget(sn_id).set_visible(cx, false);
        }

        if id != box_band_id {
            self.ui.widget(box_band_id).set_visible(cx, false);
        }

        if id != data_import_db_id {
            self.ui.widget(data_import_db_id).set_visible(cx, false);
        }
        self.ui.widget(id).set_visible(cx, true);
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct MyRoot {
    #[deref]
    view: View,
}

impl Widget for MyRoot {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if scope.data.get::<Store>().is_none() {
            return DrawStep::done();
        }
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if scope.data.get::<Store>().is_none() {
            return;
        }
        self.view.handle_event(cx, event, scope);
    }
}
