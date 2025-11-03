use makepad_widgets::*;

use super::providers::ConnectionSettingsAction;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::modal::*;
    use crate::settings::providers::Providers;
    use crate::settings::type_setting_view::TypeView;
    use crate::settings::template_setting_view::TemplateView;
    use crate::settings::map_setting_view::MapView;
    use crate::widgets::login_view::LoginView;

    HorizontalSeparator = <RoundedView> {
        width: 2, height: Fill
        show_bg: true
        draw_bg: {
            color: #d3d3d3
        }
    }
    SettingPages = <RoundedShadowView> {
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

        type_frame = <TypeView> {visible: true}
        template_frame = <TemplateView> {visible: false}
        map_frame = <MapView> {visible: false}
    }

    pub ProvidersScreen = {{ProvidersScreen}} {
        width: Fill, height: Fill
        spacing: 20
        // flow: Overlay
        <View> {
            width: Fit, height: Fill
            flow: Down

            padding: {left: 30, top: 20}
            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 25}
                    color: #000
                }
                text: "软件设置"
            }
            <View> {
                height:Fill
            }
            providers = <Providers> {}
            // <Label> {
            //     draw_text:{
            //         text_style: <BOLD_FONT>{font_size: 12}
            //         color: #000
            //     }
            //     text: "非专业人员请勿乱动，否则可能造成不可逆的后果!!!"
            // }
        }

        adaptive_view = <View> {
                spacing: 10
                padding: {top: 10}
                // providers = <Providers> {}
                setting_view = <SettingPages> {}

        }
    }

}

#[derive(Widget, LiveHook, Live)]
pub struct ProvidersScreen {
    #[deref]
    view: View,
    #[rust]
    selected_provider: Option<String>,
}

impl Widget for ProvidersScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // if let Some(store) = scope.data.get_mut::<Store>(){
        //     if store.logined {
        //         self.view.modal(ids!(login_view)).close(cx);
        //     }else{
        //         self.view.modal(ids!(login_view)).open(cx);
        //     }
        // }
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ProvidersScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let stack_navigation = self.stack_navigation(ids!(navigation));
        stack_navigation.handle_stack_view_actions(cx, actions);

        for action in actions {
            if let ConnectionSettingsAction::ProviderSelected(address) = action.cast() {
                self.selected_provider = Some(address)
            }
        }

        if let Some(view) = &self.selected_provider {
            let view = view.as_str();
            match view {
                "类型设置" => {
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(type_frame))
                        .set_visible(cx, true);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(template_frame))
                        .set_visible(cx, false);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(map_frame))
                        .set_visible(cx, false);
                }
                "模板设置" => {
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(type_frame))
                        .set_visible(cx, false);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(template_frame))
                        .set_visible(cx, true);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(map_frame))
                        .set_visible(cx, false);
                }
                _ => {
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(type_frame))
                        .set_visible(cx, false);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(template_frame))
                        .set_visible(cx, false);
                    self.view
                        .widget(ids!(setting_view))
                        .widget(ids!(map_frame))
                        .set_visible(cx, true);
                }
            }
        }
    }
}
