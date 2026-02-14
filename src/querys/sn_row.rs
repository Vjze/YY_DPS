use std::collections::HashMap;

use makepad_widgets::*;

use crate::structs::Datas;

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
                pn =  <Col> {width: Fill {
                                        weight: 1.0
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
                vf =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                im =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                rs =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                res =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                sen =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                icc =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                idark =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                i_xtalk =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                kink =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                imkink =  <Col> {width: Fill {
                                        weight: 0.8
                                    }}
                testtime = <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                packtime = <Col> {width: Fill {
                                        weight: 2.0
                                    }}
                cartontime = <Col> {width: Fill {
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
        if let Some(data) = scope.data.get::<Datas>() {
            let carton_nos = data.carton_data.carton_no.clone();
            let label = self.label(ids!(h_wrapper.carton_no.label));
            label.set_text(cx, &carton_nos);

            let box_nos = data.pack_data.box_no.clone();
            let label = self.label(ids!(box_no.label));
            label.set_text(cx, &box_nos.to_string());

            let sn = data.sn_data.sn.clone();
            let label = self.label(ids!(sn.label));
            label.set_text(cx, &sn);

            let pn = data.carton_data.yypn.clone();
            let label = self.label(ids!(pn.label));
            label.set_text(cx, &pn);

            let ith = data.sn_data.ith.clone();
            let label = self.label(ids!(ith.label));
            label.set_text(cx, &ith);

            let po = data.sn_data.po.clone();
            let label = self.label(ids!(pf.label));
            label.set_text(cx, &po);

            let se = data.sn_data.se.clone();
            let label = self.label(ids!(se.label));
            label.set_text(cx, &se);

            let vf = data.sn_data.vf.clone();
            let label = self.label(ids!(vf.label));
            label.set_text(cx, &vf);

            let im = data.sn_data.im.clone();
            let label = self.label(ids!(im.label));
            label.set_text(cx, &im);

            let rs = data.sn_data.rs.clone();
            let label = self.label(ids!(rs.label));
            label.set_text(cx, &rs);

            let res = data.sn_data.res.clone();
            let label = self.label(ids!(res.label));
            label.set_text(cx, &res);

            let sen = data.sn_data.sen.clone();
            let label = self.label(ids!(sen.label));
            label.set_text(cx, &sen);

            let icc = data.sn_data.icc.clone();
            let label = self.label(ids!(icc.label));
            label.set_text(cx, &icc);

            let idark = data.sn_data.idark.clone();
            let label = self.label(ids!(idark.label));
            label.set_text(cx, &idark);

            let i_xtalk = data.sn_data.i_xtalk.clone();
            let label = self.label(ids!(i_xtalk.label));
            label.set_text(cx, &i_xtalk);

            let kink = data.sn_data.kink.clone();
            let label = self.label(ids!(kink.label));
            label.set_text(cx, &kink);

            let imkink = data.sn_data.imkink.clone();
            let label = self.label(ids!(imkink.label));
            label.set_text(cx, &imkink);

            let testtime = data.sn_data.testdate.clone().to_string();
            let label = self.label(ids!(testtime.label));
            label.set_text(cx, &testtime);

            let pack_time = data.pack_data.pack_packtime.clone();
            let label = self.label(ids!(packtime.label));
            label.set_text(cx, &pack_time);

            let carton_time = data.carton_data.carton_packtime.clone();
            let label = self.label(ids!(cartontime.label));
            label.set_text(cx, &carton_time);
        };

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}
