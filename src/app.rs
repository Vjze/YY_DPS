use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    store::Store,
    utils::error::{MyError, MyTip},
    widgets::dialog::ErrprModalAction,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::widgets::SidebarMenuButton;
    use crate::shared::desktop_buttons::MolyDesktopButton;

    use crate::export::export_view::ExportScreen;
    use crate::settings::providers_screen::ProvidersScreen;
    use crate::widgets::dialog::*;


    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_LOCAL = dep("crate://self/resources/icons/local.svg")
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
        // sn_frame = <MolyServerScreen> {visible: false}
        providers_frame = <ProvidersScreen> {visible: false}
    }

    SidebarMenu = <RoundedView> {
        width: 90, height: Fill,
        flow: Down, spacing: 15.0,
        padding: { top: 40, bottom: 20, left: 10, right: 0 },

        align: {x: 0.5, y: 0.0},

        show_bg: true,
        draw_bg: {
            color: (SIDEBAR_BG_COLOR),
            instance border_radius: 0.0,
        }

        logo = <View> {
            width: Fit, height: Fit
            padding: {left:10, bottom:20}
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
            text: "Sn查询",
            draw_icon: {
                svg_file: (ICON_LOCAL),
            }
        }
        <HorizontalFiller> {}
        providers_tab = <SidebarMenuButton> {
            text: "设置",
            draw_icon: {
                svg_file: (ICON_CLOUD),
            }
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(1440, 1024), title: "DPS"},
            pass: {clear_color: (THEME_COLOR_FG_APP)},
            caption_bar = {
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return mix(#C1CDC1,#B0E0E6,self.pos.x);
                    }
                }
                caption_label = <View> {
                    width: Fill, height: Fill,
                    align: {x: 0.5, y: 0.5},
                    label = <Label> {
                        text: "DPS",
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> {
                                font_size: 12
                            }
                            color: #000000,
                        }

                        margin: {left: 100

                        }
                    }
                }
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
                        // Mobile = {
                        //     application_pages = <ApplicationPages> {
                        //         margin: 0
                        //     }
                        // }

                        // Desktop = {
                            sidebar_menu = <SidebarMenu> {}
                            application_pages = <ApplicationPages> {}
                        // }
                    }
                }
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
    pub store: Option<Store>,
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
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);
        let rt = self.rt.handle().clone();
        if let Event::Startup = event {
            self.ui.view(id!(body)).set_visible(cx, false);
            let _guard = rt.enter();
            let store = rt.block_on(async move { Store::init().await });
            self.store = Some(store);
        }
        // println!("store = {:?}",self.store);
        // If the store is not loaded, do not continue with store-dependent logic
        // however, we still want the window to handle Makepad events. (e.g. window initialization events, platform context changes, etc.)
        // self.store = Some(Store { ..Default::default() });
        let Some(store) = self.store.as_mut() else {
            self.ui.handle_event(cx, event, &mut Scope::empty());
            return;
        };

        let scope = &mut Scope::with_data(store);
        self.ui.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut navigate_to_export = false;
        let mut navigate_to_sn = false;
        let mut navigate_to_providers = false;

        // TODO: Replace this with a proper navigation widget.
        if let Some(selected_tab) = self
            .ui
            .radio_button_set(ids!(
                sidebar_menu.export_tab,
                sidebar_menu.sn_tab,
                sidebar_menu.providers_tab,
            ))
            .selected(cx, actions)
        {
            match selected_tab {
                0 => navigate_to_export = true,
                1 => navigate_to_sn = true,
                2 => navigate_to_providers = true,
                _ => {}
            }
        }
        // Handle navigation after processing all actions
        if navigate_to_providers {
            self.navigate_to(cx, id!(application_pages.providers_frame));
        } else if navigate_to_export {
            self.navigate_to(cx, id!(application_pages.export_frame));
        } else if navigate_to_sn {
            self.navigate_to(cx, id!(application_pages.sn_frame));
        }
        for action in actions {
            if let Some(err) = action.downcast_ref::<MyError>() {
                let content = err.to_string();
                println!("{content}");
                self.ui
                    .modal(id!(dialog_ui.dialog_ui_inner))
                    .label(id!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(id!(dialog_ui)).open(cx);
            }
            if let Some(tip) = action.downcast_ref::<MyTip>() {
                let content = tip.to_string();
                println!("{content}");
                self.ui
                    .modal(id!(dialog_ui.dialog_ui_inner))
                    .label(id!(prompt))
                    .set_text(cx, &content);
                self.ui.modal(id!(dialog_ui)).open(cx);
            }
            if let Some(ErrprModalAction::Close) = action.downcast_ref() {
                self.ui.modal(id!(dialog_ui)).close(cx);
            }
        }
        cx.redraw_all();
    }
}

impl App {
    fn navigate_to(&mut self, cx: &mut Cx, id: &[LiveId]) {
        let providers_id = id!(application_pages.providers_frame);
        let export_id = id!(application_pages.export_frame);
        let sn_id = id!(application_pages.sn_frame);

        if id != providers_id {
            self.ui.widget(providers_id).set_visible(cx, false);
        }

        if id != export_id {
            self.ui.widget(export_id).set_visible(cx, false);
        }

        if id != sn_id {
            self.ui.widget(sn_id).set_visible(cx, false);
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
