use makepad_widgets::*;


live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;


    pub MapView = {{MapView}}<RoundedShadowView> {
        width: Fill, height: Fill
        // align: {x: 0.0, y: 0.0}
        padding: {left: 30, right: 30, top: 30, bottom: 30}
        show_bg: true
        draw_bg: {
            color: (MAIN_BG_COLOR_DARK)
            border_radius: 4.5,
            uniform shadow_color: #0002
            shadow_radius: 8.0,
            shadow_offset: vec2(0.0,-1.5)
        }

        content = <View> {
            flow: Down, spacing: 20

            <Label> {
                text: "map_view"
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 25}
                    color: #000
                }
            }
        }
    }
}

// TODO: Rename into MapView
#[derive(Widget, LiveHook, Live)]
struct MapView {
    #[deref]
    view: View,

    #[rust]
    provider: String,
    #[rust]
    initialized: bool,
}

impl Widget for MapView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MapView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
    }
}
// impl TypeViewRef {
//     pub fn set_provider(&mut self, cx: &mut Cx, provider: String) {
//         if let Some(mut inner) = self.borrow_mut() {
//             inner.provider = provider.clone();

//             // Update the text inputs
//             let api_key_input = inner.text_input(id!(api_key));
//             if let Some(api_key) = &provider.api_key {
//                 api_key_input.set_text(cx, &api_key);
//             } else {
//                 api_key_input.set_text(cx, "");
//             }

//             inner.text_input(id!(api_host)).set_text(cx, &provider);
//             inner.label(id!(name)).set_text(cx, &provider.name);
//             inner
//                 .check_box(id!(provider_enabled_switch))
//                 .set_active(cx, provider.enabled);

//             if provider.was_customly_added {
//                 inner.view(id!(remove_provider_view)).set_visible(cx, true);
//             } else {
//                 inner.view(id!(remove_provider_view)).set_visible(cx, false);
//             }

//             inner.view.redraw(cx);
//         }
//     }
// }