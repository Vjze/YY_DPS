use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;



    pub TypeView = {{TypeView}}
        <RoundedShadowView> {
            width: Fill, height: Fill
            align: {x: 0.0, y: 0.0}
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
                    text: "type_view"
                    draw_text:{
                        text_style: <BOLD_FONT>{font_size: 25}
                        color: #000
                    }
                }

            }

    }
}

// TODO: Rename into TypeView
#[derive(Widget, LiveHook, Live)]
struct TypeView {
    #[deref]
    view: View,
}

impl Widget for TypeView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for TypeView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {}
}
