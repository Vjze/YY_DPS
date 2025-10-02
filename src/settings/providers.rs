use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;



    ProviderItem = {{ProviderItem}}<RoundedView> {
        width: Fill, height: 40
        flow: Overlay
        show_bg: true
        draw_bg: {
            border_radius: 5
        }
        padding: {left: 50}
        align: {x: 0.0, y: 0.5}

        main_view = <View> {
            cursor: Hand
            align: {x: 0.0, y: 0.5}
            spacing: 20
            flow: Right

            // provider_icon = <View> {
            //     width: Fit, height: Fit
            //     image_wrapper = <View> {
            //         width: Fit, height: Fit
            //         provider_icon_image = <Image> {
            //             width: 25, height: 25
            //         }
            //         visible: true
            //     }

            //     label_wrapper = <RoundedView> {
            //         width: 25, height: 25
            //         visible: false
            //         show_bg: true
            //         draw_bg: {
            //             color: #344054
            //             border_radius: 6
            //         }
            //         align: {x: 0.5, y: 0.5}

            //         initial_label = <Label> {
            //             draw_text:{
            //                 text_style: <BOLD_FONT>{font_size: 12}
            //                 color: #f
            //             }
            //         }
            //     }
            // }


            <View> {
                flow: Right
                width: Fill, height: Fill
                spacing: 20
                align: {x: 0.0, y: 0.5}

                provider_name_label = <Label> {
                    draw_text:{
                        text_style: <BOLD_FONT>{font_size: 15}
                        color: #000
                    }
                }

                filler = <View> { width: Fill, height: Fill }
            }

        }

    }



    pub Providers = {{Providers}} {
                width: 200, height: Fill
                flow: Down, spacing: 10
                padding: {left: 10, right: 10}
                providers_list = <PortalList> {
                    width: Fill, height: Fill
                    provider_item = <ProviderItem> {}
                }

                // provider_icons: [
                //     (ICON_OPENAI),
                //     (ICON_GEMINI),
                //     (ICON_SILICONFLOW),
                //     (ICON_OPENROUTER),
                //     (ICON_MOLYSERVER),
                // ]




    }
}

#[derive(Widget, Live)]
struct Providers {
    #[deref]
    view: View,
    #[rust]
    selected_provider: Option<String>,
}
impl LiveHook for Providers {
    fn before_apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        // 设置一个默认选中项，例如“类型设置”
        self.selected_provider = Some("类型设置".to_string());
        // 确保在初始化时触发一次视图切换
        cx.action(ConnectionSettingsAction::ProviderSelected(
            "类型设置".to_string(),
        ));
    }
}

impl Widget for Providers {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let all_providers = vec!["类型设置", "模板设置", "映射设置"];
        let entries_count = all_providers.len();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, entries_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < entries_count {
                        let template = live_id!(provider_item);
                        let item = list.item(cx, item_id, template);

                        // hide the separator for the first item
                        if item_id == 0 {
                            item.view(id!(separator)).set_visible(cx, false);
                        }

                        let provider = all_providers[item_id];
                        // let icon = self.get_provider_icon(provider.to_string());
                        let is_selected = self.selected_provider == Some(provider.to_string());
                        item.as_provider_item().set_provider(
                            cx,
                            provider.to_string().clone(),
                            // icon,
                            is_selected,
                        );
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for Providers {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        for action in actions {
            // Handle selected provider
            if let ConnectionSettingsAction::ProviderSelected(provider_url) = action.cast() {
                self.selected_provider = Some(provider_url);
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
struct ProviderItem {
    #[deref]
    view: View,

    #[rust]
    provider: String,
}

impl Widget for ProviderItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update the label
        self.label(id!(provider_name_label))
            .set_text(cx, &self.provider);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ProviderItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let was_item_clicked = self.view(id!(main_view)).finger_up(actions).is_some();
        if was_item_clicked {
            cx.action(ConnectionSettingsAction::ProviderSelected(
                self.provider.clone(),
            ));
        }
    }
}

impl ProviderItemRef {
    fn set_provider(&mut self, cx: &mut Cx, provider: String, is_selected: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.provider = provider.clone();

        inner.view(id!(image_wrapper)).set_visible(cx, false);

        // Show the label
        let label_view = inner.view(id!(label_wrapper));
        label_view.set_visible(cx, true);

        // Get first character of the provider name
        let first_char = provider
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();

        label_view
            .label(id!(initial_label))
            .set_text(cx, &first_char);
        // }

        if is_selected && cx.display_context.is_desktop() {
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { color: #EAECEF }
                },
            );
        } else {
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { color: #f9f9f9 }
                },
            );
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ConnectionSettingsAction {
    None,
    ProviderSelected(String),
}
