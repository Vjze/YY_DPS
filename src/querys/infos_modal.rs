use makepad_widgets::*;

live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::style::*;
    use crate::querys::sn_tabel::SnInfosTable;

    pub InfosModal = {{InfosModal}} {
        <RoundedView> {
            flow: Down
            width: 800
            height: 600
            // padding: {top: 15, right: 25 bottom: 25 left: 25}
            spacing: 15
            show_bg: true
            draw_bg: {
                color: #fff
                uniform border_radius: 4.0
                fn pixel(self) -> vec4 {
                    let border_color = #d4;
                    let border_size = 1;
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let body = #fff

                    sdf.box(
                        1.,
                        1.,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        self.border_radius
                    )
                    sdf.fill_keep(body)

                    sdf.stroke(
                        border_color,
                        border_size
                    )
                    return sdf.result
                }
            }

            <RoundedView> {
                width: Fill,
                height: Fit,
                align: {x: 0.5, y: 0.5}
                show_bg: true,
                draw_bg: {
                    uniform border_radius: 4.0
                    uniform border_color: #0000
                    fn get_color(self) -> vec4 {
                        return mix(#B0E0E6,#C1CDC1,self.pos.x)
                    }

                    fn get_border_color(self) -> vec4 {
                        return self.border_color
                    }

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                        sdf.box(
                            self.border_inset.x + self.border_size,
                            self.border_inset.y + self.border_size,
                            self.rect_size.x - (self.border_inset.x + self.border_inset.z + self.border_size * 2.0),
                            self.rect_size.y - (self.border_inset.y + self.border_inset.w + self.border_size * 2.0),
                            max(1.0, self.border_radius)
                        )
                        sdf.fill_keep(self.get_color())
                        if self.border_size > 0.0 {
                            sdf.stroke(self.get_border_color(), self.border_size)
                        }
                        return sdf.result;
                    }

                        }
                <Label> {
                    text: "详细信息"
                    draw_text: {
                        text_style:{font_size: 16},
                        color: #000
                    }
                }
                <View> {
                    align: {y:1.0}
                    close_btn = <Button> {
                        text: "X"
                    }
                }
            }
            <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                padding:15.
                infos_tabel = <SnInfosTable> {}
            }
        }

    }
);

// 添加 InfosModal 结构体和实现
#[derive(Live, LiveHook, Widget)]
pub struct InfosModal {
    #[deref]
    view: View,
}
#[derive(Clone, Debug, DefaultNone)]
pub enum ErrprModalAction {
    None,
    Close,
}
impl Widget for InfosModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for InfosModal {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let close_btn = self.button(ids!(close_btn));
        if close_btn.clicked(actions) {
            Cx::post_action(ErrprModalAction::Close);
        }
    }
}
