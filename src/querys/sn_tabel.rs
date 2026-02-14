use crate::{store::Store, structs::Datas};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::querys::sn_row::*;
    RowHeaderLabel = <View> {
        // width: 100,
        height: Fit
        align: {x: 0.5, y: 0.5  }
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
    HeaderRow = <View> {
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

        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "箱号"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "盒号"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "Sn"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 1.0
                            }, label = {text: "Pn"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Ith"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Po"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Se"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Vf"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Im"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Rs"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Res"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Sen"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Icc"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Idark"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "I_Xtalk"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "kink"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "ImKink"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "测试时间"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "装盒时间"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "装箱时间"} }
    }
    pub SnInfosTable = {{SnInfosTable}} <ScrollXYView> {
            width: Fill,
            height: Fill,
            show_bg: true
            draw_bg: {
                color: (MAIN_BG_COLOR),
                // border_radius: 5
                // uniform shadow_color: #0001
                // shadow_radius: 12.0,
                // shadow_offset: vec2(0.0,-1.5)
            }
            flow: Down,
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
pub struct SnInfosTable {
    #[deref]
    view: View,
    #[rust]
    data: Vec<Datas>,
}

impl Widget for SnInfosTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            // if let Some(store) = scope.data.get::<Store>() {
            let datas = self.data.clone();
            let entries_count = datas.len();
            let last_item_id = if entries_count > 0 { entries_count } else { 0 };

            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, last_item_id);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < last_item_id {
                        let template = live_id!(ItemRow);
                        let item = list.item(cx, item_id, template);

                        let mut file_data = datas[item_id].clone();
                        let mut scope = Scope::with_data(&mut file_data);
                        item.draw_all(cx, &mut scope);
                    }
                }
            }
            // }
        }
        DrawStep::done()
    }
}
impl WidgetMatchEvent for SnInfosTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _e: &Actions, _scope: &mut Scope) {}
}
impl SnInfosTable {
    fn set_data(&mut self, _cx: &mut Cx, data: Vec<Datas>) {
        self.data = data;
    }
}

impl SnInfosTableRef {
    pub fn set_data(&self, cx: &mut Cx, data: Vec<Datas>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(cx, data);
        }
    }
}
