use crate::box_band::unband_dialog::UnbandModalAction;
use crate::box_band::work::unbind_work::{unbind_box, unbind_carton};
use crate::{
    box_band::work::{
        band_work::band_work,
        query_work::{BoxBandData, query_carton_info},
    },
    store::Store,
    utils::error::MyError,
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};
use makepad_widgets::*;
use tokio::runtime::Runtime;
use tracing::info;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;
    use crate::box_band::band_table::*;
    use crate::box_band::unband_dialog::UnbandModal;

    FirstRow = <View> {
        width: Fill,
        height: Fit,
        spacing: 20,

        <Label> {
            text: "箱号:"
            width: Fit,
            height: 40
            padding: {top: 5}
            draw_text: {
                color: #000000,
                text_style: {
                    font_size: 16
                }
            }
        }
        carton_input = <MolyTextInput> {
            empty_text: "请输入箱号...."
            width: Fill,
            height: 40
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

        query_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "箱号查询"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size: 16
                }
            }
            draw_bg: {
                uniform border_size: 1.0
                uniform border_radius: 5.0
                uniform color: #00FFFF
                uniform color_hover: #FFB6C1
                uniform color_disabled: #A9A9A9
            }
        }

        band_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "盒号绑定"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size: 16
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
        unband_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "盒号解绑"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size: 16
                }
            }
            draw_bg: {
                uniform border_size: 1.0
                uniform border_radius: 5.0
                uniform color: #FF4500
                uniform color_hover: #FFB6C1
                uniform color_disabled: #A9A9A9
            }
        }
        boxs_num = <Label> {
            padding: {
                top: 5
            }
            text: "一共: 0 盒"
            draw_text: {
                color: #000,
                text_style: {
                    font_size: 16
                }
            }
        }
    }
    BandRowHeaderLabel = <View> {
        height: Fit
        align: {x: 0.5, y: 0.5}
        label = <Label> {
            width: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD>{
                    font_size: 15,
                }
                color: #1C1C1C,
            }
        }
    }
    BandHeaderRow = <View> {
        align: {x: 0.0, y: 0.5}
        width: Fill
        height: Fit,
        spacing: 30,
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #F2F4F7;
            }
        }
        <BandRowHeaderLabel> {
            width: Fill {
                weight: 1.0
            },
            label = {text: "箱号"}
        }
        <BandRowHeaderLabel> {
            width: Fill {
                weight: 1.0
            },
            label = {text: "旧盒号"}
        }
        <BandRowHeaderLabel> {
            width: Fill {
                weight: 1.0
            },
            label = {text: "新盒号"}
        }
        <BandRowHeaderLabel> {
            width: Fill {
                weight: 1.0
            },
            label = {text: "料号"}
        }
        <BandRowHeaderLabel> {
            width: Fill {
                weight: 1.0
            },
            label = {text: "装盒时间"}
        }
    }
    pub BoxBandView = {{BoxBandView}} {
        width: Fill,
        height: Fill,
        padding: 15,
        spacing: 10,
        flow: Overlay
        <View> {
            flow: Down,
            <FirstRow> {}
            <BandHeaderRow> {
                cursor: Default
            }
            <BandTable> {}
        }
        unbind_modal = <Modal> {
            content: {
                <UnbandModal> {}
            }
        }

    }
}
#[derive(Live, LiveHook, Widget)]
struct BoxBandView {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
}
#[derive(Clone, Debug, Default)]
struct BoxBandAction {
    data: Vec<BoxBandData>,
}
#[derive(Clone, Debug, Default)]
struct BoxUnBandAction {
    carton_no: String,
}
impl Widget for BoxBandView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl BoxBandView {
    fn query(&self, _cx: &mut Cx, _scope: &mut Scope, carton_no: String) {
        let rt = self.rt.handle().clone();
        if carton_no.is_empty() {
            Cx::post_action(MyError::Zdyknown("请输入箱号!!!".to_string()));
        } else {
            let carton = carton_no.clone();
            rt.spawn(async move {
                let res = query_carton_info(&carton).await;
                match res {
                    Ok(data) => {
                        Cx::post_action(BoxBandAction { data });
                    }
                    Err(e) => {
                        Cx::post_action(e);
                    }
                }
            });
        }
    }
}
impl WidgetMatchEvent for BoxBandView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let query_btn = self.view.button(ids!(query_btn));
        let band_btn = self.view.button(ids!(band_btn));
        let carton_input = self.view.text_input(ids!(carton_input));
        let boxs_num = self.view.label(ids!(boxs_num));
        let unband_btn = self.view.button(ids!(unband_btn));
        let rt = self.rt.handle().clone();

        for action in actions {
            if let Some(data_action) = action.downcast_ref::<BoxBandAction>() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    let num = format!("一共: {} 盒", data_action.data.len());
                    boxs_num.set_text(cx, &num.to_string());
                    store.box_band_store.box_data = data_action.data.clone();
                }
            }
            if let Some(UnbandModalAction::Close) = action.downcast_ref() {
                self.modal(ids!(unbind_modal)).close(cx);
            }
            if let Some(UnbandModalAction::Action(no, select)) = action.downcast_ref() {
                let mut pool = None;
                if let Some(store) = scope.data.get_mut::<Store>() {
                    pool = store.pool.clone();
                }
                if select == "箱号" {
                    let res = rt
                        .block_on(async move { unbind_carton(&no, &pool.clone().unwrap()).await });
                    match res {
                        Ok(_) => {
                            enqueue_popup_notification(PopupItem {
                                kind: PopupKind::Success,
                                auto_dismissal_duration: Some(2.5),
                                message: format!("箱号: {} 解绑成功.", no),
                            });
                        }
                        Err(e) => {
                            Cx::post_action(e);
                        }
                    }
                } else {
                    let res =
                        rt.block_on(async move { unbind_box(&no, &pool.clone().unwrap()).await });
                    match res {
                        Ok(_) => {
                            enqueue_popup_notification(PopupItem {
                                kind: PopupKind::Success,
                                auto_dismissal_duration: Some(2.5),
                                message: format!("盒号: {} 解绑成功.", no),
                            });
                        }
                        Err(e) => {
                            Cx::post_action(e);
                        }
                    }
                }
            }
        }
        if let Some((i, _)) = carton_input.returned(actions) {
            self.query(cx, scope, i);
        }

        if carton_input.text().is_empty() {
            query_btn.set_enabled(cx, false);
            query_btn.set_disabled(cx, true);
            band_btn.set_enabled(cx, false);
            band_btn.set_disabled(cx, true);
        } else {
            query_btn.set_enabled(cx, true);
            query_btn.set_disabled(cx, false);
            band_btn.set_enabled(cx, true);
            band_btn.set_disabled(cx, false);
        }
        if query_btn.clicked(actions) {
            self.query(cx, scope, carton_input.text());
        }
        if band_btn.clicked(actions) {
            if let Some(store) = scope.data.get::<Store>() {
                if store.box_band_store.box_data.is_empty() {
                    Cx::post_action(MyError::Zdyknown("没有数据，无法绑定!!!".to_string()));
                } else {
                    let datas = store.box_band_store.box_data.clone();
                    let carton = carton_input.text();
                    rt.spawn(async move {
                        let res = band_work(datas).await;
                        match res {
                            Ok(_) => {
                                let carton = carton.clone();
                                enqueue_popup_notification(PopupItem {
                                    kind: PopupKind::Success,
                                    auto_dismissal_duration: Some(2.5),
                                    message: format!("箱号: {} 绑定成功.", carton),
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
        if unband_btn.clicked(actions) {
            self.modal(ids!(unbind_modal)).open(cx);
        }
    }
}
