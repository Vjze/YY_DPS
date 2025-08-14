use makepad_widgets::*;
use std::collections::HashMap;
use crate::store::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::widgets::row::*;
    RowHeaderLabel = <View> {
        width: 100,
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
    HeaderRow = <RoundedView> {
        align: {x: 0.0, y: 0.5}
        width: Fill
        height: Fit,
        // padding: {top: 10, bottom: 10, left: 20, right: 20}
        // Heads-up: the spacing and row header widths need to match the row values
        spacing: 30,
        show_bg: true
        draw_bg: {
            color: #F2F4F7;
        }


        <RowHeaderLabel> {width: 180, label = {text: "箱号"} }
        <RowHeaderLabel> {width: 180, label = {text: "盒号"} }      
        <RowHeaderLabel> {width: 180, label = {text: "Sn"} }
        <RowHeaderLabel> {width: 70, label = {text: "Ith"} }
        <RowHeaderLabel> {width: 70, label = {text: "Po"} }
        <RowHeaderLabel> {width: 70, label = {text: "Se"} }
        <RowHeaderLabel> {width: 70, label = {text: "Sen"} }
        <RowHeaderLabel> {width: 300, label = {text: "测试时间"} }
    }
    pub InfosTable = {{InfosTable}} <RoundedShadowView> {
            width: Fill,
            height: Fill,
            show_bg: true
            draw_bg: {
                color: (MAIN_BG_COLOR)
                border_radius: 5
                uniform shadow_color: #0001
                shadow_radius: 12.0,
                shadow_offset: vec2(0.0,-1.5)
            }
            flow: Down,
            padding: {left:15}
            HeaderRow = <HeaderRow> {
                cursor: Default
            }
            list = <PortalList> {
                drag_scrolling: false

                ItemRow = <DataRow> {
                    cursor: Default
                }
            }


    }
}

#[derive(Live, LiveHook, Widget)]
pub struct InfosTable {
    #[deref]
    view: View,
    #[rust]
    data: Vec<HashMap<String, String>>,
}

impl Widget for InfosTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(props) = scope.data.get::<Store>() {
            if let Some(datas) = props.datas.clone() {
                self.data = datas.clone();
            }

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
                        let template = live_id!(ItemRow);
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
impl WidgetMatchEvent for InfosTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _e: &Actions, _scope: &mut Scope) {}
}
