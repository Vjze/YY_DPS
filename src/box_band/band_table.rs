use makepad_widgets::*;

use crate::{box_band::work::query_work::BoxBandData, store::Store};
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::widgets::row::*;

    pub Line = <View> {
        width: Fill,
        height: 1,
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

    BandDataRow = {{BandDataRow}} {
        <View> {
            height: 45,
            flow: Down
            width: Fill
            align: {x: 0.0, y: 0.5}

            h_wrapper = <View> {
                flow: Right
                spacing: 30

                carton_no = <Col> {
                    width: Fill {
                        weight: 1.0
                    }
                }
                old_box_no = <Col> {
                    width: Fill {
                        weight: 1.0
                    }
                }
                new_box_no = <Col> {
                    width: Fill {
                        weight: 1.0
                    }
                }

                pn = <Col> {
                    width: Fill {
                        weight: 1.0
                    }
                }

                bandtime = <Col> {
                    width: Fill {
                        weight: 1.0
                    }
                }
            }
            separator_line = <Line> {}
        }
    }

    pub BandTable = {{BandTable}} <RoundedShadowView> {
        width: Fill,
        height: Fill,
        show_bg: true
        draw_bg: {
            color: (MAIN_BG_COLOR),
            border_radius: 5
            uniform shadow_color: #0001
            shadow_radius: 12.0,
            shadow_offset: vec2(0.0, -1.5)
        }
        list = <PortalList> {
            drag_scrolling: false
            BandDataRow = <BandDataRow> {
                cursor: Default
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BandTable {
    #[deref]
    view: View,
    #[rust]
    data: Vec<BoxBandData>,
}

impl Widget for BandTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(props) = scope.data.get::<Store>() {
            // info!("BandTable update data");
            self.data = props.box_band_store.box_data.clone();
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let entries_count = self.data.clone().len();
        let last_item_id = if entries_count > 0 { entries_count } else { 0 };
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, last_item_id);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < last_item_id {
                        let template = live_id!(BandDataRow);
                        let item = list.item(cx, item_id, template);

                        let mut file_data = self.data[item_id].clone();
                        let mut scope = Scope::with_data(&mut file_data);
                        item.draw_all(cx, &mut scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}
impl WidgetMatchEvent for BandTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}

#[derive(Live, LiveHook, Widget)]
pub struct BandDataRow {
    #[deref]
    view: View,
}

impl Widget for BandDataRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(data) = scope.data.get::<BoxBandData>() {
            let carton_nos = data.carton_no.clone();
            let label = self.label(id!(h_wrapper.carton_no.label));
            label.set_text(cx, &carton_nos);

            let box_nos = data.box_no.clone();
            let label = self.label(id!(old_box_no.label));
            label.set_text(cx, &box_nos.to_string());

            let nwe_box_nos = data.new_box_no.clone();
            let label = self.label(id!(new_box_no.label));
            label.set_text(cx, &nwe_box_nos);

            let pns = data.pn.clone();
            let label = self.label(id!(pn.label));
            label.set_text(cx, &pns);


            let bandtimes = data.create_time.clone();
            let label = self.label(id!(bandtime.label));
            label.set_text(cx, &bandtimes);
        };

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for BandDataRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}
