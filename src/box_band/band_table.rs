use crate::{box_band::work::query_work::BoxBandData, store::Store, utils::error::MyError};
use makepad_widgets::*;
use tracing::info;
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

    BandDataRow =
    // {{BandDataRow}} {
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
                // new_box_no = <Col> {
                //     width: Fill {
                //         weight: 1.0
                //     }
                // }
                <View> {
                    align: {x: 0.0, y: 0.5}
                    width: Fill {
                        weight: 1.0
                    }
                    new_box_no_input = <TextInput> {

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
    // }

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
            BandDataRow = <CachedView> {
                <BandDataRow> {}
            }

            // BandDataRow = <BandDataRow> {
            //     cursor: Default
            // }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BandTable {
    #[deref]
    view: View,
}

impl Widget for BandTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(store) = scope.data.get::<Store>() {
                let entries_count = store.box_band_store.box_data.clone().len();
                let last_item_id = if entries_count > 0 { entries_count } else { 0 };
                if let Some(mut list) = item.as_portal_list().borrow_mut() {
                    list.set_item_range(cx, 0, last_item_id);
                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < last_item_id {
                            let template = live_id!(BandDataRow);
                            let item = list.item(cx, item_id, template);

                            let file_data = store.box_band_store.box_data[item_id].clone();

                            let carton_nos = file_data.carton_no.clone();
                            let label = item.label(ids!(h_wrapper.carton_no.label));
                            label.set_text(cx, &carton_nos);

                            let box_nos = file_data.box_no.clone();
                            let label = item.label(ids!(h_wrapper.old_box_no.label));
                            label.set_text(cx, &box_nos.to_string());

                            let nwe_box_nos = file_data.new_box_no.clone();
                            // let label = self.label(ids!(new_box_no.label));
                            let input = item.text_input(ids!(h_wrapper.new_box_no_input));
                            input.set_text(cx, &nwe_box_nos);

                            let pns = file_data.pn.clone();
                            let label = item.label(ids!(h_wrapper.pn.label));
                            label.set_text(cx, &pns);

                            let bandtimes = file_data.create_time.clone();
                            let label = item.label(ids!(h_wrapper.bandtime.label));
                            label.set_text(cx, &bandtimes);
                            // let mut scope = Scope::with_data(&mut file_data);
                            item.draw_all(cx, &mut Scope::empty());
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}
impl WidgetMatchEvent for BandTable {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let list_widget = self.view.portal_list(ids!(list));
        for (item_id, item_widget) in list_widget.items_with_actions(actions) {
            let new_box_no = item_widget.text_input(ids!(new_box_no_input));
            if let Some(new_box_no) = new_box_no.changed(actions) {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    if let Some(band) = store.box_band_store.box_data.get_mut(item_id) {
                        band.new_box_no = new_box_no;
                    }
                }
            }
            if let Some((i, _)) = new_box_no.returned(actions) {
                if !i.is_empty() {
                    if let Some(store) = scope.data.get_mut::<Store>() {
                        if let Some(band) = store.box_band_store.box_data.get_mut(item_id) {
                            band.new_box_no = i;
                        }
                        let next_item_id = item_id + 1;
                        if next_item_id < store.box_band_store.box_data.len() {
                            let next_row_widget =
                                list_widget.item(cx, next_item_id, live_id!(BandDataRow));

                            next_row_widget
                                .text_input(ids!(h_wrapper.new_box_no_input))
                                .set_key_focus(cx);

                            list_widget.set_first_id_and_scroll(next_item_id, 0.0);
                        }
                    }
                } else {
                    Cx::post_action(MyError::Zdyknown(format!("输入不能为空!!!")));
                }
            }
        }
    }
}
