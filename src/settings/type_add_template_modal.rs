use makepad_widgets::*;
use rayon::prelude::*;
use std::collections::HashSet;

use crate::store::Store;
live_design!(
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub AddTemplateNameModal = {{AddTemplateNameModal}} {
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
                    text: "模板添加"
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
                            text: "选择模板名称:"
                        }
                        template_select = <DropDown> {
                            labels: []
                            width: 300, height: 40
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

#[derive(Live, LiveHook, Widget)]
pub struct AddTemplateNameModal {
    #[deref]
    view: View,
}
#[derive(Clone, Debug, DefaultNone)]
pub enum TemplateNameModalAction {
    Action(String),
    None,
    Close,
}
impl Widget for AddTemplateNameModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get::<Store>() {
            if store.setting_store.templates.len() > 0 {
                let dropdown = self.view.drop_down(ids!(template_select));
                let templates_all = store.setting_store.templates.clone();
                let template_haved = store.setting_store.type_infos.0.clone();
                let templates_set: HashSet<_> = templates_all.iter().collect();
                let part_set: HashSet<_> = template_haved.iter().collect();
                let part_iter = template_haved.par_iter().map(|p| {
                    if templates_set.contains(p) {
                        format!("{} (已存在)", p)
                    } else {
                        p.to_string()
                    }
                });
                let unique_iter = templates_all
                    .par_iter()
                    .filter(|t| !part_set.contains(t))
                    .map(|t| t.to_string());
                let mut merged_vec: Vec<String> = part_iter.chain(unique_iter).collect();
                merged_vec.sort();
                dropdown.set_labels(cx, merged_vec);
            }
        }
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for AddTemplateNameModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let accept_button = self.button(ids!(accept_button));
        let cancel_button = self.button(ids!(cancel_button));
        let template_select = self.drop_down(ids!(template_select));
        if cancel_button.clicked(actions) {
            Cx::post_action(TemplateNameModalAction::Close);
        }
        if accept_button.clicked(actions) {
            if template_select.selected_label().is_empty() {
                self.label(ids!(error)).set_text(cx, "选择框不能为空!!");
            } else if template_select.selected_label().contains(" (已存在)") {
                self.label(ids!(error))
                    .set_text(cx, "模板已存在该型号里面,无法进行重复添加!!");
            } else {
                let template_name = template_select.selected_label();
                Cx::post_action(TemplateNameModalAction::Action(template_name));
            }
        }
    }
}
