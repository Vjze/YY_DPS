use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    configs::type_config::{Infos, add_new_type, delete_type, get_type_infos, update_type},
    settings::type_add_template_modal::TemplateNameModalAction,
    store::Store,
    utils::error::MyError,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::settings::type_add_template_modal::AddTemplateNameModal;
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
            flow: Overlay
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
                <RoundedShadowView> {
                    padding: {left: 15, right: 15, bottom: 15, top:15}
                    show_bg: true
                    draw_bg: {
                        color: (MAIN_BG_COLOR_DARK)
                        border_radius: 4.5,
                        uniform shadow_color: #0002
                        shadow_radius: 8.0,
                        shadow_offset: vec2(0.0,-1.5)
                    }
                    height: Fit,
                    spacing: 10,
                    align: {y: 0.5}
                    flow: Down,
                    <Label> {
                        text: "型号查询数据配置:"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 16
                            }
                            color: #000
                        }
                    }
                    <View> {
                        align: {y: 0.5}
                        height: Fit, width: Fill,
                        spacing: 10,
                        <Label> {
                            text: "批次号查询:"
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 14
                                }
                                color: #000
                            }
                        }
                        pch_check = <MySwitch> {
                            width: 50,
                        }
                        is_have_pch_view = <View> {
                            width: Fill, height: Fit,
                            spacing: 10,
                            align: {y: 0.5}

                            <Label> {
                            text: "箱号查询批次号:"
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 14
                                }
                                color: #000
                            }
                            }
                            carton_pch_check = <MySwitch> {
                                    width: 50,
                                }
                                <Label> {
                                    text: "盒号查询批次号:"
                                    draw_text: {
                                        text_style: <REGULAR_FONT>{
                                            font_size: 14
                                        }
                                        color: #000
                                    }
                                }
                                box_pch_check = <MySwitch> {
                                    width: 50,
                                }
                        }
                        <Label> {
                            text: "自定义盒号查询:"
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 14
                                }
                                color: #000
                            }
                        }
                        zdy_box_check = <MySwitch> {
                            width: 50,
                        }
                        <Label> {
                            text: "九州绑定数据查询:"
                            draw_text: {
                                text_style: <REGULAR_FONT>{
                                    font_size: 14
                                }
                                color: #000
                            }
                        }
                        jz_band_check = <MySwitch> {
                            width: 50,
                        }
                    }
                }
                t_row = <TemplateItemsRow> {}
                <View> {
                    spacing: 10,
                    align: {x: 0.5, y: 1.0}
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
                    update_type_btn = <Button> {
                        width: 120, height: 40,
                        text: "保存类型",
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
                        text: "删除类型",
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
        type_add_template_modal = <Modal> {
            content : {
                <AddTemplateNameModal> {}
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
    rt: Runtime,
}

impl Widget for TypeView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get_mut::<Store>() {
            if !store.types.is_empty() {
                self.view
                    .drop_down(id!(type_selector))
                    .set_labels(cx, store.types.clone());
            };
            if store.type_infos.1.is_have_pch {
                self.view
                    .widget(id!(is_have_pch_view))
                    .set_visible(cx, true);
            } else {
                self.view
                    .widget(id!(is_have_pch_view))
                    .set_visible(cx, false);
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
        let add_type_btn = self.view.button(id!(add_type_btn));
        let pch_check = self.view.check_box(id!(pch_check));
        let carton_pch_check = self.view.check_box(id!(carton_pch_check));
        let box_pch_check = self.view.check_box(id!(box_pch_check));
        let zdy_box_check = self.view.check_box(id!(zdy_box_check));
        let jz_band_check = self.view.check_box(id!(jz_band_check));
        let save_type_btn = self.view.button(id!(update_type_btn));
        let del_type_btn = self.view.button(id!(del_type_btn));
        let add_template_btn = self.view.button(id!(add_template_btn));
        if let Some(s) = select.changed_label(actions) {
            input.set_text(cx, &s);
            let type_name = input.text().clone();
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            let type_infos = rt.block_on(async move {
                let res = get_type_infos(type_name).await;
                match res {
                    Ok(res) => res,
                    Err(e) => {
                        Cx::post_action(e);
                        (Vec::new(), Infos::default())
                    }
                }
            });
            if !type_infos.0.is_empty() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.type_infos = type_infos;

                    self.view
                        .check_box(id!(pch_check))
                        .set_active(cx, store.type_infos.1.is_have_pch);
                    self.view
                        .check_box(id!(carton_pch_check))
                        .set_active(cx, store.type_infos.1.carton_pch);
                    self.view
                        .check_box(id!(box_pch_check))
                        .set_active(cx, store.type_infos.1.box_pch);
                    self.view
                        .check_box(id!(jz_band_check))
                        .set_active(cx, store.type_infos.1.jz_band);
                    self.view
                        .check_box(id!(zdy_box_check))
                        .set_active(cx, store.type_infos.1.zdy_box);
                }
            }
        }
        if let Some(check) = pch_check.changed(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.type_infos.1.is_have_pch = check;
            }
        }
        if let Some(check) = zdy_box_check.changed(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.type_infos.1.zdy_box = check;
            }
        }
        if let Some(check) = carton_pch_check.changed(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.type_infos.1.carton_pch = check;
                store.type_infos.1.box_pch = !check; // 同步盒号查询状态
                self.view
                    .check_box(id!(box_pch_check))
                    .set_active(cx, !check); // 同步盒号查询状态
            }
        }
        if let Some(check) = box_pch_check.changed(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.type_infos.1.box_pch = check;
                store.type_infos.1.carton_pch = !check; // 同步箱号查询状态
                self.view
                    .check_box(id!(carton_pch_check))
                    .set_active(cx, !check); // 同步箱号查询状态
            }
        }
        if let Some(check) = jz_band_check.changed(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.type_infos.1.jz_band = check;
            }
        }
        if add_type_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                let type_name = input.text().clone();
                if type_name.is_empty() {
                    Cx::post_action(MyError::NoResult("型号名称不能为空".to_string()));
                    return;
                }
                if store.types.contains(&type_name) {
                    Cx::post_action(MyError::NoResult("型号已存在".to_string()));
                    return;
                }
                let rt = self.rt.handle().clone();
                let _guard = rt.enter();
                let infos = store.type_infos.clone();
                rt.block_on(async move {
                    let res = add_new_type(type_name, infos.0, infos.1).await;
                    match res {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
                        }
                        Err(e) => {
                            Cx::post_action(e);
                            return;
                        }
                    }
                });
            }
        }
        if add_template_btn.clicked(actions) {
            let type_name = input.text().clone();
            if type_name.is_empty() {
                Cx::post_action(MyError::Zdyknown("型号名称不能为空".to_string()));
                return;
            }
            self.view.modal(id!(type_add_template_modal)).open(cx);
        }
        if save_type_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                let type_name = input.text().clone();
                if type_name.is_empty() {
                    Cx::post_action(MyError::Zdyknown("型号名称不能为空".to_string()));
                    return;
                }
                let rt = self.rt.handle().clone();
                let _guard = rt.enter();
                rt.block_on(async move {
                    let res = update_type(
                        type_name,
                        store.type_infos.0.clone(),
                        store.type_infos.1.clone(),
                    )
                    .await;
                    match res {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
                        }
                        Err(e) => Cx::post_action(e),
                    }
                });
            }
        }
        if del_type_btn.clicked(actions) {
            let type_name = input.text().clone();
            if type_name.is_empty() {
                Cx::post_action(MyError::Zdyknown("型号名称不能为空".to_string()));
                return;
            }
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            rt.block_on(async move {
                let res = delete_type(type_name).await;
                match res {
                    Ok(res) => {
                        Cx::post_action(res);
                        Store::init().await;
                    }
                    Err(e) => Cx::post_action(e),
                }
            });
        }
        for action in actions {
            if let Some(TemplateNameModalAction::Close) = action.downcast_ref() {
                self.view.modal(id!(type_add_template_modal)).close(cx);
            }
            if let Some(TemplateNameModalAction::Action(template_name)) = action.downcast_ref() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.type_infos.0.push(template_name.clone());
                }
                self.view.modal(id!(type_add_template_modal)).close(cx);
            }
        }
        let list_widget = self.view.portal_list(id!(list));
        for (item_id, item_widget) in list_widget.items_with_actions(actions) {
            if item_widget.button(id!(del_btn)).clicked(actions) {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.type_infos.0.remove(item_id);
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
                list.set_item_range(cx, 0, state.type_infos.0.len());
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.type_infos.0.len() {
                        continue;
                    }
                    let item = list.item(cx, item_idx, live_id!(TemplateItems));
                    let label_name = item.label(id!(template_name));
                    let t_name = state.type_infos.0.get(item_idx).unwrap();
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
