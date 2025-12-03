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
                    roudan_min_input = <MolyTextInput> {
                        empty_text: "..."
                        width: Fit, height: 40
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
                    roudan_max_input = <MolyTextInput> {
                        empty_text: "..."
                        width: Fit, height: 40
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
                    roudan_size_input = <MolyTextInput> {
                        empty_text: "..."
                        width: Fit, height: 40
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
                            width: Fill { weight: 1.0},height:40,
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
                                    // draw_bg: {
                                    //     color: #333
                                    //     color_hover: #555
                                    //     color_active: #888
                                    // }
                                    // draw_text: {
                                    //     color: #EEE
                                    //     color_hover: #FFF
                                    //     color_active: #FFF
                                    // }
                                }
                            }
                        }

                // fixed_view = <View> {
                    // visible: false,
                    // spacing:10,
                    // width: Fill {weight: 1.5},
                    // align: {y: 0.5}
                    <Label> {
                        text: "固定内容:"
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }

                    fixed_content = <MolyTextInput> {
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
                decimal_input = <MolyTextInput> {
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
            scroll_bar: <ScrollBar> {}
            TopSpace = <View> {height: 0.}
            BottomSpace = <View> {height: 100.}

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
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(state) = scope.data.get_mut::<Store>() {
                    let mut keys = state
                        .setting_store
                        .template_infos
                        .infos
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>();
                    keys.sort();
                    let len = state.setting_store.template_infos.infos.len();
                    list.set_item_range(cx, 0, len);
                    if state.setting_store.template_infos.infos.is_empty() {
                        continue;
                    }
                    while let Some(item_id) = list.next_visible_item(cx) {
                        let template = match item_id {
                            0 => live_id!(TopSpace),
                            _ => live_id!(Post),
                        };
                        if item_id >= len {
                            continue;
                        }
                        let item = list.item(cx, item_id, template);
                        let infos = state.setting_store.template_infos.infos.clone();
                        let title = keys[item_id].clone();

                        // 安全获取 info
                        if let Some(info) = infos.get(&title) {
                            let rondan_bool = info.rondan;

                            // 绑定 Title
                            item.label(ids!(title)).set_text(cx, &title);

                            // 绑定 CheckBox
                            item.check_box(ids!(roudan_bool))
                                .set_active(cx, rondan_bool);
                            self.ids.insert(
                                item.check_box(ids!(roudan_bool)).widget_uid(),
                                title.clone(),
                            );

                            // 绑定随机视图可见性及内容
                            if rondan_bool {
                                // item.view(ids!(roudan_view)).set_visible(cx, true);
                                item.text_input(ids!(rondan_min_input)).set_disabled(cx, false);
                                item.text_input(ids!(rondan_max_input)).set_disabled(cx, false);
                                item.text_input(ids!(rondan_size_input)).set_disabled(cx, false);
                                item.text_input(ids!(roudan_min_input))
                                    .set_text(cx, &info.rondan_min);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_min_input)).widget_uid(),
                                    title.clone(),
                                );

                                item.text_input(ids!(roudan_max_input))
                                    .set_text(cx, &info.rondan_max);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_max_input)).widget_uid(),
                                    title.clone(),
                                );

                                item.text_input(ids!(roudan_size_input))
                                    .set_text(cx, &info.rondan_size);
                                self.ids.insert(
                                    item.text_input(ids!(roudan_size_input)).widget_uid(),
                                    title.clone(),
                                );
                            } else {
                                item.text_input(ids!(rondan_min_input)).set_disabled(cx, true);
                                item.text_input(ids!(rondan_max_input)).set_disabled(cx, true);
                                item.text_input(ids!(rondan_size_input)).set_disabled(cx, true);
                            }

                            // 绑定数据类型
                            item.drop_down(ids!(data_type))
                                .set_selected_by_label(&info.data_type, cx);
                            self.ids.insert(
                                item.drop_down(ids!(data_type)).widget_uid(),
                                title.clone(),
                            );

                            // 绑定数据选择 DropDown
                            if item.drop_down(ids!(data_type)).selected_label() == "数据" {
                                item.drop_down(ids!(data_select)).set_disabled(cx, false);
                                let labels = state.setting_store.all_column_name.clone();
                                item.drop_down(ids!(data_select)).set_labels(cx, labels);
                                item.drop_down(ids!(data_select))
                                    .set_selected_by_label(&info.data_select, cx);
                                self.ids.insert(
                                    item.drop_down(ids!(data_select)).widget_uid(),
                                    title.clone(),
                                );
                            } else {
                                item.drop_down(ids!(data_select)).set_disabled(cx, true);
                            }

                            // 绑定固定内容
                            item.text_input(ids!(fixed_content))
                                .set_text(cx, &info.fixed_content);
                            self.ids.insert(
                                item.text_input(ids!(fixed_content)).widget_uid(),
                                title.clone(),
                            );

                            // 绑定小数点
                            item.text_input(ids!(decimal_input))
                                .set_text(cx, &info.decimal);
                            self.ids.insert(
                                item.text_input(ids!(decimal_input)).widget_uid(),
                                title.clone(),
                            );
                        }

                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
        // ★★★ 关键修改：移除这里的 cx.redraw_all() ★★★
        // 只有在数据真正更新时（在 handle_actions 中）才调用重绘
    }
}

impl WidgetMatchEvent for TemplateInfosRow {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 使用正确的 ID "list" 获取 PortalList
        let list_widget = self.view.portal_list(ids!(list));

        // 遍历列表中产生 Action 的子项
        for (_item_id, item_widget) in list_widget.items_with_actions(actions) {
            // 定义一个闭包来统一处理数据更新，避免代码重复
            // 参数: 触发组件的UID, 更新逻辑闭包
            let mut update_data =
                |cx: &mut Cx, widget_uid: WidgetUid, update_fn: &mut dyn FnMut(&mut InfoDetail)| {
                    // 1. 通过 UID 查找对应的 map Key (Title)
                    if let Some(key) = self.ids.get(&widget_uid) {
                        if let Some(state) = scope.data.get_mut::<Store>() {
                            // 2. 更新 Store
                            if let Some(data) =
                                state.setting_store.template_infos.infos.get_mut(key)
                            {
                                update_fn(data);
                                // 3. ★ 只有数据改变后才请求重绘
                                cx.redraw_all();
                            }
                        }
                    }
                };

            let rondan_bool = item_widget.check_box(ids!(roudan_bool));
            let rondan_min_input = item_widget.text_input(ids!(roudan_min_input));
            let rondan_max_input = item_widget.text_input(ids!(roudan_max_input));
            let rondan_size_input = item_widget.text_input(ids!(roudan_size_input));
            let data_type = item_widget.drop_down(ids!(data_type));
            let data_select = item_widget.drop_down(ids!(data_select));
            let fixed_content = item_widget.text_input(ids!(fixed_content));
            let decimal_input = item_widget.text_input(ids!(decimal_input));

            // ------------------ 启用随机 (roudan_bool) ------------------
            if let Some(active) = rondan_bool.changed(actions) {
                info!("启用随机按钮点击，当前状态：{}", active);
                update_data(cx, rondan_bool.widget_uid(), &mut |data| {
                    data.rondan = active;
                });
                if active {
                    rondan_min_input.set_disabled(cx, false);
                    rondan_max_input.set_disabled(cx, false);
                    rondan_size_input.set_disabled(cx, false);
                } else {
                    rondan_min_input.set_disabled(cx, true);
                    rondan_max_input.set_disabled(cx, true);
                    rondan_size_input.set_disabled(cx, true);
                }
            }

            // ------------------ 随机下限 (rondan_min_input) ------------------
            if let Some(text) = rondan_min_input.changed(actions) {
                let text_val = text.clone();
                update_data(cx, rondan_min_input.widget_uid(), &mut |data| {
                    data.rondan_min = text_val.clone();
                });
            }

            // ------------------ 随机上限 (rondan_max_input) ------------------
            if let Some(text) = rondan_max_input.changed(actions) {
                let text_val = text.clone();
                update_data(cx, rondan_max_input.widget_uid(), &mut |data| {
                    data.rondan_max = text_val.clone();
                });
            }

            // ------------------ 随机粒度 (rondan_size_input) ------------------
            if let Some(text) = rondan_size_input.changed(actions) {
                let text_val = text.clone();
                update_data(cx, rondan_size_input.widget_uid(), &mut |data| {
                    data.rondan_size = text_val.clone();
                });
            }

            // ------------------ 数据类型 (data_type) ------------------
            if let Some(_) = data_type.changed(actions) {
                let selected = data_type.selected_label();
                update_data(cx, data_type.widget_uid(), &mut |data| {
                    data.data_type = selected.clone();
                });
                // 处理 UI 逻辑
                if selected == "数据" {
                    item_widget
                        .view(ids!(data_select_view))
                        .set_visible(cx, true);
                    item_widget
                        .text_input(ids!(fixed_content))
                        .set_disabled(cx, true);
                    item_widget
                        .drop_down(ids!(data_select))
                        .set_disabled(cx, false);
                } else if selected == "固定内容" {
                    item_widget
                        .view(ids!(data_select_view))
                        .set_visible(cx, false);
                    item_widget
                        .text_input(ids!(fixed_content))
                        .set_disabled(cx, false);
                    item_widget
                        .drop_down(ids!(data_select))
                        .set_disabled(cx, true);
                    item_widget.drop_down(ids!(data_select)).set_text(cx, "");
                } else {
                    item_widget
                        .view(ids!(data_select_view))
                        .set_visible(cx, false);
                    item_widget
                        .text_input(ids!(fixed_content))
                        .set_disabled(cx, true);
                    item_widget
                        .drop_down(ids!(data_select))
                        .set_disabled(cx, true);
                    item_widget
                        .drop_down(ids!(data_select))
                        .set_selected_by_label("", cx);
                }
            }

            // ------------------ 数据选择 (data_select) ------------------
            if let Some(_) = data_select.changed(actions) {
                let selected = data_select.selected_label();
                update_data(cx, data_select.widget_uid(), &mut |data| {
                    data.data_select = selected.clone();
                });
            }

            // ------------------ 固定内容 (fixed_content) ------------------
            if let Some(text) = fixed_content.changed(actions) {
                let text_val = text.clone();
                update_data(cx, fixed_content.widget_uid(), &mut |data| {
                    data.fixed_content = text_val.clone();
                });
            }

            // ------------------ 小数位 (decimal_input) ------------------
            if let Some(text) = decimal_input.changed(actions) {
                let text_val = text.clone();
                update_data(cx, decimal_input.widget_uid(), &mut |data| {
                    data.decimal = text_val.clone();
                });
            }
        }
    }
}
