use crate::configs::type_config::{Infos, get_type_infos};
use crate::export::works::carton_query::get_res;
use crate::structs::{Data, Datas};
use crate::utils::{memory::DataStore, retry::retry_default};
use crate::utils::error::MyError;
use crate::widgets::progress::MyProgressWidgetExt;
use crate::{
    export::Exportable,
    store::Store,
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};
use makepad_widgets::*;
use std::sync::{Arc, mpsc};
use tokio::runtime::Runtime;
use tracing::info;

// 常量字符串，避免重复分配
mod constants {
    pub const CARTON_QUERY: &str = "箱号查询";
    pub const BATCH_QUERY: &str = "批量查询";
    pub const QUERY_SUCCESS: &str = "查询成功";
    pub const QUERY_FAILED: &str = "查询失败";
    pub const QUERYING: &str = "查询中";
    pub const EXPORT_COMPLETE: &str = "数据导出完成";
}
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;
    use crate::export::export_tabel::*;
    use crate::widgets::widget::MyDropdown;
    use crate::widgets::progress::MyProgress;
    FirstRow = <View> {
        width: Fill,
        height: Fit,
        spacing:20,
        // show_bg: true,
        // draw_bg: {
        //     color: #D9D9D9
        // }
        type_selector = <MyDropdown> {
            width: 100,
            labels:["type_1","type_2","type_3"],

        }
        // carton_input = <InputClean> {
        carton_input = <MolyTextInput> {
            empty_text: "请输入箱号...."
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
        query_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "箱号查询"
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
            lock_check = <CheckBox> {
                width: Fit
                height: 40
                text: "导出锁定"
                align: { x: 0., y: .5}
                active: true
                draw_text: {
                    color: #000000,
                    text_style: {
                        font_size:14
                    }
                }
                label_walk: {
                            width: Fit, height: Fit,
                            margin: <THEME_MSPACE_H_1> { left: 28. }
                        }
                draw_bg: {
                    uniform size: 25.0;
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
                // color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
    }
    SecondRow = <View> {
        width: Fill
        height: Fit
        align: {y: 0.5}
        spacing: 10,
        <Label> {
            text: "类型信息:"
            draw_text: {
                text_style: {
                    font_size:16
                }
            }
        }
        pch_q = <Label> {
            width: Fill
            text: "批次号查询:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
        pch_q_b = <Label> {
            width: Fill
            text: "批次号-盒号查询:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
        pch_q_c = <Label> {
            width: Fill
            text: "批次号-箱号查询:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
        zdy_q = <Label> {
            width: Fill
            text: "自定义盒号查询:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
        jz_bind = <Label> {
            width: Fill
            text: "尾标绑定查询:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
        templates = <Label> {
            width: Fill
            text: "关联模板:"
            draw_text: {
                text_style: {
                    font_size:14
                }
            }
        }
    }
    StateBar = <View> {
        width: Fill,
        height: Fit,
        align: {y: 0.5}
        // padding: {left: 20, right: 20, top: 0, bottom: 0},
        spacing: 10,
        <Label> {
            text: "状态: "
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
        state_label = <Label> {
            text: "未开始"
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
        progress = <MyProgress> {
            width: Fill, value: .0
        }
    }
    pub ExportScreen = {{ExportScreen}} {
        <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            padding: 15,
            spacing: 10,
            <FirstRow> {}
            <SecondRow> {}
            <View> {
                width: Fill,
                height: Fill,
                flow: Overlay,
                spacing: 10,
                <ExTable> {}
                <View> {
                    width: Fill,
                    height: Fill,
                    align: {x: 0.5, y: 0.5}
                    loading_spinner = <LoadingSpinner> {
                        width: Fit {
                            min: 200.0,
                            max: 500.0,
                        },
                        height: Fit {
                            min: 200.0,
                            max: 500.0,
                        },
                        visible: false
                    }
                }

            }
            <StateBar> {}
        }
    }
}
#[derive(Live, Widget)]
pub struct ExportScreen {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust(None)] // 默认初始化为 None
    pub export_processor: Option<Arc<dyn Exportable>>,
    #[rust]
    progress_receiver: Option<mpsc::Receiver<Data>>,
    #[rust]
    datas: DataStore<Datas>,
    #[rust]
    data: Vec<Data>,
}
#[derive(Debug)]
pub struct TestMsg {
    pub name: String,
    pub age: u32,
}
#[derive(Clone, Debug, Default)]
pub struct ExportAction {
    data: Vec<Datas>,
}

impl LiveHook for ExportScreen {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.export_processor = Some(crate::export::new_export_processor());
    }
}
impl Widget for ExportScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let (Event::Signal, Some(rx)) = (event, &mut self.progress_receiver) {
            // let mut last_msg = None;
            // 循环读取直到取到最新的一条（Drain all available results）
            while let Ok(msg) = rx.try_recv() {
                self.data.push(msg);
                // 避免频繁计算，只在需要时计算进度
                let data_len = self.data.len();
                let total_len = self.datas.len();
                
                if total_len > 0 {
                    let p = (data_len as f64 / total_len as f64) * 100.0;
                    self.view.my_progress(ids!(progress)).set_value(cx, p);
                }
                self.view.redraw(cx);
            }
        }
        self.ui_runner().handle(cx, event, scope, self);
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(store) = scope.data.get::<Store>() {
            self.view
                .drop_down(ids!(type_selector))
                .set_labels(cx, store.setting_store.types.as_ref().clone());
            if store.datas_store.export_datas.is_empty() {
                self.view.button(ids!(export_btn)).set_disabled(cx, true);
            } else {
                self.view.button(ids!(export_btn)).set_disabled(cx, false);
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ExportScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let input = self.view.text_input(ids!(carton_input));
        let query_btn = self.view.button(ids!(query_btn));
        let export_btn = self.view.button(ids!(export_btn));
        let type_name = self.view.drop_down(ids!(type_selector));
        let qty_label = self.label(ids!(qty_label));
        let rt = self.rt.handle().clone();
        let ui = self.ui_runner();
        for action in actions {
            if let Some(data_action) = action.downcast_ref::<ExportAction>() {
                self.datas.set_data(data_action.data.clone());
                let qty = self.datas.get_cached_quantity();
                qty_label.set_text(cx, &qty);
            }
        }
        if input.text().is_empty() {
            query_btn.set_text(cx, constants::BATCH_QUERY);
        } else {
            query_btn.set_text(cx, constants::CARTON_QUERY);
        }
        if let Some(t_name) = type_name.changed_label(actions) {
            // 显示加载状态
            self.view.label(ids!(pch_q)).set_text(cx, "加载中...");
            self.view.label(ids!(pch_q_b)).set_text(cx, "加载中...");
            self.view.label(ids!(pch_q_c)).set_text(cx, "加载中...");
            self.view.label(ids!(jz_bind)).set_text(cx, "加载中...");
            self.view.label(ids!(zdy_q)).set_text(cx, "加载中...");
            self.view.label(ids!(templates)).set_text(cx, "加载中...");
            
            let ui = self.ui_runner();
            let t_name = t_name.clone();
            rt.spawn(async move {
                let res = retry_default(|| get_type_infos(&t_name)).await;
                info!("type_infos: {:?}", res);
                let type_infos = match res {
                    Ok(res) => res,
                    Err(e) => {
                        Cx::post_action(e);
                        (Vec::new(), Infos::default())
                    }
                };
                
                ui.defer_with_redraw(move |me, cx, _scope| {
                    if !type_infos.0.is_empty() {
                        let pch_q_text = format!("批次号查询: {}", bool2string(type_infos.1.is_have_pch));
                        me.view.label(ids!(pch_q)).set_text(cx, &pch_q_text);
                        let carton_pch_text =
                            format!("批次号-箱号查询: {}", bool2string(type_infos.1.carton_pch));
                        me.view
                            .label(ids!(pch_q_c))
                            .set_text(cx, &carton_pch_text);
                        let box_pch_text =
                            format!("批次号-盒号查询: {}", bool2string(type_infos.1.box_pch));
                        me.view.label(ids!(pch_q_b)).set_text(cx, &box_pch_text);
                        let jz_band_text = format!("尾标绑定查询: {}", bool2string(type_infos.1.jz_bind));
                        me.view.label(ids!(jz_bind)).set_text(cx, &jz_band_text);
                        let zdy_box_text = format!("自定义盒号查询: {}", bool2string(type_infos.1.zdy_box));
                        me.view.label(ids!(zdy_q)).set_text(cx, &zdy_box_text);
                        let templates_text = format!("关联模板: {}", type_infos.0.join(", "));
                        me.view
                            .label(ids!(templates))
                            .set_text(cx, &templates_text);
                    } else {
                        // 重置为默认状态
                        me.view.label(ids!(pch_q)).set_text(cx, "批次号查询: 未启用");
                        me.view.label(ids!(pch_q_b)).set_text(cx, "批次号-盒号查询: 未启用");
                        me.view.label(ids!(pch_q_c)).set_text(cx, "批次号-箱号查询: 未启用");
                        me.view.label(ids!(jz_bind)).set_text(cx, "尾标绑定查询: 未启用");
                        me.view.label(ids!(zdy_q)).set_text(cx, "自定义盒号查询: 未启用");
                        me.view.label(ids!(templates)).set_text(cx, "关联模板: 无");
                    }
                });
            });
        }
        if query_btn.clicked(actions) {
            self.view.view(ids!(loading_spinner)).set_visible(cx, true);
            self.view.label(ids!(state_label)).set_text(cx, constants::QUERYING);
            let mut pool = None;
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.datas_store.clear_export_datas();
                qty_label.set_text(cx, "总数量: 0 PCS");
                pool = store.pool.clone();
            }
            self.data.clear();
            self.datas.clear();
            self.view.my_progress(ids!(progress)).set_value(cx, 0.);
            let processor = self.export_processor.as_ref().unwrap().clone();
            let carton = input.text().clone();
            let is_multi = query_btn.text() == "批量查询";
            let type_name = type_name.selected_label().clone();
            let (sender, receiver) = mpsc::channel();
            self.progress_receiver = Some(receiver);
            let pool_clone = pool.clone();
            rt.spawn(async move {
                if let Some(p) = pool_clone {
                    let res = processor
                        .carton_query(carton, type_name, is_multi, &p)
                        .await;

                    match res {
                    Ok(mut data) => {
                        // 发布导出结果 via AppBus
                        crate::app_bus::post(crate::app_bus::BusEvent::ExportResult(data.clone()));
                            let d = get_res(&mut data, &p, sender).await;
                            match d {
                                Ok(data) => {
                                    ui.defer_with_redraw(move |me, cx, scope| {
                                        if let Some(store) = scope.data.get_mut::<Store>() {
                                            store.datas_store.set_export_datas(data);
                                            me.view.view(ids!(loading_spinner)).set_visible(cx, false);
                                            enqueue_popup_notification(PopupItem {
                                                kind: PopupKind::Success,
                                                auto_dismissal_duration: Some(2.5),
                                                message: "查询完成，可以进行导出.".to_string(),
                                            });
                                        }
                                        me.view.label(ids!(state_label)).set_text(cx, constants::QUERY_SUCCESS);
                                    });
                                }
                                Err(e) => {
                                    ui.defer_with_redraw(move |me, cx, _scope| {
                                        me.view.view(ids!(loading_spinner)).set_visible(cx, false);
                                        me.view.label(ids!(state_label)).set_text(cx, constants::QUERY_FAILED);
                                    });
                                    Cx::post_action(e);
                                }
                            }
                        }
                        Err(e) => {
                            ui.defer_with_redraw(move |me, cx, _scope| {
                                me.view.view(ids!(loading_spinner)).set_visible(cx, false);
                                me.view.label(ids!(state_label)).set_text(cx, "查询失败");
                            });
                            Cx::post_action(e);
                        }
                    }
                } else {
                    ui.defer_with_redraw(move |me, cx, _scope| {
                        me.view.view(ids!(loading_spinner)).set_visible(cx, false);
                        me.view.label(ids!(state_label)).set_text(cx, "数据库未连接");
                    });
                    Cx::post_action(MyError::DatabaseNotConnected);
                }
            });
        }
        if export_btn.clicked(actions) {
            info!("开始导出数据...");
            let processor = self.export_processor.as_ref().unwrap().clone();
            let lock = self.view.check_box(ids!(lock_check)).active(cx);
            if let Some(store) = scope.data.get::<Store>() {
                let data_len = store.datas_store.export_datas.len();
                info!("导出数据数量: {}", data_len);
                if data_len > 0 {
                    let type_name = type_name.selected_label();
                    let data = store.datas_store.export_datas.as_ref().clone();
                    rt.spawn(async move {
                        let res = processor.export(&type_name, data, lock).await;
                        match res {
                            Ok(_) => {
                                enqueue_popup_notification(PopupItem {
                                    kind: PopupKind::Success,
                                    auto_dismissal_duration: Some(2.5),
                                    message: constants::EXPORT_COMPLETE.to_string(),
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
    }
}
fn bool2string(value: bool) -> String {
    if value {
        "启用".to_string()
    } else {
        "未启用".to_string()
    }
}
