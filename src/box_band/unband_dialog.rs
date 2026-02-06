use makepad_widgets::*;
live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::widgets::widget::MyDropdown;

    pub UnbandModal = {{UnbandModal}} {
        width: Fit,
        height: Fit,

        wrapper = <RoundedView> {
            flow: Down
            width: 600
            height: Fit
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
                    text: "盒号解绑"
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
                spacing: 20
                    <View> {
                        height: Fit,
                        align: {x:0.5, y:0.5}
                        spacing: 15,
                        padding: {left: 15, right: 15}
                        <Label> {
                            width: Fill
                            draw_text: {
                                text_style: {
                                    font_size: 14
                                },
                                color: #000
                                wrap: Word
                            }
                            text: "选择解绑项:"
                        }
                        untype_selector = <MyDropdown> {
                            width: 100,height:40
                            labels:["箱号","盒号"],
                            padding: {top:12,left:15}
                            draw_text: {
                                uniform color: #000
                                uniform color_down: #000
                                uniform color_hover: #000
                                uniform color_focus: #000
                            }
                            draw_bg: {
                                color: (MAIN_BG_COLOR_DARK),
                                uniform border_radius:5.0,
                                uniform border_size: 1.0
                                uniform border_color_1: #333
                                uniform border_color_1_hover: #555
                                // uniform shadow_color: #0002
                                // shadow_radius: 9.0,
                                // shadow_offset: vec2(0.0,-2.0)
                            }
                            popup_menu: <PopupMenu> {
                                // 自定义菜单背景
                                draw_bg: { uniform color: #333, uniform border_color: #666,
                                    uniform border_size: 1.0 }
                                menu_item: <PopupMenuItem> {
                                    // 自定义菜单项
                                    padding: {left: 20, top: 8, bottom: 8, right: 10}
                                    draw_bg: {
                                        color: #333
                                        color_hover: #555 // 悬停背景
                                        color_active: #888 // 选中背景
                                    }
                                    draw_text: {
                                        color: #EEE // 默认文字颜色
                                        color_hover: #FFF // 悬停文字颜色
                                        color_active: #FFF // 选中文字颜色
                                    }
                                }
                            }
                        }
                    }
                    <View> {
                        height: Fit,
                        align: {x:0.5, y:0.5}
                        spacing: 15,
                        padding: {left: 15, right: 15}
                        <Label> {
                            width: Fill
                            draw_text: {
                                text_style: {
                                    font_size: 14
                                },
                                color: #000
                                wrap: Word
                            }
                            text: "条码:"
                        }
                        no_input = <MolyTextInput> {
                            empty_text: "输入需要解绑的箱号或者盒号...."
                            width: Fill, height: 40
                            padding: 10,
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 12
                                }
                                color: #000
                            }
                            draw_bg: {
                                uniform border_radius: 5.0
                                uniform border_size: 1.0
                            }
                            draw_cursor: {
                                uniform color: #FFF
                            }
                        }
                    }
                    <View> {
                        height: Fit,
                        align: {x:0.5, y:0.5}
                        spacing: 15,
                        padding: {left: 15, right: 15}
                        <Label> {
                            width: Fill
                            draw_text: {
                                text_style: {
                                    font_size: 14
                                },
                                color: #000
                                wrap: Word
                            }
                            text: "密码:"
                        }
                        password_input = <MolyTextInput> {
                            empty_text: "输入密码...."
                            width: Fill, height: 40
                            padding: 10,
                            is_password: true,
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 12
                                }
                                color: #000
                            }
                            draw_bg: {
                                uniform border_radius: 5.0
                                uniform border_size: 1.0
                            }
                            draw_cursor: {
                                uniform color: #FFF
                            }
                        }
                    }
                    error = <Label> {
                        width: Fill
                        draw_text: {
                            text_style: {
                                font_size: 14
                            },
                            color: #000
                            wrap: Word
                        }
                        text: ""
                    }
                <View> {
                    width: Fill, height: Fit
                    flow: Right,
                    align: {x: 1.0, y: 1.0}
                    padding: 15
                    spacing: 15,
                    <Label> {
                        width: Fill
                        draw_text: {
                            text_style: {
                                font_size: 14
                            },
                            color: #000
                            wrap: Word
                        }
                    }
                    accept_button = <Button> {
                        width: 100
                        height: 40
                        padding: {left: 15, right: 15}

                        text: "解绑"
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

#[derive(Live, LiveHook, Widget)]
pub struct UnbandModal {
    #[deref]
    view: View,
}
#[derive(Clone, Debug, DefaultNone)]
pub enum UnbandModalAction {
    Action(String, String),
    None,
    Close,
}
impl Widget for UnbandModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for UnbandModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let accept_button = self.button(ids!(accept_button));
        let cancel_button = self.button(ids!(cancel_button));
        let text_input = self.text_input(ids!(no_input));
        let password_input = self.text_input(ids!(password_input));
        let type_select = self.drop_down(ids!(untype_selector));

        if cancel_button.clicked(actions) {
            Cx::post_action(UnbandModalAction::Close);
        }
        if accept_button.clicked(actions) {
            if text_input.text().is_empty() {
                self.label(ids!(error)).set_text(cx, "输入框不能为空!!");
            } else if password_input.text().is_empty() {
                self.label(ids!(error)).set_text(cx, "密码框不能为空!!");
            } else if type_select.selected_label().is_empty() {
                self.label(ids!(error)).set_text(cx, "类型选择不能为空!!");
            } else if password_input.text() != "Unbind" {
                self.label(ids!(error)).set_text(cx, "密码错误!!");
            } else {
                let no = text_input.text();
                let unbindtype = type_select.selected_label();
                Cx::post_action(UnbandModalAction::Action(unbindtype, no));
            }
        }
    }
}
