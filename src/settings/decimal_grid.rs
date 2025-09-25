use std::collections::HashMap;

use makepad_widgets::{event::TriggerEvent, *};

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
                // 使用固定的keys数组
                let keys = vec![
                    "ith", "vf", "im", "po", "rs", "se", "icc", "kink", "imkink", "res", "sen",
                    "vbr", "i_xtalk", "mdpid", "idark","deltaP（dB）","mdpid"
                ];
                let keys_len = keys.len();
                let props = scope.props.get::<DecimalRowProps>().unwrap();
                let row_idx = props.props;
                let first_idx = row_idx * 10;

                // 始终渲染10个项，但要考虑到keys数组的实际长度
                let num_to_render = 10.min(keys_len.saturating_sub(first_idx));
                list.set_item_range(cx, 0, num_to_render);

                for i in 0..num_to_render {
                    if i >= keys_len {
                        break;
                    }
                    let global_idx = first_idx + i;

                    let key = keys[global_idx];
                    let item = list.item(cx, i, live_id!(DecimalItem));
                    let label_name = item.label(id!(decimal_name));
                    let widget_id = item.text_input(id!(decimal_input)).widget_uid();
                    let value = key.to_string();
                    self.ids.insert(widget_id, value);
                    // 设置标签文本
                    label_name.set_text(cx, key);
                    item.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if let Event::Trigger(trigger_event) = event {
            if let Some(state) = scope.data.get_mut::<Store>() {
                // 遍历所有 triggers
                for (_area, triggers) in &trigger_event.triggers {
                    for trigger in triggers {
                        if trigger.id == live_id!(update_decimal_inputs) {
                            let keys = vec![
                                "ith", "vf", "im", "po", "rs", "se", "iop", "kink", "imkink",
                                "res", "sen", "icc", "i_xtalk", "mdpid", "idark",
                            ];
                            let keys_len = keys.len();

                            if let Some(props) = scope.props.get::<DecimalRowProps>() {
                                let row_idx = props.props;
                                let first_idx = row_idx * 10;
                                let num_to_render = 10.min(keys_len.saturating_sub(first_idx));

                                let list_widget = self.view.portal_list(id!(decimal_row));
                                for i in 0..num_to_render {
                                    if i >= keys_len {
                                        break;
                                    }
                                    let global_idx = first_idx + i;
                                    let key = keys[global_idx];

                                    let item = list_widget.item(cx, i, live_id!(DecimalItem));
                                    let decimal_input = item.text_input(id!(decimal_input));
                                    let store_value = state
                                        .template_infos
                                        .numbers
                                        .get(key)
                                        .map(|v| v.to_string())
                                        .unwrap_or_default();
                                    decimal_input.set_text(cx, &store_value);
                                }
                            }
                        }
                    }
                }
            }
        }
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
                        if let Some(value) = store.template_infos.numbers.get_mut(i) {
                            *value = input.parse::<i64>().unwrap_or_default();
                        } else {
                            store
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
}

impl Widget for DecimalGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // 使用一个固定的keys数组来决定渲染的结构
                let keys = vec![
                    "ith", "vf", "im", "po", "rs", "se", "iop", "kink", "imkink", "res", "sen",
                    "icc", "i_xtalk", "mdpid", "idark",
                ];
                let keys_len = keys.len();

                // 根据固定keys数组的长度来计算行数，而不是state中的数据长度
                let len = keys_len.div_ceil(10);
                list.set_item_range(cx, 0, len);

                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= len {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(DecimalRow));
                    let props = DecimalRowProps { props: row_idx };
                    let mut scope = Scope::with_props(&props);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if let Event::Trigger(trigger_event) = event {
            if let Some(store) = scope.data.get_mut::<Store>() {
                let grid_area = store.grid_area;
                if let Some(triggers) = trigger_event.triggers.get(&grid_area) {
                    for trigger in triggers {
                        if trigger.id == live_id!(update_decimal_inputs) {
                            let keys = vec![
                                "ith", "vf", "im", "po", "rs", "se", "iop", "kink", "imkink",
                                "res", "sen", "icc", "i_xtalk", "mdpid", "idark",
                            ];
                            let keys_len = keys.len();
                            let len = keys_len.div_ceil(10);

                            for row_idx in 0..len {
                                let row = self.view.portal_list(id!(decimal_grid)).item(
                                    cx,
                                    row_idx,
                                    live_id!(DecimalRow),
                                );
                                let props = DecimalRowProps { props: row_idx };
                                let mut scope = Scope::with_data_props(store, &props);
                                row.handle_event(
                                    cx,
                                    &Event::Trigger(TriggerEvent {
                                        triggers: [(row.area(), triggers.clone())]
                                            .into_iter()
                                            .collect(),
                                    }),
                                    &mut scope,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
