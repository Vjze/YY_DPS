use makepad_widgets::*;

use crate::{store::Store, utils::error::{MyError, MyTip}};
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;

    FirstRow = <View> {
        width: Fill,
        height: 100,
        padding: 15,
        spacing:20,
        // show_bg: true,
        // draw_bg: {
        //     color: #D9D9D9
        // }
        type_selector = <DropDown> {
            width: 100,height:40
            labels:["type_1","type_2","type_3"],
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
        carton_input = <MolyTextInput> {
            empty_text: "请输入箱号...."
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
        query_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "箱号查询"
            draw_text: {
                color: #000000,
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
        export_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "数据导出"
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
        

        qty_label = <Label> {
            padding: {
                top:5
            }
            text: "总数量: 0 PCS"
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
    }

    pub ExportScreen = {{ExportScreen}} {
        <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing:20,
            <FirstRow> {

            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct ExportScreen {
    #[deref]
    view: View,
    #[rust]
    store: Store,
}
impl Widget for ExportScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ExportScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let input = self.view.text_input(id!(carton_input));
        let query_btn = self.view.button(id!(query_btn));
        let export_btn = self.view.button(id!(export_btn));
        if input.text().is_empty() {
            query_btn.set_text(cx, "批量查询");
        } else {
            query_btn.set_text(cx, "箱号查询");
        }
        if self.store.datas.is_empty() {
            export_btn.set_disabled(cx, true);
        } else {
            export_btn.set_disabled(cx, false);
        }
    }
}
