use makepad_widgets::*;
script_mod!(
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.DialogBase =  #(ErrorDialog::register_widget(vm))

    mod.widgets.ErrorDialog = mod.widgets.DialogBase {
        width: Fit,
        height: Fit,

        wrapper := RoundedView {
            flow: Down
            width: 600
            height: Fit
            draw_bg.color: #fff

            title := RoundedView {
                width: Fill,
                height: Fit,
                align: Center
                draw_bg +: {
                    border_radius: 4.0
                    border_color: #0000
                    get_color: fn(){
                        return mix(#B0E0E6,#C1CDC1,self.pos.x)
                    }

                    get_border_color: fn(){
                        return self.border_color
                    }

                    pixel: fn(){
                        let sdf = Sdf2d.viewport(self.pos * self.rect_size)
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
                Label {
                    text: "提示"
                    draw_text +: {
                        text_style +:{font_size: 16},
                        color: #000
                    }
                }
            }
                RoundedView {
                    width: Fill,
                    height: Fit,
                    draw_bg.color: #DCDCDC
                    draw_bg.radius: 8.0
                    padding: 10 spacing: 12
                    flow: Down align: Center
                    prompt := Label {
                        width: Fill
                        draw_text +: {
                            text_style +: {
                                font_size: 14
                            },
                            color: #000
                        }
                        text: "提示内容"
                    }
                    accept_button := Button {
                        width: 100
                        height: 40
                        padding: Inset{left: 15, right: 15}

                        text: "确定"
                        draw_text +: {
                            color: #000000,
                            text_style +: {
                                font_size:16
                            }
                        }
                        draw_bg +: {
                            border_size: 1.0
                            border_radius: 5.0
                            color: #AFEEEE
                            color_hover: #9370DB
                            color_disabled: #DCDCDC

                        }
                    }
                }


        }

    }
);
#[derive(Clone)]
pub struct ErrorDialogProps {
    pub error_text: String,
}
// 添加 ErrorDialog 结构体和实现
#[derive(Script, ScriptHook, Widget)]
pub struct ErrorDialog {
    #[deref]
    view: View,
}
#[derive(Clone, Debug)]
pub enum ErrprModalAction {
    None,
    Close,
}
impl Widget for ErrorDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for ErrorDialog {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let accept_button = self.button(cx, ids!(accept_button));
        let accept_button_clicked = accept_button.clicked(actions);
        if accept_button_clicked {
            Cx::post_action(ErrprModalAction::Close);
        }
    }
}
impl ErrorDialog {
    fn set_err_text(&mut self, cx: &mut Cx, error_text: String) {
        self.view.label(cx, ids!(prompt)).set_text(cx, &error_text);
    }
}

impl ErrorDialogRef {
    pub fn set_err_text(&self, cx: &mut Cx, error_text: String) {
        info!("set_err_text: {}", error_text);
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_err_text(cx, error_text);
        }
    }
}
