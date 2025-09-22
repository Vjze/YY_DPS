use makepad_widgets::*;

use crate::store::Store;
live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::style::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub LoginView = {{LoginView}} {
        width: Fit,
        height: Fit,

        wrapper = <RoundedView> {
            flow: Down
            width: 600
            height: Fit

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

            title = <RoundedView> {
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
                    text: "登录"
                    draw_text: {
                        text_style:{font_size: 16},
                        color: #000
                    }
                }
            }
            body = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                padding:{left: 15}

                spacing: 40
                    <View> {
                        width: Fill
                        height: Fit
                        align: {x: 0.5, y: 0.5}
                        spacing: 10
                        <Label> {
                            width: 100
                            height: Fit
                            text: "用户名："
                            draw_text: {
                                color: #000000,
                                text_style: {
                                    font_size:16
                                }
                            }
                        }
                        user_name = <MolyTextInput> {
                            width: Fill
                            height: Fit
                            empty_text: "请输入用户名..."
                            draw_text: {
                                color: #000000,
                                text_style: {
                                    font_size:16
                                }
                            }
                        }
                    }
                    <View> {
                        width: Fill
                        height: Fit
                        spacing: 10
                        align: {x: 0.5, y: 0.5}

                        <Label> {
                            width: Fit
                            height: Fit
                            text: "密码："
                            draw_text: {
                                color: #000000,
                                text_style: {
                                    font_size:16
                                }
                            }
                        }
                        user_password = <MolyTextInput> {
                            width: Fill
                            height: Fit
                            empty_text: "请输入密码..."
                            is_password: true
                            draw_text: {
                                color: #000000,
                                text_style: {
                                    font_size:16
                                }
                            }
                        }
                    }



                <View> {
                    width: Fill, height: Fit
                    flow: Right,
                    align: {x: 1.0, y: 1.0}
                    spacing: 30
                    padding: 15

                    accept_button = <Button> {
                        width: 100
                        height: 40
                        padding: {left: 15, right: 15}

                        text: "确定"
                        draw_text: {
                            color: #000000,
                            text_style: {
                                font_size:16
                            }
                        }
                        draw_bg: {
                            uniform border_size: 1.0
                            uniform border_radius: 5.0
                            uniform color: #AFEEEE
                            uniform color_hover: #9370DB
                            uniform color_disabled: #DCDCDC

                         }
                    }

                    cancel_button = <Button> {
                        width: 100
                        height: 40
                        padding: {left: 15, right: 15}

                        text: "取消"
                        draw_text: {
                            color: #000000,
                            text_style: {
                                font_size:16
                            }
                        }
                        draw_bg: {
                            uniform border_size: 1.0
                            uniform border_radius: 5.0
                            uniform color: #AFEEEE
                            uniform color_hover: #9370DB
                            uniform color_disabled: #DCDCDC

                         }
                    }
                }
            }
        }

    }
);

// 添加 LoginView 结构体和实现
#[derive(Live, LiveHook, Widget)]
pub struct LoginView {
    #[deref]
    view: View,
}
#[derive(Clone, Debug, DefaultNone)]
pub enum ErrprModalAction {
    None,
    Close,
}
impl Widget for LoginView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for LoginView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let accept_button = self.button(id!(accept_button));
        let cancel_button = self.button(id!(cancel_button));
        let user_name = self.text_input(id!(user_name)).text();
        let user_password = self.text_input(id!(user_password)).text();
        if self.text_input(id!(user_name)).returned(actions).is_some(){
             self.text_input(id!(user_password)).set_key_focus(cx);
        }
        if accept_button.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                if user_name != "admin" || user_password != "123456" {
                    return;
                }

                store.logined = true;
            }
        }
        if cancel_button.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.logined = false;
            }
        }
    }
}
