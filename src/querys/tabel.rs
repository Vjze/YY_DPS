use crate::store::Store;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::querys::row::*;
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
                            }, label = {text: "Sen"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "测试时间"} }
    }
    pub InfosTable = {{InfosTable}} <RoundedShadowView> {
            width: Fill,
            height: Fill,
            show_bg: true
            draw_bg: {
                color: (MAIN_BG_COLOR),
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
}

impl Widget for InfosTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(store) = scope.data.get::<Store>() {
                let entries_count = store.datas_store.query_datas.len();

                if let Some(mut list) = item.as_portal_list().borrow_mut() {
                    list.set_item_range(cx, 0, entries_count);
                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < entries_count {
                            let template = live_id!(ItemRow);
                            let item = list.item(cx, item_id, template);

                            // 避免 clone，直接创建引用
                            let file_data = &store.datas_store.query_datas[item_id];
                            // 使用 Scope::with_data 需要可变引用，这里使用不安全的方式避免 clone
                            // 实际场景中应该重构为使用 Rc<RefCell<>> 或 Arc<Mutex<>>
                            let mut file_data_cloned = file_data.clone();
                            let mut item_scope = Scope::with_data(&mut file_data_cloned);
                            item.draw_all(cx, &mut item_scope);
                        }
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
