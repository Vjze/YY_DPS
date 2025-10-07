use makepad_widgets::*;

use crate::{data_import::work::extract_data::ImportDBDatas, store::Store};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::data_import::import_row::*;
    RowHeaderLabel = <View> {
        width: 100,
        height: Fit
        align: {x: 0.5, y: 0.5  }
        label = <Label> {
            width: Fit
            draw_text: {
                text_style: {
                    font_size: 15,
                }
                color: #1C1C1C,
            }
        }
    }
    HeaderRow = <RoundedView> {
        width: Fill
        height: Fit,
        spacing: 30,
        <RowHeaderLabel> {width: Fill, label = {text: "Sn"} }
        <RowHeaderLabel> {width: 100, label = {text: "温度"} }
        <RowHeaderLabel> {width: 100, label = {text: "Ith"} }
        <RowHeaderLabel> {width: 100, label = {text: "Se"} }
        <RowHeaderLabel> {width: 100, label = {text: "Po"} }
        <RowHeaderLabel> {width: 100, label = {text: "Vf"} }
        <RowHeaderLabel> {width: 100, label = {text: "im"} }
        <RowHeaderLabel> {width: 100, label = {text: "Sen"} }
        <RowHeaderLabel> {width: 100, label = {text: "icc"} }
    }
    pub ImportTable = {{ImportTable}} <RoundedShadowView> {
            width: Fill,
            height: Fill,
            show_bg: true
            draw_bg: {
                color: #F3F3F3,
                border_radius: 5
                uniform shadow_color: #0001
                shadow_radius: 12.0,
                shadow_offset: vec2(0.0,-1.5)
            }
            flow: Down,
            HeaderRow = <HeaderRow> {
                cursor: Default
            }
            list = <PortalList> {
                drag_scrolling: false

                ItemRow = <ImportRow> {
                    cursor: Default
                }
            }


    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImportTable {
    #[deref]
    view: View,
    #[rust]
    datas: Vec<ImportDBDatas>
}

impl Widget for ImportTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get::<Store>() {
            self.datas = store.import_datas.data.clone()
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            let entries_count = self.datas.len();
            let last_item_id = entries_count;
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, last_item_id);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < last_item_id {
                        let template = live_id!(ItemRow);
                        let item = list.item(cx, item_id, template);
                        // let mut file_data = state.import_datas.data[item_id].clone();
                        let mut file_data = self.datas[item_id].clone();
                        let mut scope = Scope::with_data(&mut file_data);
                        item.draw_all(cx, &mut scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}
impl WidgetMatchEvent for ImportTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _e: &Actions, _scope: &mut Scope) {}
}
