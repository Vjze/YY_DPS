use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{box_band::work::query_work::query_carton_info, store::Store, utils::error::MyError};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;

    FirstRow = <View> {
        width: Fill,
        height: Fit,
        spacing:20,

        <Label> {
            text: "箱号:"
            width: Fit, height: 40
            padding: {top:5}
            draw_text: {
                color: #000000,
                text_style: {
                    font_size:16
                }
            }
        }
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

        band_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "盒号绑定"
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
        boxs_num = <Label> {
            padding: {
                top:5
            }
            text: "一共: 0 盒"
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
    }
    BandRowHeaderLabel = <View> {
        // width: 100,
        height: Fit
        align: {x: 0.0, y: 0.5  }
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
            }, label = {text: "箱号"} }
        <BandRowHeaderLabel> {
             width: Fill {
                                weight: 1.0
                            }, label = {text: "旧盒号"} }      
        <BandRowHeaderLabel> { width: Fill {
                                weight: 1.0
                            } label = {text: "新盒号"} }
        <BandRowHeaderLabel> { width: Fill {
                                weight: 1.0
                            } label = {text: "装盒时间"} }
    }
    pub BoxBandView = {{BoxBandView}} {
        width: Fill,
        height: Fill,
        padding: 15,
        spacing: 10,
        flow: Down,
        <FirstRow> {},
        BandHeaderRow = <BandHeaderRow> {
                cursor: Default
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
        let rt = self.rt.handle().clone();
        if carton_input.text().is_empty() {
            query_btn.set_disabled(cx, true);
            band_btn.set_disabled(cx, true);
        } else {
            query_btn.set_disabled(cx, false);
            band_btn.set_disabled(cx, false);
        }
        if query_btn.clicked(actions) {
            if carton_input.text().is_empty() {
                Cx::post_action(MyError::Zdyknown("请输入箱号!!!".to_string()));
            } else {
                let _guard = rt.enter();
                if let Some(store) = scope.data.get_mut::<Store>() {
                    rt.block_on(async move {
                        let res = query_carton_info(carton_input.text().clone()).await;
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
    }
}
