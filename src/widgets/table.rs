use makepad_widgets::*;

use crate::store::Store;


live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::widgets::row::*;
    RowHeaderLabel = <View> {
        width: 100
        height: Fit
        align: {x: 0.0, y: 0.5}
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
        height: 50,
        padding: {top: 10, bottom: 10, left: 20, right: 20}
        // Heads-up: the spacing and row header widths need to match the row values
        spacing: 30,
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return mix(#AFEEEE,#DCDCDC,self.pos.x);
            }
        }


        <RowHeaderLabel> {  label = {text: "箱号"} }
        <RowHeaderLabel> {  label = {text: "盒号"} }
        <RowHeaderLabel> {  label = {text: "Sn"} }
        <RowHeaderLabel> {  label = {text: "Ith"} }
        <RowHeaderLabel> {  label = {text: "Pf"} }
        <RowHeaderLabel> {  label = {text: "Se"} }
        <RowHeaderLabel> {  label = {text: "Sen"} }
        <RowHeaderLabel> {  label = {text: "测试时间"} }
    }
    pub InfosTable = {{InfosTable}} <RoundedView> {
            width: Fill,
            height: Fill,
            align: {x: 0.5, y: 0.5}
            flow: Down,
            // show_bg: true
            // draw_bg: {
            //     // color: #f9f9f9,
            //     border_radius: 5,
            //     // uniform shadow_color: #0001
            //     // shadow_radius: 12.0,
            //     // shadow_offset: vec2(0.0,-1.5)
            // }
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
    data: Store,
}

impl Widget for InfosTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(props) = scope.data.get::<Store>() {
            self.data = props.clone();
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let entries_count = self.data.clone().datas.len();
        let last_item_id = if entries_count > 0 { entries_count } else { 0 };
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, last_item_id);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < last_item_id {
                        let template = live_id!(ItemRow);
                        let item = list.item(cx, item_id, template);

                        let mut file_data = self.data.clone().datas[item_id].clone();
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
