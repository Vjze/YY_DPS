use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{configs::type_config::get_type_infos, store::Store, utils::error::MyError};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    EXCEL_ICON = dep("crate://self/resources/images/excel.png");

    TemplateItems = <RoundedView> {
        margin: {right:25}
        padding: 15,
        width: 256,
        height: 256,
        align: {x:0.5}
        flow: Down,
        show_bg: true
        draw_bg: {
            color: #0003,
            border_radius: 8.5,
            // uniform shadow_color: #0003
            // shadow_radius: 18.0,
            // shadow_offset: vec2(0.0,-1.5)
        }
        template_name = <Label> {
            text: "test"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        <Image> {
            width: 160, height: 160,
            source: (EXCEL_ICON),
            // align: {x:0.5}
        }
        del_btn = <Button> {
            width: 150, height: 45,
            padding: 0,
            text: "删除",
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
            draw_bg: {
                color: (MAIN_BG_COLOR_DARK),
                uniform border_radius: 10.0,
                uniform border_size: 1.0,
                uniform border_color_1: #333
            }
        }
    }
    TemplateItemsRow = {{TemplateItemsRow}} {
        list =<PortalList> {
            height: 256,
            flow: Right,
            TemplateItems = <TemplateItems> {
                cursor: Default
            }
        }
    }


    pub TypeView = {{TypeView}}
        <RoundedShadowView> {
            width: Fill, height: Fill
            align: {x: 0.0, y: 0.0}
            padding: {left: 15, right: 15, bottom: 15, top:15}
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

                <View> {
                    width: Fill, height: Fit,
                    spacing:15,
                    type_input = <MolyTextInput> {
                        empty_text: "新增型号必须输入名称...."
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
                    add_type_btn = <Button> {
                        text: "新增型号"
                        width: 100, height: 40,
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
                    <Labelbold> {
                        padding:6,
                        text: "选择类型:"
                        draw_text: {
                            text_style: {
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    type_selector = <DropDown> {
                        width: 200,height:40
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
                }
                t_row = <TemplateItemsRow> {}
                <View> {
                    align: {x: 0.5}
                    add_template_btn = <Button> {
                        width: 120, height: 40,
                        text: "添加模板",
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 15,
                                
                            }
                            color: #000
                        }
                        draw_bg: {
                            color: #ADD8E6,
                            uniform border_radius: 5.0,
                            uniform border_size: 1.0,
                            uniform border_color_1: #333
                        }
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
    #[rust(Runtime::new().unwrap())]
    rt: Runtime
}

impl Widget for TypeView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get_mut::<Store>() {
            if !store.types.is_empty() {
                self.view.drop_down(id!(type_selector)).set_labels(cx, store.types.clone());
            }
        }
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for TypeView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let select = self.view.drop_down(id!(type_selector));
        let input = self.view.text_input(id!(type_input));
        if let Some(s) = select.changed_label(actions){
            input.set_text(cx, &s);
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            let type_infos = rt.block_on(async move{
                let res = get_type_infos(input.text().clone()).await;
                match res {
                    Ok(res) => {
                        res
                    },
                    Err(e) => {
                        Cx::post_action(e);
                        Vec::new()
                    },
                }
            });
            if !type_infos.is_empty() {
                
                if let Some(store) = scope.data.get_mut::<Store>(){
                    store.set_type_infos(type_infos)
                }
                
            }
        }
        let list_widget = self.view.portal_list(id!(list));
        for(item_id, item_widget)  in list_widget.items_with_actions(actions) {
            if item_widget.button(id!(del_btn)).clicked(actions) {
                if let Some(store) = scope.data.get_mut::<Store>() {
                   store.type_infos.remove(item_id);
                }
            }
        }
        cx.redraw_all();
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TemplateItemsRow {
    #[deref]
    view: View,
}

impl Widget for TemplateItemsRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<Store>().unwrap();
                list.set_item_range(cx, 0, state.type_infos.len());
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.type_infos.len() {
                        continue;
                    }
                    let item = list.item(cx, item_idx, live_id!(TemplateItems));
                    let label_name = item.label(id!(template_name));
                    let t_name = state.type_infos.get(item_idx).unwrap();
                    label_name.set_text(cx, &t_name);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}