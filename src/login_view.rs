use makepad_widgets::*;

use crate::{
    store::Store,
    utils::error::{LoginResult, MyError},
};

live_design! {

    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::modal::*;

    ICON_LOGO = dep("crate://self/resources/images/logo.png")
    pub LoginScreen = {{LoginScreen}} {
        width: Fill, height: Fill,
        align: {x: 0.5, y: 0.5}
        show_bg: true,
        draw_bg: {
            color: #FFF
        }
        flow: Overlay,
        <RoundedShadowView> {
            padding: 50,
            width: 400,
            height: 400,
            spacing:10,
            flow: Down,
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                color: (MAIN_BG_COLOR_DARK)
                border_radius: 4.5,
                uniform shadow_color: #0002
                shadow_radius: 8.0,
                shadow_offset: vec2(0.0,-1.5)
            }
            <View> {
                width: Fill
                height: Fit
                spacing:20,
                align: {x: 0.5, y:0.5}
                <Image> {
                    width: 50, height: 50,
                    source: (ICON_LOGO)
                }
                <H1> {
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:20
                        }
                    }
                    text: "数据查询导出工具"
                }
            }
            <View> {
                align: {y: 0.5}
                spacing: 10,
                <Label> {
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:16
                        }
                    }
                    text: "账户:"
                }
                user_name = <TextInput> {
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:16
                        }
                    }
                    empty_text: "输入用户名..."
                }
            }
            <View> {
                align: {y: 0.5}
                spacing: 10,
                <Label> {
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:16
                        }
                    }
                    text: "密码:"
                }
                use_password = <TextInput> {
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:16
                        }
                    }
                    empty_text: "输入密码...",
                    is_password: true,

                }
            }
            <View> {
                align: {x: 0.5, y: 1.0}
                spacing: 30,
                free_btn = <Button> {
                    text: "跳过登录"
                    draw_text: {
                        color: #000,
                        text_style: {
                            font_size:16
                        }
                    }
                    draw_bg: {
                        uniform border_size: 1.0
                        uniform border_radius: 5.0
                        uniform color: #FF7F50
                        uniform color_hover: #FFB6C1
                        uniform color_disabled: #A9A9A9

                    }
                }
                login_btn = <Button> {
                    width: 100,
                    text: "登录"
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

#[derive(Widget, LiveHook, Live)]
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
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let user_name = self.view.text_input(id!(user_name));
        let user_password = self.view.text_input(id!(use_password));
        let free_btn = self.view.button(id!(free_btn));
        let login_btn = self.view.button(id!(login_btn));
        if login_btn.clicked(actions) || user_password.returned(actions).is_some() {
            if user_name.text().is_empty() {
                Cx::post_action(MyError::LoginError(format!("用户名不能为空!!!")));
            } else if user_password.text().is_empty() {
                Cx::post_action(MyError::LoginError(format!("密码不能为空!!!")));
            } else {
                if user_name.text() == "admin" && user_password.text() == "123456" {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        store.logined = true;
                        store.free_login = false;
                    }
                    Cx::post_action(LoginResult::Logined);
                } else {
                    Cx::post_action(MyError::LoginError(format!("用户名或者密码错误!!!")));
                }
            }
        }
        if free_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.logined = true;
                store.free_login = true;
            }
            Cx::post_action(LoginResult::FreeLogin);
        }
    }
}
