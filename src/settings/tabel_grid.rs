use std::collections::HashMap;

use makepad_widgets::*;

use crate::store::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    TableItem = <RoundedView> {
        width: Fit,
        height: Fit,
        spacing:20,
        align: {y:0.5}
        flow: Right
        <Label> {
            text: "表格位置:"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 14
                }
                color: #000
            }
        }
        tabel_key = <MolyTextInput> {
            empty_text: "例如:A1"
            width: 80, height: 40
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
        <Label> {
            text: "表格内容:"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 14
                }
                color: #000
            }
        }
        table_value = <MolyTextInput> {
            empty_text: "例如:Deta或者自定义.."
            width: 180, height: 40
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
    TableRow = {{TableRow}} {
        table_row = <PortalList> {
            height: Fit,
            flow: Right,
            spacing:20,
            TableItem = <TableItem> {
                cursor: Default
            }
        }
    }
    pub TableGrid = {{TableGrid}} {
        table_grid = <PortalList> {
            height: Fill
            flow: Down,
            spacing:50
            TableRow = <TableRow> {}
        }
    }
}
pub struct TableRowProps {
    pub props: usize,
}
#[derive(Live, LiveHook, Widget)]
pub struct TableRow {
    #[deref]
    view: View,
    #[rust]
    key_ids: HashMap<WidgetUid, String>,
    #[rust]
    value_ids: HashMap<WidgetUid, String>,
}
impl Widget for TableRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(store) = scope.data.get::<Store>() {
                    list.set_item_range(cx, 0, 2);
                    let mut keys = store
                        .setting_store
                        .template_infos
                        .tables
                        .iter()
                        .map(|(key, _)| {
                            let k = key.clone();
                            k
                        })
                        .collect::<Vec<_>>();
                    keys.sort();
                    let keys_len = keys.len();
                    let values = store.setting_store.template_infos.tables.clone();
                    let props = scope.props.get::<TableRowProps>().unwrap();
                    let row_idx = props.props;
                    let first_idx = row_idx * 2;
                    for i in 0..2 {
                        let global_idx = first_idx + i;
                        let item = list.item(cx, i, live_id!(TableItem));
                        let tabel_key_input = item.text_input(ids!(tabel_key));
                        let table_value_input = item.text_input(ids!(table_value));
                        let key_id = tabel_key_input.widget_uid();
                        let value_id = table_value_input.widget_uid();

                        if global_idx < keys_len {
                            let key = &keys[global_idx];
                            let store_value: String = values.get(key).map_or("", |v| v).to_string();
                            tabel_key_input.set_text(cx, &key);
                            table_value_input.set_text(cx, &store_value);
                            self.key_ids.insert(key_id, key.clone());
                            self.value_ids.insert(value_id, store_value.to_string());
                        } else {
                            tabel_key_input.set_text(cx, "");
                            table_value_input.set_text(cx, "");
                            self.key_ids.insert(key_id, String::new());
                            self.value_ids.insert(value_id, String::new());
                        }
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
impl WidgetMatchEvent for TableRow {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(ids!(table_row));
        for (_, item_widget) in list_widget.items_with_actions(actions) {
            let tabel_key_input = item_widget.text_input(ids!(tabel_key));
            let table_value_input = item_widget.text_input(ids!(table_value));

            if let Some(new_key) = tabel_key_input.changed(actions) {
                let id = tabel_key_input.widget_uid();
                if let Some(old_key) = self.key_ids.get(&id).cloned() {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        let mut value_to_insert = String::new();
                        // 移除旧键，并获取对应的值
                        if !old_key.is_empty() {
                            if let Some(removed_value) =
                                store.setting_store.template_infos.tables.remove(&old_key)
                            {
                                value_to_insert = removed_value;
                            }
                        }
                        // 插入新键和值
                        store
                            .setting_store
                            .template_infos
                            .tables
                            .insert(new_key.clone(), value_to_insert);

                        // 同步更新 key_ids 和 value_ids 中的键
                        self.key_ids.insert(id, new_key.clone());
                        let value_id = table_value_input.widget_uid();
                        self.value_ids.insert(value_id, new_key.clone());

                        cx.redraw_all();
                    }
                }
            }

            if let Some(new_value) = table_value_input.changed(actions) {
                let id = table_value_input.widget_uid();
                if let Some(key) = self.value_ids.get(&id) {
                    if !key.is_empty() {
                        if let Some(store) = scope.data.get_mut::<Store>() {
                            if let Some(value) =
                                store.setting_store.template_infos.tables.get_mut(key)
                            {
                                *value = new_value;
                                cx.redraw_all();
                            }
                        }
                    }
                }
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct TableGrid {
    #[deref]
    view: View,
}

impl Widget for TableGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    let len = 2;
                    list.set_item_range(cx, 0, len);
                    while let Some(row_idx) = list.next_visible_item(cx) {
                        if row_idx >= len {
                            continue;
                        }

                        let row = list.item(cx, row_idx, live_id!(TableRow));
                        let props = TableRowProps { props: row_idx };
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
