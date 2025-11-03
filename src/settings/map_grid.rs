use std::collections::HashMap;

use makepad_widgets::*;

use crate::store::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    MapItem = <RoundedView> {
        width: Fit,
        height: Fit,
        spacing:10,
        align: {x:0.5, y:0.5},
        flow: Right
        map_name = <Label> {
            width: 140,
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        map_selector = <DropDownFlat> {
            width: 160,height:40
            // labels:["template_1","template_2","template_3"],
            labels: [""],
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
    MapRow = {{MapRow}} {
        map_row =  <PortalList> {
            height: Fit,
            flow: Right,
            spacing:30,
            MapItem = <MapItem> {
                cursor: Default
            }
        }
    }
    pub MapGrid = {{MapGrid}} {
        width: Fill,
        height: Fill,
        map_grid = <PortalList> {
            width: Fill
            height: Fill
            flow: Down,

            MapRow = <MapRow> {}
        }
    }
}
pub struct MapRowProps {
    pub props: usize,
}
#[derive(Live, LiveHook, Widget)]
pub struct MapRow {
    #[deref]
    view: View,
    #[rust]
    ids: HashMap<WidgetUid, String>,
}
impl Widget for MapRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(store) = scope.data.get::<Store>() {
                    // 动态设置项范围为 numbers 的长度
                    let keys_len = store.setting_store.map_infos.len();
                    let props = scope.props.get::<MapRowProps>().unwrap();
                    let row_idx = props.props;
                    let first_idx = row_idx * 3;
                    let num_to_render = 3.min(keys_len.saturating_sub(first_idx));
                    let all_column_name = store.setting_store.all_column_name.clone();
                    list.set_item_range(cx, 0, num_to_render);
                    // 迭代 numbers 的键值对
                    let mut keys = store
                        .setting_store
                        .map_infos
                        .iter()
                        .map(|(key, _)| {
                            let k = key.clone();
                            k
                        })
                        .collect::<Vec<_>>();
                    keys.sort(); // 可选：按键排序以确保一致的显示顺序
                    let values = store.setting_store.map_infos.clone();
                    for i in 0..num_to_render {
                        let global_idx = first_idx + i;
                        if global_idx >= keys_len {
                            break;
                        }
                        let key = &keys[global_idx]; // 获取正确全局索引的 key
                        // 使用行内索引 i 来创建项
                        let item = list.item(cx, i, live_id!(MapItem));
                        let widget_id = item.drop_down(ids!(map_selector)).widget_uid();
                        let map_name = item.label(ids!(map_name));
                        let map_selector = item.drop_down(ids!(map_selector));
                        let store_value = values.get(key).clone().unwrap();
                        self.ids.insert(widget_id, key.clone());
                        map_name.set_text(cx, &key);
                        map_selector.set_labels(cx, all_column_name.clone());
                        map_selector.set_selected_by_label(&store_value, cx);
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
        cx.redraw_all();
    }
}
impl WidgetMatchEvent for MapRow {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(ids!(map_row));
        for (_, item_widget) in list_widget.items_with_actions(actions) {
            let map_selector = item_widget.drop_down(ids!(map_selector));
            if let Some(selected) = map_selector.changed_label(actions) {
                let id = map_selector.widget_uid();
                if let Some(i) = self.ids.get(&id) {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        if let Some(value) = store.setting_store.map_infos.get_mut(i) {
                            *value = selected.clone();
                        }
                        println!("datas = {:?}", store.setting_store.map_infos);
                    }
                }
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct MapGrid {
    #[deref]
    view: View,
}

impl Widget for MapGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    // info!("Drawing MapGrid with map_infos: {:?}", store.setting_store.map_infos);
                    let len = store.setting_store.map_infos.len().div_ceil(3);
                    list.set_item_range(cx, 0, len);
                    while let Some(row_idx) = list.next_visible_item(cx) {
                        if row_idx >= len {
                            continue;
                        }

                        let row = list.item(cx, row_idx, live_id!(MapRow));
                        let props = MapRowProps { props: row_idx };
                        let mut scope = Scope::with_data_props(store, &props);
                        row.draw_all(cx, &mut scope);
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
