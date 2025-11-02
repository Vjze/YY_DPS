use std::collections::HashMap;

use makepad_widgets::*;

use crate::store::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    DecimalItem = <RoundedView> {
        width: 80,
        height: Fit,
        align: {x:0.5}
        flow: Down
        decimal_input = <MolyTextInput> {
            empty_text: "..."
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
                uniform color: #000
            }
        }
        decimal_name = <Label> {
             draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
    }
    DecimalRow = {{DecimalRow}} {
        decimal_row = <PortalList> {
            spacing: 25,
            height: Fit,
            flow: Right,
            DecimalItem = <DecimalItem> {
                cursor: Default
            }
        }
    }
    pub DecimalGrid = {{DecimalGrid}} {
        decimal_grid = <PortalList> {
            height: Fill,
            flow: Down,
            DecimalRow = <DecimalRow> {}
        }
    }
}
pub struct DecimalRowProps {
    pub props: usize,
}

#[derive(Live, LiveHook, Widget)]
pub struct DecimalRow {
    #[deref]
    view: View,
    #[rust]
    ids: HashMap<WidgetUid, String>,
}
impl Widget for DecimalRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
        if let Some(mut list) = item.as_portal_list().borrow_mut() {
             if let Some(state) = scope.data.get_mut::<Store>() {
            let keys = vec![
                "ith", "vf", "im", "po", "rs", "se", "icc", "kink",
                "imkink", "res", "sen", "vbr", "i_xtalk", "mdpid",
                "idark", "deltaP（dB）", "mdpid",
            ];
            let keys_len = keys.len();

            let props = scope.props.get::<DecimalRowProps>().unwrap();
            let row_idx = props.props;
            let first_idx = row_idx * 10;

            let num_to_render = 10.min(keys_len.saturating_sub(first_idx));
            list.set_item_range(cx, 0, num_to_render);

            // --- THE FIX ---
            // 1. 在循环外获取数据并克隆，立即释放对 `scope` 的不可变借用。
            //    `values` 现在是一个拥有的 HashMap (如果 store 存在的话)。
            let values = state.setting_store.template_infos.numbers.clone();
            // 循环开始时，对 `scope` 的不可变借用已经结束。

            for i in 0..num_to_render {
                if i >= keys_len { break; }
                let global_idx = first_idx + i;

                let key = keys[global_idx];
                let item_widget = list.item(cx, i, live_id!(DecimalItem));
                
                let label_name = item_widget.label(id!(decimal_name));
                let input_widget = item_widget.text_input(id!(decimal_input));
                let widget_id = input_widget.widget_uid();
                
                self.ids.insert(widget_id, key.to_string());

                // 2. 无条件设置标签
                label_name.set_text(cx, key);
                let v = values.get(key).unwrap_or(&0);
                // 3. 使用克隆出来的数据来设置输入框
                // if let Some(ref local_values) = values {
                //     if let Some(num) = local_values.get(key) {
                        input_widget.set_text(cx, &v.to_string());
                //     } else {
                //         input_widget.set_text(cx, "");
                //     }
                // } else {
                //     // 如果 store 本身就不存在
                //     input_widget.set_text(cx, "");
                // }
                
                // 4. 现在可以安全地可变借用 `scope`，因为没有其他借用存在。
                item_widget.draw_all(cx, scope);
            }}
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
impl WidgetMatchEvent for DecimalRow {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(id!(decimal_row));
        for (_, item_widget) in list_widget.items_with_actions(actions) {
            let text_input = item_widget.text_input(id!(decimal_input));
            if let Some(input) = text_input.changed(actions) {
                let id = text_input.widget_uid();
                if let Some(i) = self.ids.get(&id) {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        if let Some(value) = store.setting_store.template_infos.numbers.get_mut(i) {
                            *value = input.parse::<i64>().unwrap_or_default();
                        } else {
                            store.setting_store
                                .template_infos
                                .numbers
                                .insert(i.clone(), input.parse::<i64>().unwrap_or_default());
                        }
                    }
                }
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct DecimalGrid {
    #[deref]
    view: View,
    #[rust]
    data: HashMap<String, i64>,
}

impl Widget for DecimalGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
        if let Some(mut list) = item.as_portal_list().borrow_mut() {
            // 移除对 store 的检查，因为网格结构不依赖于 store 中的数据
            let keys = vec![
                "ith", "vf", "im", "po", "rs", "se", "icc", "kink",
                "imkink", "res", "sen", "vbr", "i_xtalk", "mdpid",
                "idark", "deltaP（dB）", "mdpid",
            ];
            let keys_len = keys.len();

            // 根据固定 keys 数组的长度来计算行数
            let len = keys_len.div_ceil(10);
            list.set_item_range(cx, 0, len);
            let data = scope.data.get_mut::<Store>().unwrap();
            while let Some(row_idx) = list.next_visible_item(cx) {
                if row_idx >= len {
                    continue;
                }

                let row = list.item(cx, row_idx, live_id!(DecimalRow));
                let props = DecimalRowProps { props: row_idx };
                let mut scope = Scope::with_data_props(data, &props);
                row.draw_all(cx, &mut scope);
            }
        }
    }
    DrawStep::done()
}

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get::<Store>() {
            if !store.setting_store.template_infos.numbers.is_empty() {
                self.data = store.setting_store.template_infos.numbers.clone()
            }
        }
        self.view.handle_event(cx, event, scope);
    }
}
