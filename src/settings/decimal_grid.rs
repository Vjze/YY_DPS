use std::collections::HashMap;

use makepad_widgets::*;
use tracing::info;

use crate::{configs::decimal_config::InfoDetail, store::Store};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    Post = <View> {
        width: Fill,
        height: Fit,
        spacing:10,

        body = <RoundedView> {
            width: Fill, height: Fit
            content = <View> {
                width: Fill, height: Fit
                align: {y: 0.5}
                spacing: 10,
                title = <Label> {
                    width: Fill { weight: 0.3}
                    text: "..."
                    draw_text: {
                        text_style: <REGULAR_FONT>{
                            font_size: 12
                        }
                        color: #000
                    }
                }
                roudan_bool = <CheckBox> {
                    width: Fill { weight: 0.5}
                    text: "启用随机"
                    draw_text: {
                        text_style: <REGULAR_FONT>{
                            font_size: 12
                        }
                        color: #000
                    }
                }
                roudan_view = <View> {
                    width: Fill { weight: 2.1}
                    spacing: 10,
                    align: {y: 0.5}
                    <Label> {
                        text: "下限:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    roudan_min_input = <TextInput> {
                        empty_text: "..."
                        width: Fill { weight: 0.5}, height: 40
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
                    }
                    <Label> {
                        text: "上限:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    roudan_max_input = <TextInput> {
                        empty_text: "..."
                        width: Fill { weight: 0.5}, height: 40
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
                    }
                    <Label> {
                        text: "波动范围:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    roudan_size_input = <TextInput> {
                        empty_text: "..."
                        width: Fill { weight: 0.5}, height: 40
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
                    }
                }
                data_type = <DropDown> {
                        width: Fill { weight: 0.5},height:40
                        labels:["数据","固定内容","留空"],
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
                        }
                        popup_menu: <PopupMenu> {
                            draw_bg: { uniform color: #333, uniform border_color: #666,
                                uniform border_size: 1.0 }
                            menu_item: <PopupMenuItem> {
                                padding: {left: 20, top: 8, bottom: 8, right: 10}
                                draw_bg: {
                                    color: #333
                                    color_hover: #555
                                    color_active: #888
                                }
                                draw_text: {
                                    color: #EEE
                                    color_hover: #FFF
                                    color_active: #FFF
                                }
                            }
                        }
                    }

                        data_select = <DropDown> {
                            width: Fill { weight: 0.7},height:40,
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
                            }
                            popup_menu: <PopupMenu> {
                                draw_bg: { uniform color: #333, uniform border_color: #666,
                                    uniform border_size: 1.0 }
                                menu_item: <PopupMenuItem> {
                                    padding: {left: 20, top: 8, bottom: 8, right: 10}
                                }
                            }
                        }
                    <Label> {
                        text: "固定内容:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }

                    fixed_content = <TextInput> {
                        empty_text: "..."
                        width: Fill {weight: 0.8}, height: 40,
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
                        }
                // }
                <Label> {
                        text: "小数点:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                decimal_input = <TextInput> {
                    empty_text: "..."
                    width: Fit, height: 40,
                    padding: 10,
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                }
            }
        }
    }
    pub TemplateInfosRow = {{TemplateInfosRow}} {
        list = <PortalList> {
            keep_invisible: true
            scroll_bar: <ScrollBar> {}
            Post = <CachedView>{
                flow: Down,
                <Post> {}
                <Hr> {}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TemplateInfosRow {
    #[deref]
    view: View,
    #[rust]
    ids: HashMap<WidgetUid, String>,
}

impl Widget for TemplateInfosRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let state = scope.data.get_mut::<Store>().unwrap();
        let infos = state.setting_store.template_infos.infos.clone();
        let mut keys = infos.keys().cloned().collect::<Vec<String>>();
        keys.sort();
        let len = state.setting_store.template_infos.infos.len();
        let len = if len > 0 { len } else { 0 };
        let labels = state.setting_store.all_column_name.clone();
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // if let Some(state) = scope.data.get_mut::<Store>() {

                list.set_item_range(cx, 0, len);
                // if state.setting_store.template_infos.infos.is_empty() {
                //     continue;
                // }
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < len {
                        // if item_id >= len {
                        //     continue;
                        // }
                        let item = list.item(cx, item_id, live_id!(Post));

                        let title = keys[item_id].clone();

                        if let Some(info) = infos.get(&title) {
                            if title == "ith" {
                                info!("info {:?}", info);
                            }

                            let roudan = info.roudan;

                            item.label(ids!(title)).set_text(cx, &title);

                            item.check_box(ids!(roudan_bool)).set_active(cx, roudan);
                            self.ids.insert(
                                item.check_box(ids!(roudan_bool)).widget_uid(),
                                title.clone(),
                            );

                            if roudan {
                                item.text_input(ids!(roudan_min_input))
                                    .set_disabled(cx, false);
                                item.text_input(ids!(roudan_max_input))
                                    .set_disabled(cx, false);
                                item.text_input(ids!(roudan_size_input))
                                    .set_disabled(cx, false);
                                item.text_input(ids!(roudan_min_input))
                                    .set_text(cx, &info.roudan_min);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_min_input)).widget_uid(),
                                    title.clone(),
                                );

                                item.text_input(ids!(roudan_max_input))
                                    .set_text(cx, &info.roudan_max);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_max_input)).widget_uid(),
                                    title.clone(),
                                );

                                item.text_input(ids!(roudan_size_input))
                                    .set_text(cx, &info.roudan_size);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_size_input)).widget_uid(),
                                    title.clone(),
                                );
                            } else {
                                item.text_input(ids!(roudan_min_input))
                                    .set_disabled(cx, true);
                                item.text_input(ids!(roudan_max_input))
                                    .set_disabled(cx, true);
                                item.text_input(ids!(roudan_size_input))
                                    .set_disabled(cx, true);
                            }

                            item.drop_down(ids!(data_type))
                                .set_selected_by_label(&info.data_type, cx);
                            self.ids.insert(
                                item.drop_down(ids!(data_type)).widget_uid(),
                                title.clone(),
                            );

                            if item.drop_down(ids!(data_type)).selected_label() == "数据" {
                                item.drop_down(ids!(data_select)).set_disabled(cx, false);

                                item.drop_down(ids!(data_select))
                                    .set_labels(cx, labels.clone());
                                item.drop_down(ids!(data_select))
                                    .set_selected_by_label(&info.data_select, cx);
                                item.text_input(ids!(fixed_content)).set_disabled(cx, true);
                                self.ids.insert(
                                    item.drop_down(ids!(data_select)).widget_uid(),
                                    title.clone(),
                                );
                            } else {
                                item.drop_down(ids!(data_select)).set_disabled(cx, true);
                            }

                            item.text_input(ids!(fixed_content))
                                .set_text(cx, &info.fixed_content);
                            self.ids.insert(
                                item.text_input(ids!(fixed_content)).widget_uid(),
                                title.clone(),
                            );

                            item.text_input(ids!(decimal_input))
                                .set_text(cx, &info.decimal);
                            self.ids.insert(
                                item.text_input(ids!(decimal_input)).widget_uid(),
                                title.clone(),
                            );
                            item.draw_all(cx, scope);
                        }
                        
                        // item.draw_all(cx, &mut Scope::empty());
                    }
                }
                // }
            }
        }
        // self.view.draw_walk(cx, scope, walk)
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }
}

impl WidgetMatchEvent for TemplateInfosRow {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(ids!(list));

        for (_item_id, item_widget) in list_widget.items_with_actions(actions) {
            let mut update_data =
                |cx: &mut Cx, widget_uid: WidgetUid, update_fn: &mut dyn FnMut(&mut InfoDetail)| {
                    if let Some(key) = self.ids.get(&widget_uid) {
                        if let Some(state) = scope.data.get_mut::<Store>() {
                            if let Some(data) =
                                state.setting_store.template_infos.infos.get_mut(key)
                            {
                                update_fn(data);
                                cx.redraw_all();
                            }
                        }
                    }
                };

            let roudan_bool = item_widget.check_box(ids!(roudan_bool));
            let roudan_min_input = item_widget.text_input(ids!(roudan_min_input));
            let roudan_max_input = item_widget.text_input(ids!(roudan_max_input));
            let roudan_size_input = item_widget.text_input(ids!(roudan_size_input));
            let data_type = item_widget.drop_down(ids!(data_type));
            let data_select = item_widget.drop_down(ids!(data_select));
            let fixed_content = item_widget.text_input(ids!(fixed_content));
            let decimal_input = item_widget.text_input(ids!(decimal_input));

            // ------------------ 启用随机 (roudan_bool) ------------------
            if let Some(active) = roudan_bool.changed(actions) {
                update_data(cx, roudan_bool.widget_uid(), &mut |data| {
                    info!("roudan_before {:?}", data);
                    data.roudan = active;
                    info!("roudan_after {:?}", data);
                });
            }
            // ------------------ 随机下限 (roudan_min_input) ------------------
            if let Some(roudan_min_input_text_val) = roudan_min_input.changed(actions) {
                update_data(cx, roudan_min_input.widget_uid(), &mut |data| {
                    data.roudan_min = roudan_min_input_text_val.clone();
                });
            }

            // ------------------ 随机上限 (roudan_max_input) ------------------
            if let Some(roudan_max_input_text_val) = roudan_max_input.changed(actions) {
                update_data(cx, roudan_max_input.widget_uid(), &mut |data| {
                    data.roudan_max = roudan_max_input_text_val.clone();
                });
            }

            // ------------------ 随机粒度 (roudan_size_input) ------------------
            if let Some(roudan_size_input_text_val) = roudan_size_input.changed(actions) {
                update_data(cx, roudan_size_input.widget_uid(), &mut |data| {
                    data.roudan_size = roudan_size_input_text_val.clone();
                });
            }

            // ------------------ 数据类型 (data_type) ------------------
            if let Some(data_type_selected) = data_type.changed_label(actions) {
                update_data(cx, data_type.widget_uid(), &mut |data| {
                    data.data_type = data_type_selected.clone();
                });
            }

            // ------------------ 数据选择 (data_select) ------------------
            if let Some(data_select_selected) = data_select.changed_label(actions) {
                update_data(cx, data_select.widget_uid(), &mut |data| {
                    data.data_select = data_select_selected.clone();
                });
            }

            // ------------------ 固定内容 (fixed_content) ------------------
            if let Some(fixed_content_text_val) = fixed_content.changed(actions) {
                update_data(cx, fixed_content.widget_uid(), &mut |data| {
                    info!("data_before {:?}", data);
                    data.fixed_content = fixed_content_text_val.clone();
                    info!("data_after {:?}", data);
                });
            }

            // ------------------ 小数位 (decimal_input) ------------------
            if let Some(decimal_input_text_val) = decimal_input.changed(actions) {
                update_data(cx, decimal_input.widget_uid(), &mut |data| {
                    data.decimal = decimal_input_text_val.clone();
                });
            }
        }
    }
}
