use std::collections::HashMap;
use std::sync::Arc;

use crate::querys::DatasQuery;
use crate::widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification};
use crate::{store::Store, utils::error::MyError};
use bb8_tiberius::ConnectionManager;
use chrono::Local;
use makepad_widgets::*;
use tokio::runtime::Runtime;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;
    use crate::querys::tabel::InfosTable;
    FirstRow = <View> {
        width: Fill,
        height: Fit,
        spacing:20,
        align: {y:0.5}
        <Label> {
            text: "查询类型:"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        type_selector = <DropDown> {
            width: 60,height:40
            labels:["箱号","盒号","Sn"],
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
        <Label> {
            text: "料号:"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        pn_input = <MolyTextInput> {
            empty_text: "输入料号...."
            width: 100, height: 40
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
        is_sn_query = <View> {
            width: Fit,
            spacing:15,
            align: {y:0.5}
            <Label> {
                text: "选择设备:"
                draw_text: {
                    text_style: <REGULAR_FONT>{
                        font_size: 12
                    }
                    color: #000
                }
            }
            devices_selector = <DropDown> {
                width: 60,height:40
                labels:["全部","10G","2.5G"],
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
            <Label> {
                text: "选择结果:"
                draw_text: {
                    text_style: <REGULAR_FONT>{
                        font_size: 12
                    }
                    color: #000
                }
            }
            result_selector = <DropDown> {
                width: 60,height:40
                labels:["全部","Ok","NG"],
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
        date = <MySwitch> {
            text: "使用日期"
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
        }
        date_view = <View> {
            width: Fill,
            align: {y: 0.5}
            spacing: 10,
            <Label> {
                text: "开始时间:"
                draw_text: {
                    text_style: <REGULAR_FONT>{
                        font_size: 12
                    }
                    color: #000
                }
            }
            start_time_input = <MolyTextInput> {
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
            <Label> {
                text: "结束时间:"
                draw_text: {
                    text_style: <REGULAR_FONT>{
                        font_size: 12
                    }
                    color: #000
                }
            }
            end_time_input = <MolyTextInput> {
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
        }
    }
    SecondRow = <View> {
        width: Fill,
        height: Fit,
        spacing: 20,
        query_input = <MolyTextInput> {
            empty_text: "请输入箱号、盒号、Sn...."
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
        worker_input = <MolyTextInput> {
            empty_text: "工号...."
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
                uniform color: #FFF
            }
        }
        query_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "开始查询"
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
        export_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "数据导出"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size:16
                }
            }
            draw_bg: {
                uniform border_size: 1.0
                uniform border_radius: 5.0
                uniform color: #AFEEEE
                uniform color_hover: #9370DB
                uniform color_disabled: #DCDCDC

            }
        }

        qty_label = <Label> {
            padding: {
                top:5
            }
            text: "总数量: 0 PCS"
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
    }
    pub QueryScreen = {{QueryScreen}} {
        <View> {
            width: Fill,
            height: Fill,
            padding: 15,
            spacing: 10,
            flow: Down,
            <FirstRow> {}
            <SecondRow> {}
            <InfosTable> {}
        }
    }
}
#[derive(Live, Widget)]
pub struct QueryScreen {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust(None)] // 默认初始化为 None
    pub datas_query_processor: Option<Arc<dyn DatasQuery>>,
}

#[derive(Clone, Debug, Default)]
pub struct QueryAction {
    data: Vec<HashMap<String, String>>,
}
impl LiveHook for QueryScreen {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.datas_query_processor = Some(crate::querys::new_query_processor());
    }
}
impl Widget for QueryScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
       
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
         if let Some(store) = scope.data.get::<Store>() {
            if store.datas_store.query_datas.is_empty() {
                self.view.button(ids!(export_btn)).set_disabled(cx, true);
            } else {
                self.view.button(ids!(export_btn)).set_disabled(cx, false);
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for QueryScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let input = self.view.text_input(ids!(query_input));
        let query_btn = self.view.button(ids!(query_btn));
        let export_btn = self.view.button(ids!(export_btn));
        let type_select = self.view.drop_down(ids!(type_selector));
        let use_date = self.view.check_box(ids!(date));
        let start_time_input = self.view.text_input(ids!(start_time_input));
        let end_time_input = self.view.text_input(ids!(end_time_input));
        let pn_input = self.view.text_input(ids!(pn_input));
        let worker_input = self.view.text_input(ids!(worker_input));
        let devices = self.view.drop_down(ids!(devices_selector));
        let res = self.view.drop_down(ids!(result_selector));
        let qty_label = self.view.label(ids!(qty_label));
        let processor = self.datas_query_processor.as_ref().unwrap().clone();
        let rt = self.rt.handle().clone();
        for action in actions {
            if let Some(data_action) = action.downcast_ref::<QueryAction>() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.datas_store.query_datas = data_action.data.clone();
                }
            }
        }
        if query_btn.clicked(actions) {
            if input.text().is_empty()
                && start_time_input.text().is_empty()
                && end_time_input.text().is_empty()
                && pn_input.text().is_empty()
                && worker_input.text().is_empty()
            {
                Cx::post_action(MyError::AllNone);
            } else {
                let mut pool: Option<bb8::Pool<ConnectionManager>> = None;
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.datas_store.query_datas.clear();
                    qty_label.set_text(
                        cx,
                        &format!("总数量: {} PCS", store.datas_store.query_datas.len()),
                    );
                    pool = store.pool.clone();
                }
                let query_input = input.text();
                let query_type = type_select.selected_label();
                let use_date = use_date.active(cx);
                let query_start_time = start_time_input.text();
                let query_end_time = end_time_input.text();
                let query_pn = pn_input.text();
                let query_worker = worker_input.text();
                let query_devices = devices.selected_label();
                let query_result = res.selected_label();
                if query_type == "Sn" {
                    let sns = if query_input.is_empty() {
                        vec![]
                    } else {
                        vec![query_input.clone()]
                    };
                    rt.spawn(async move {
                        let res = processor
                            .sn_query_datas(
                                sns,
                                query_pn,
                                use_date,
                                query_start_time,
                                query_end_time,
                                query_result,
                                query_devices,
                                query_worker,
                                &pool.unwrap(),
                            )
                            .await;
                        match res {
                            Ok(data) => {
                                Cx::post_action(QueryAction { data });
                            }
                            Err(err) => {
                                Cx::post_action(err);
                            }
                        }
                    });
                } else if query_type == "盒号" {
                    rt.spawn(async move {
                        let res = processor
                            .box_query(
                                query_input,
                                use_date,
                                query_start_time,
                                query_end_time,
                                query_pn,
                                &pool.unwrap(),
                            )
                            .await;
                        match res {
                            Ok(data) => {
                                Cx::post_action(QueryAction { data });
                            }
                            Err(err) => {
                                Cx::post_action(err);
                            }
                        }
                    });
                } else {
                    rt.spawn(async move {
                        let res = processor
                            .get_carton_datas(
                                query_input,
                                use_date,
                                query_start_time,
                                query_end_time,
                                query_pn,
                                &pool.unwrap(),
                            )
                            .await;
                        match res {
                            Ok(data) => {
                                Cx::post_action(QueryAction { data });
                            }
                            Err(err) => {
                                Cx::post_action(err);
                            }
                        }
                    });
                }
            }
        }
        if export_btn.clicked(actions) {
            let processor = self.datas_query_processor.as_ref().unwrap().clone();
            if let Some(store) = scope.data.get::<Store>() {
                if !store.datas_store.query_datas.is_empty() {
                    let data = store.datas_store.query_datas.clone();
                    rt.spawn(async move {
                        let res = processor.data_export(data).await;
                        match res {
                            Ok(_path) => {
                                enqueue_popup_notification(PopupItem {
                                    kind: PopupKind::Success,
                                    auto_dismissal_duration: Some(2.5),
                                    message: "数据导出完成".to_string(),
                                });
                            }
                            Err(e) => {
                                Cx::post_action(e);
                            }
                        }
                    });
                }
            }
        }
        if use_date.active(cx) {
            self.view.widget(ids!(date_view)).set_visible(cx, true);
            let date = Local::now().date_naive();
            let start_time = date.format("%Y-%m-%d 00:00:00").to_string();
            let end_time = date.format("%Y-%m-%d 23:59:59").to_string();
            start_time_input.set_text(cx, &start_time);
            end_time_input.set_text(cx, &end_time);
        } else {
            self.view.widget(ids!(date_view)).set_visible(cx, false);
        }

        if type_select.selected_label() != "Sn" {
            self.view.widget(ids!(is_sn_query)).set_visible(cx, false);
        } else {
            self.view.widget(ids!(is_sn_query)).set_visible(cx, true);
        }
    }
}
