use bb8_tiberius::ConnectionManager;
use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{export::works::carton_query::do_carton_query, store::Store, structs::Datas};

live_design! {
    use makepad_widgets::base::*;
    use makepad_widgets::widget::*;
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub Line = <View> {
        width: Fill,
        height: 1,
        // padding: {top: 0, bottom: 0, left: 20, right: 20},
        // margin: {top: 0, bottom: 0, left: 20, right: 20},
        show_bg: true,
        draw_bg: {
            color: #1C1C1C,
        }
    }
    Col = <View> {
        width: Fit
        align: {x: 0.5, y: 0.5}
        label = <Label> {
            draw_text: {
                text_style: {font_size: 12},
                color: #1C1C1C,
            }
        }
    }
    pub DataRow = {{DataRow}} {
        flow: Overlay,
        width: Fill,
        height: Fit,
        <View> {
            height: 45,
            flow: Down
            width: Fill
            align: {x: 0.5, y: 0.5}
            animator: {
                    hover = {
                        default: off,

                        off = {
                            from: {
                                all: Forward {
                                    duration: 0.1,
                                },
                            },
                            apply: {
                                // width: 230,
                                height: 40,
                            },
                            redraw: true,
                        }

                        on = {
                            from: {
                                all: Forward {
                                    duration: 0.1,
                                },
                            },
                            apply: {
                                // width: 256,
                                height: 45,
                            },
                            redraw: true,
                        }
                    }
                }
            h_wrapper = <View> {
                flow: Right
                width: Fill
                // padding: {top: 10, bottom: 10, left: 20, right: 20}
                spacing: 30

                carton_no = <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                box_no =  <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                pn =  <Col> {width: Fill {
                                        weight: 1.0
                                    }}
                pack_time =  <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                carton_time =  <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                <View> {
                    align: {x: 0.5, y: 0.5}
                    width: Fill {
                        weight: 1.0
                    }
                    info_button = <Button> {
                        width: Fit
                        text: "详细信息"
                    }
                }
            }
            separator_line = <Line> {}
        }

    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DataRow {
    #[deref]
    view: View,
}

impl Widget for DataRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(data) = scope.data.get::<Datas>() {
            let carton_nos = data.carton_data.carton_no.clone();
            let label = self.label(ids!(h_wrapper.carton_no.label));
            label.set_text(cx, &carton_nos);

            let box_nos = data.pack_data.box_no.clone();
            let label = self.label(ids!(box_no.label));
            label.set_text(cx, &box_nos.to_string());

            let pn = data.carton_data.yypn.clone();
            let label = self.label(ids!(pn.label));
            label.set_text(cx, &pn);

            let pack_time = data.pack_data.pack_packtime.clone();
            let label = self.label(ids!(pack_time.label));
            label.set_text(cx, &pack_time);

            let carton_time = data.carton_data.carton_packtime.clone();
            let label = self.label(ids!(carton_time.label));
            label.set_text(cx, &carton_time);
        };

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}
