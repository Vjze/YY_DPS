use std::collections::HashMap;

use makepad_widgets::*;

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
            align: {x: 0.0, y: 0.5}

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
                sn =  <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                ith =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                pf =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                se =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                sen =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                testtime = <Col> {width: Fill {
                                        weight: 2.0
                                    }}
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
        if let Some(data) = scope.data.get::<HashMap<String, String>>() {
            let carton_nos = data
                .get("carton_no")
                .cloned()
                .unwrap_or_else(|| "".to_string());
            let label = self.label(id!(h_wrapper.carton_no.label));
            label.set_text(cx, &carton_nos);

            let box_nos = data
                .get("box_no")
                .cloned()
                .unwrap_or_else(|| "".to_string());
            let label = self.label(id!(box_no.label));
            label.set_text(cx, &box_nos.to_string());

            let sns = data.get("sn").cloned().unwrap_or_else(|| "".to_string());
            let label = self.label(id!(sn.label));
            label.set_text(cx, &sns);

            let iths = data.get("ith").cloned().unwrap_or_else(|| "".to_string());
            let label = self.label(id!(ith.label));
            label.set_text(cx, &iths);

            let pfs = data.get("po").cloned().unwrap_or_else(|| "".to_string());
            let label = self.label(id!(pf.label));
            label.set_text(cx, &pfs);

            let ses = data.get("se").cloned().unwrap_or_else(|| "".to_string());
            let label = self.label(id!(se.label));
            label.set_text(cx, &ses);

            let sents = data.get("sen").cloned().unwrap_or_else(|| "".to_string());
            let label = self.label(id!(sen.label));
            label.set_text(cx, &sents);

            let testtimes = data
                .get("testtime")
                .cloned()
                .unwrap_or_else(|| "".to_string());
            let label = self.label(id!(testtime.label));
            label.set_text(cx, &testtimes);
        };

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}
