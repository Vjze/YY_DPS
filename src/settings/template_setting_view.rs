use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    configs::decimal_config::{DecimalConfig, get_decimal_config_value},
    store::Store,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    DecimalItem = <RoundedView> {
        width: 80,
        height: Fit,
        margin: {right:25},
        // padding: {left: 15},
        spacing:10,
        align: {x:0.5}
        flow: Down
        decimal_name = <Label> {
             draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        decimal_input = <MolyTextInput> {
            empty_text: "输入数字0或者1或者其它...."
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
    }
    DecimalRow = {{DecimalRow}} {
        <PortalList> {
            height: Fit,
            flow: Right,

            DecimalItem = <DecimalItem> {
                cursor: Default
            }
        }
    }
    DecimalGrid = {{DecimalGrid}} {
        <PortalList> {
            height: 150
            flow: Down,

            DecimalRow = <DecimalRow> {}
        }
    }

    DecimalStringItem = <RoundedView> {
        width: 150,
        height: Fit,
        margin: {right:25},
        // padding: {left: 15},
        spacing:10,
        align: {x:0.5}
        flow: Down
        decimal_name = <DropDown> {
                        width: 150,height:40
                        labels:["template_1","template_2","template_3"],
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
        decimal_input = <MolyTextInput> {
            empty_text: "输入数字0或者1或者其它...."
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
    }
    DecimalStringRow = {{DecimalStringRow}} {
        <PortalList> {
            height: Fit,
            flow: Right,

            DecimalStringItem = <DecimalStringItem> {
                cursor: Default
            }
        }
    }
    DecimalStringGrid = {{DecimalStringGrid}} {
        <PortalList> {
            height: 150
            flow: Down,

            DecimalStringRow = <DecimalStringRow> {}
        }
    }
    
    pub TemplateView = {{TemplateView}} <RoundedShadowView> {
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
                    template_input = <MolyTextInput> {
                        empty_text: "新增模板必须输入名称...."
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
                    add_template_btn = <Button> {
                        text: "新增模板"
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
                        text: "选择模板:"
                        draw_text: {
                            text_style: {
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    template_selector = <DropDown> {
                        width: 200,height:40
                        labels:["template_1","template_2","template_3"],
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
                <ScrollYView> {
                    padding:15,
                    // show_bg: true
                    // draw_bg: {
                    //     color: (MAIN_BG_COLOR_DARK)
                    //     border_radius: 4.5,
                    //     uniform shadow_color: #0002
                    //     shadow_radius: 8.0,
                    //     shadow_offset: vec2(0.0,-1.5)
                    // }
                    height: Fill,
                    spacing: 10,
                    flow: Down,
                    <Label> {
                        text: "小数点配置:"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 16
                            }
                            color: #000
                        }
                    }
                    decimal_row = <DecimalGrid> {}
                    <Label> {
                        text: "固定内容配置:"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 16
                            }
                            color: #000
                        }
                    }
                    <DecimalStringGrid> {}
                }


                <View> {
                    spacing: 10,
                    align: {x: 0.5, y: 1.0}
                    width: Fill,
                    height: Fit
                    update_type_btn = <Button> {
                        width: 120, height: 40,
                        text: "模板更新",
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
                    del_type_btn = <Button> {
                        width: 120, height: 40,
                        text: "模板删除",
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

// TODO: Rename into TemplateView
#[derive(Widget, LiveHook, Live)]
struct TemplateView {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    rt: Runtime,
}

impl Widget for TemplateView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get_mut::<Store>() {
            if !store.templates.is_empty() {
                self.view
                    .drop_down(id!(template_selector))
                    .set_labels(cx, store.templates.clone());
            };
        }
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for TemplateView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let select = self.view.drop_down(id!(template_selector));
        let input = self.view.text_input(id!(template_input));
        let add_type_btn = self.view.button(id!(add_template_btn));
        if let Some(value) = select.changed_label(actions) {
            input.set_text(cx, &value);
            let type_name = input.text().clone();
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            let decimal_infos = rt.block_on(async move {
                let res = get_decimal_config_value(type_name).await;
                match res {
                    Ok(res) => res,
                    Err(e) => {
                        Cx::post_action(e);
                        DecimalConfig::default()
                    }
                }
            });
            println!("infos = {:?}",decimal_infos);
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.set_decimal_config(decimal_infos);
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DecimalRow {
    #[deref]
    view: View,
}
impl Widget for DecimalRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(state) = scope.data.get_mut::<Store>() {
                    // 动态设置项范围为 numbers 的长度
                    let item_count = state.template_infos.numbers.len();
                    let row_idx = *scope.props.get::<usize>().unwrap();
                    let first_idx = row_idx * 6;
                    let num_remaining = item_count - first_idx;
                    let x = 10.min(num_remaining);
                    list.set_item_range(cx, 0, x);
                    // 迭代 numbers 的键值对
                    let mut keys = state.template_infos.numbers.keys().collect::<Vec<_>>();
                    keys.sort(); // 可选：按键排序以确保一致的显示顺序
                    for (item_idx, key) in keys.iter().enumerate() {
                        if item_idx >= x {
                            continue;
                        }
                        // 直接使用 item_idx 创建项
                        let item = list.item(cx, item_idx, live_id!(DecimalItem));
                        let label_name = item.label(id!(decimal_name));
                        let decimal_input = item.text_input(id!(decimal_input));

                        // 设置 label 为 key
                        label_name.set_text(cx, key);

                        // 设置 input 为 value（转换为字符串）
                        if let Some(value) = state.template_infos.numbers.get(*key) {
                            decimal_input.set_text(cx, &value.to_string());
                        } else {
                            decimal_input.set_text(cx, "");
                        }

                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
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
                let state = scope.data.get_mut::<Store>().unwrap();
                let len = state.template_infos.numbers.len().div_ceil(6);
                list.set_item_range(cx, 0, len);
                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= len {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(DecimalRow));
                    let mut scope = Scope::with_data_props(state, &row_idx);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}


#[derive(Live, LiveHook, Widget)]
pub struct DecimalStringRow {
    #[deref]
    view: View,
}
impl Widget for DecimalStringRow {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                if let Some(state) = scope.data.get_mut::<Store>() {
                    // 动态设置项范围为 numbers 的长度
                    let item_count = state.template_infos.strings.len();
                    let row_idx = *scope.props.get::<usize>().unwrap();
                    let first_idx = row_idx * 10;
                    let num_remaining = item_count - first_idx;
                    let x = 10.min(num_remaining);
                    list.set_item_range(cx, 0, x);
                    // 迭代 numbers 的键值对
                    let mut keys = state.template_infos.strings.iter().map(|(key,_)|{
                        let k = key.clone();
                        k
                    }).collect::<Vec<_>>();
                    keys.sort(); // 可选：按键排序以确保一致的显示顺序
                    for (item_idx, key) in keys.clone().iter().enumerate() {
                        if item_idx >= x {
                            continue;
                        }
                        // 直接使用 item_idx 创建项
                        let item = list.item(cx, item_idx, live_id!(DecimalStringItem));
                        let label_name = item.drop_down(id!(decimal_name));
                        let decimal_input = item.text_input(id!(decimal_input));
                        label_name.set_labels(cx, keys.clone());
                        // 设置 label 为 key
                        label_name.set_selected_by_label(&key,cx);

                        // 设置 input 为 value（转换为字符串）
                        if let Some(value) = state.template_infos.strings.get(key) {
                            decimal_input.set_text(cx, &value.to_string());
                        } else {
                            decimal_input.set_text(cx, "");
                        }

                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct DecimalStringGrid {
    #[deref]
    view: View,
}

impl Widget for DecimalStringGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<Store>().unwrap();
                let len = state.template_infos.strings.len().div_ceil(10);
                list.set_item_range(cx, 0, len);
                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= len {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(DecimalStringRow));
                    let mut scope = Scope::with_data_props(state, &row_idx);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
