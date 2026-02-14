use makepad_widgets::*;

use crate::{
    store::Store,
    utils::error::{LoginResult, MyError},
};

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    let ICON_LOGO = crate_resource("self://resources/images/logo.png")

    mod.widgets.LoginScreenBase = #(LoginScreen::register_widget(vm))
    mod.widgets.LoginScreen = set_type_default() do mod.widgets.LoginScreenBase {
        width: Fill, height: Fill,
        align: Align{x: 0.5, y: 0.5}
        draw_bg +: {
            color: #FFF
        }
        flow: Flow.Overlay,
        RoundedView {
            padding: 50,
            width: 400,
            height: 400,
            spacing:10,
            flow: Flow.Down,
            align: Center
            draw_bg +: {
                // color: #f2f2f2,
                border_radius: 4.5,
                shadow_color: #E0FFFF,
                shadow_radius: 8.0,
                shadow_offset: vec2(0.0,-1.5)
            }
            SolidView {
                width: Fill
                height: Fit
                spacing:20,
                align: Center
                Image {
                    width: 50, height: 50,
                    src: ICON_LOGO
                }
                    H1{
                        text: "数据查询导出工具"
                            draw_text.color: #000
                            draw_text.text_style.font_size: 20
                    }

            }
            View {
                align: Align{y: 0.5}
                spacing: 10,
                Label {
                    draw_text.color: #000,
                    draw_text.text_style.font_size: 16
                    text: "账户:"
                }
                user_name := TextInput {
                    draw_text.color: #000,
                        draw_text.text_style.font_size: 16
                    empty_text: "输入用户名..."
                }
            }
            View {
                align: Align{y: 0.5}
                spacing: 10,
                Label {
                    draw_text.color: #000,
                        draw_text.text_style.font_size: 16
                    text: "密码:"
                }
                use_password := TextInput {
                    draw_text.color: #000,
                        draw_text.text_style.font_size: 16
                    empty_text: "输入密码...",
                    is_password: true,

                }
            }
            View {
                align: Align{x: 0.5, y: 1.0}
                spacing: 30,
                free_btn := Button {
                    text: "跳过登录"
                    draw_text.color: #000,
                    draw_text.text_style.font_size: 16
                    draw_bg +: {
                        border_size: uniform(1.0)
                        border_radius: uniform(5.0)
                        color: uniform(#FF7F50)
                        color_hover: uniform(#FFB6C1)
                        olor_disabled: (#A9A9A9)

                    }
                }
                login_btn := Button {
                    width: 100,
                    text: "登录"
                    draw_text.color: #000,
                        draw_text.text_style.font_size: 16
                    draw_bg +: {
                        border_size: uniform(1.0)
                        border_radius: uniform(5.0)
                        color: uniform(#FF7F50)
                        color_hover: uniform(#FFB6C1)
                        color_disabled: (#A9A9A9)

                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct LoginScreen {
    #[deref]
    view: View,
}
impl Widget for LoginScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for LoginScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let user_name = self.view.text_input(cx, ids!(user_name));
        let user_password = self.view.text_input(cx, ids!(use_password));
        let free_btn = self.view.button(cx, ids!(free_btn));
        let login_btn = self.view.button(cx, ids!(login_btn));
        if login_btn.clicked(actions) || user_password.returned(actions).is_some() {
            if user_name.text().is_empty() {
                Cx::post_action(MyError::LoginError(format!("用户名不能为空!!!")));
            } else if user_password.text().is_empty() {
                Cx::post_action(MyError::LoginError(format!("密码不能为空!!!")));
            } else {
                if user_name.text() == "admin" && user_password.text() == "123456" {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        store.login_store.logined = true;
                        store.login_store.free_login = false;
                    }
                    Cx::post_action(LoginResult::Logined);
                } else {
                    Cx::post_action(MyError::LoginError(format!("用户名或者密码错误!!!")));
                }
            }
        }
        if free_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.login_store.logined = true;
                store.login_store.free_login = true;
            }
            Cx::post_action(LoginResult::FreeLogin);
        }
    }
}
