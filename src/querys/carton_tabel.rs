use crate::{querys::works::carton_q_sn::get_carton_sn_datas, store::Store, structs::Datas};
use bb8_tiberius::ConnectionManager;
use makepad_widgets::*;
use tokio::runtime::Runtime;
use tracing::info;
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::querys::carton_row::*;
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
                                weight: 1.0
                            }, label = {text: "料号"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "装盒时间"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "装箱时间"} }
        <RowHeaderLabel> {width: Fill {
                                weight: 1.0
                            }, label = {text: "详细信息"} }
    }
    pub CartonTable = {{CartonTable}} <RoundedShadowView> {
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
#[derive(Debug)]
pub struct SnInfosAction {
    pub data: Vec<Datas>,
}

#[derive(Live, LiveHook, Widget)]
pub struct CartonTable {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
}

impl Widget for CartonTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(store) = scope.data.get::<Store>() {
                let entries_count = store.datas_store.query_datas.len();
                let last_item_id = if entries_count > 0 { entries_count } else { 0 };

                if let Some(mut list) = item.as_portal_list().borrow_mut() {
                    list.set_item_range(cx, 0, last_item_id);
                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < last_item_id {
                            let template = live_id!(ItemRow);
                            let item = list.item(cx, item_id, template);

                            let mut file_data = store.datas_store.query_datas[item_id].clone();
                            let mut scope = Scope::with_data(&mut file_data);
                            item.draw_all(cx, &mut scope);
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}
impl WidgetMatchEvent for CartonTable {
    fn handle_actions(&mut self, _cx: &mut Cx, e: &Actions, scope: &mut Scope) {
        let rt = self.rt.handle().clone();
        let list_widget = self.view.portal_list(ids!(list));
        let mut pool: Option<bb8::Pool<ConnectionManager>> = None;
        for (item_id, item_widget) in list_widget.items_with_actions(e) {
            if item_widget.button(ids!(info_button)).clicked(e) {
                // 4. 关键：通过 item_id (索引) 回溯数据
                if let Some(store) = scope.data.get::<Store>() {
                    let data_list = &store.datas_store.query_datas;
                    pool = store.pool.clone();

                    if item_id < data_list.len() {
                        let current_data = &data_list[item_id];
                        let carton_no = current_data.carton_data.carton_no.clone();
                        let d = current_data.carton_data.clone();
                        // --- 成功获取到当前行的数据 ---
                        info!("选中的箱号是: {}", carton_no);
                        rt.spawn(async move {
                            let o_d = d.clone();
                            let res = get_carton_sn_datas(carton_no, &pool.unwrap()).await;
                            match res {
                                Ok(mut r) => {
                                    let new_datas = r
                                        .iter_mut()
                                        .map(|s| {
                                            s.carton_data = o_d.clone();
                                            s.clone()
                                        })
                                        .collect::<Vec<Datas>>();
                                    Cx::post_action(SnInfosAction { data: new_datas });
                                }
                                Err(err) => {
                                    Cx::post_action(err);
                                }
                            }
                        });
                    }
                }
            }
        }
    }
}
