use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    configs::decimal_config::{
        DecimalConfig, add_new_template, delete_template, get_decimal_config_value, update_template,
    },
    settings::{add_template_modal::TemplateModalAction, delete_modal::DeleteModalAction},
    store::Store,
    utils::error::MyError,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::settings::decimal_grid::DecimalGrid;
    use crate::settings::fixed_content_grid::FixedGrid;
    use crate::settings::tabel_grid::TableGrid;
    use crate::settings::add_template_modal::AddTemplateModal;
    use crate::settings::delete_modal::DeleteModal;

    pub TemplateView = {{TemplateView}} <RoundedShadowView> {
        width: Fill, height: Fill
        align: {x: 0.0, y: 0.0}
        padding: {left: 15, right: 15, bottom: 15, top:15}
        show_bg: true
        flow: Overlay
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
                    clear_template_btn = <Button> {
                        width: 120, height: 40,
                        text: "数据清空",
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
                <ScrollYView> {
                    width: Fill,
                    height: Fit,
                    spacing: 15,
                    flow: Down,
                    <View> {
                        height: 180,
                        width: Fill,
                        flow: Down,
                        spacing: 10,
                        align: {x: 0.5}
                        <Label> {
                            text: "小数点配置:"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{
                                    font_size: 16
                                }
                                color: #000
                            }
                        }
                        <DecimalGrid> {}
                    }
                    <View> {
                        height: 280,
                        width: Fill,
                        flow: Down,
                        spacing: 10,
                        align: {x: 0.5}
                        <Label> {
                            text: "固定内容配置:"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{
                                    font_size: 16
                                }
                                color: #000
                            }
                        }
                        <FixedGrid> {}
                    }
                    <View> {
                        height: 150,
                        width: Fill,
                        flow: Down,
                        spacing: 10,
                        align: {x: 0.5}
                        <Label> {
                            text: "表格内容配置:"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{
                                    font_size: 16
                                }
                                color: #000
                            }
                        }
                        <TableGrid> {}
                    }


                <View> {
                    spacing: 10,
                    align: {x: 0.5, y: 1.0}
                    width: Fill,
                    height: Fit
                    update_template_btn = <Button> {
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
                    delete_template_btn = <Button> {
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
        add_modal = <Modal> {
            content: {
                <AddTemplateModal> {}
            }
        }
        delete_modal = <Modal> {
            content: {
                <DeleteModal> {}
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
            if !store.setting_store.templates.is_empty() {
                self.view
                    .drop_down(ids!(template_selector))
                    .set_labels(cx, store.setting_store.templates.clone());
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
        let select = self.view.drop_down(ids!(template_selector));
        let input = self.view.text_input(ids!(template_input));
        let add_btn = self.view.button(ids!(add_template_btn));
        let update_btn = self.view.button(ids!(update_template_btn));
        let delete_btn = self.view.button(ids!(delete_template_btn));
        let clear_btn = self.view.button(ids!(clear_template_btn));
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
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.setting_store.template_infos = decimal_infos;
            }
        }
        if add_btn.clicked(actions) {
            self.modal(ids!(add_modal)).open(cx);
        }
        if update_btn.clicked(actions) {
            let template_name = input.text().clone();
            let rt = self.rt.handle().clone();
            if let Some(store) = scope.data.get_mut::<Store>() {
                let template_infos = store.setting_store.template_infos.clone();
                let _guard = rt.enter();
                rt.block_on(async move {
                    match update_template(template_name, template_infos).await {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
                        }
                        Err(e) => {
                            Cx::post_action(e);
                        }
                    }
                });
            };
        }
        if delete_btn.clicked(actions) {
            if input.text().is_empty() {
                Cx::post_action(MyError::Zdyknown(format!("模板名称不能为空!!!")));
            } else {
                self.modal(ids!(delete_modal)).open(cx);
            }
        }
        if clear_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.setting_store.template_infos = DecimalConfig::default();
                input.set_text(cx, "");
            }
        }
        for action in actions {
            if let Some(TemplateModalAction::Close) = action.downcast_ref() {
                self.modal(ids!(add_modal)).close(cx);
            }
            if let Some(TemplateModalAction::Action(rows)) = action.downcast_ref() {
                let template_name = input.text().clone();
                let rt = self.rt.handle().clone();
                if let Some(store) = scope.data.get_mut::<Store>() {
                    let template_infos = store.setting_store.template_infos.clone();
                    let _guard = rt.enter();
                    rt.block_on(async move {
                        match add_new_template(template_name, template_infos, rows.clone()).await {
                            Ok(res) => {
                                Cx::post_action(res);
                                Store::init().await;
                            }
                            Err(e) => {
                                Cx::post_action(e);
                            }
                        }
                    });
                };
            }
            if let Some(DeleteModalAction::Close) = action.downcast_ref() {
                self.modal(ids!(delete_modal)).close(cx);
            }
            if let Some(DeleteModalAction::Action) = action.downcast_ref() {
                let template_name = select.text().clone();
                let rt = self.rt.handle().clone();
                let _guard = rt.enter();
                rt.block_on(async move {
                    match delete_template(template_name).await {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
                        }
                        Err(e) => {
                            Cx::post_action(e);
                        }
                    }
                });
            }
        }
    }
}
