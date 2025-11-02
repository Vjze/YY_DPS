use std::collections::HashMap;

use makepad_widgets::*;

use crate::store::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    FixedItem = <RoundedView> {
        width: Fit,
        height: Fit,
        spacing:10,
        align: {x:0.5, y:0.5},
        flow: Right
        fixed_name = <Label> {
            width: 110,
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        fixed_input = <MolyTextInput> {
            empty_text: "输入固定内容...."
            width: 200, height: 40
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
                uniform color: #000
            }
        }
    }
    FixedRow = {{FixedRow}} {
        fixed_row =  <PortalList> {
            height: Fit,
            flow: Right,
            spacing:30,
            FixedItem = <FixedItem> {
                cursor: Default
            }
        }
    }
    pub FixedGrid = {{FixedGrid}} {
        width: Fill,
        height: Fill,
        fixed_grid = <PortalList> {
            width: Fill
            height: Fill
            flow: Down,

            FixedRow = <FixedRow> {}
        }
    }
}
pub struct FixedRowProps {
    pub props: usize,
}
#[derive(Live, LiveHook, Widget)]
pub struct FixedRow {
    #[deref]
    view: View,
    #[rust]
    ids: HashMap<WidgetUid, String>,
}
impl Widget for FixedRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(state) = scope.data.get::<Store>() {
                    // 动态设置项范围为 numbers 的长度
                    let keys_len = state.setting_store.template_infos.strings.len();
                    let props = scope.props.get::<FixedRowProps>().unwrap();
                    let row_idx = props.props;
                    let first_idx = row_idx * 3;
                    let num_to_render = 3.min(keys_len.saturating_sub(first_idx));

                    list.set_item_range(cx, 0, num_to_render);
                    // 迭代 numbers 的键值对
                    let mut keys = state.setting_store
                        .template_infos
                        .strings
                        .iter()
                        .map(|(key, _)| {
                            let k = key.clone();
                            k
                        })
                        .collect::<Vec<_>>();
                    keys.sort(); // 可选：按键排序以确保一致的显示顺序
                    let values = state.setting_store.template_infos.strings.clone();
                    for i in 0..num_to_render {
                        let global_idx = first_idx + i;
                        if global_idx >= keys_len {
                            break;
                        }
                        let key = &keys[global_idx]; // 获取正确全局索引的 key
                        // 使用行内索引 i 来创建项
                        let item = list.item(cx, i, live_id!(FixedItem));
                        let widget_id = item.text_input(id!(fixed_input)).widget_uid();
                        let fixed_name = item.label(id!(fixed_name));
                        self.ids.insert(widget_id, key.clone());
                        fixed_name.set_text(cx, &key);
                        let fixed_input = item.text_input(id!(fixed_input));
                        let store_value = values.get(&key.clone())
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        fixed_input.set_text(cx, &store_value);
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
impl WidgetMatchEvent for FixedRow {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(id!(fixed_row));
        for (_, item_widget) in list_widget.items_with_actions(actions) {
            let text_input = item_widget.text_input(id!(fixed_input));
            if let Some(input) = text_input.changed(actions) {
                let id = text_input.widget_uid();
                if let Some(i) = self.ids.get(&id) {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        if let Some(value) = store.setting_store.template_infos.strings.get_mut(i) {
                            *value = input;
                        }
                    }
                }
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct FixedGrid {
    #[deref]
    view: View,
}

impl Widget for FixedGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<Store>().unwrap();
                let len = state.setting_store.template_infos.strings.len().div_ceil(3);
                list.set_item_range(cx, 0, len);
                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= len {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(FixedRow));
                    let props = FixedRowProps { props: row_idx };
                    let mut scope = Scope::with_data_props(state, &props);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
