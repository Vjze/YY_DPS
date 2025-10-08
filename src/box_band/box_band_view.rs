use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    box_band::work::{band_work::band_work, query_work::query_carton_info},
    store::Store,
    utils::error::MyError,
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;
    use crate::box_band::band_table::*;

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
        <Label> {
            text: "新盒号:"
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
        new_box_no_input = <MolyTextInput> {
            empty_text: "请输入新盒号...."
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
                uniform color: #FF7F50
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
        flow: Down,
        <FirstRow> {}
        <BandHeaderRow> {
            cursor: Default
        }
        <BandTable> {}
    }
}
#[derive(Live, LiveHook, Widget)]
struct BoxBandView {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
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

impl WidgetMatchEvent for BoxBandView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let query_btn = self.view.button(id!(query_btn));
        let band_btn = self.view.button(id!(band_btn));
        let carton_input = self.view.text_input(id!(carton_input));
        let boxs_num = self.view.label(id!(boxs_num));
        let new_box_input = self.view.text_input(id!(new_box_no_input));
        if let Some(input) = new_box_input.changed(actions) {
            if let Some(props) = scope.data.get_mut::<Store>() {
                let datas = props.box_data.clone();
                let len = datas.len();
                let mut new_box_nos = vec![];
                for i in 1..len + 1 {
                    let mut new_data = datas.get(i - 1).unwrap().clone();
                    let padded_number = format!("{:03}", i);
                    let new_box = format!("{}-{}", input, padded_number);
                    new_data.new_box_no = new_box.clone();
                    new_box_nos.push(new_data);
                }
                props.box_data = new_box_nos;
                // info!("props.box_data: {:?}", props.box_data);
            }
        }
        let rt = self.rt.handle().clone();
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
            if carton_input.text().is_empty() {
                Cx::post_action(MyError::Zdyknown("请输入箱号!!!".to_string()));
            } else {
                let _guard = rt.enter();
                let carton = carton_input.clone().text();
                if let Some(store) = scope.data.get_mut::<Store>() {
                    rt.block_on(async move {
                        let carton_input = carton;
                        let res = query_carton_info(carton_input).await;
                        match res {
                            Ok(data) => {
                                let num = format!("一共: {} 盒", data.len());
                                boxs_num.set_text(cx, &num);
                                store.box_data = data;
                            }
                            Err(e) => {
                                Cx::post_action(e);
                            }
                        }
                    });
                }
            }
        }
        if band_btn.clicked(actions) {
            if let Some(store) = scope.data.get::<Store>() {
                if store.box_data.is_empty() {
                    Cx::post_action(MyError::Zdyknown("没有数据，无法绑定!!!".to_string()));
                } else {
                    let _guard = rt.enter();
                    let datas = store.box_data.clone();
                    let carton = carton_input.text();
                    rt.block_on(async move {
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
    }
}
