use makepad_widgets::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tracing::info;

use crate::{
    export::Exportable,
    store::Store,
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};
live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::modal::*;
    use crate::shared::widgets::*;
    use crate::export::export_tabel::*;
    FirstRow = <View> {
        width: Fill,
        height: Fit,
        spacing:20,
        // show_bg: true,
        // draw_bg: {
        //     color: #D9D9D9
        // }
        type_selector = <DropDown> {
            width: 100,height:40
            labels:["type_1","type_2","type_3"],
            padding: {top:12,left:15}
            draw_text: {
                uniform color: #000
                uniform color_down: #000
                uniform color_hover: #000
                uniform color_focus: #000
            }
            draw_bg: {
                color: (MAIN_BG_COLOR_DARK),
                uniform border_radius:5.0,
                uniform border_size: 1.0
                uniform border_color_1: #333
                uniform border_color_1_hover: #555
                // uniform shadow_color: #0002
                // shadow_radius: 9.0,
                // shadow_offset: vec2(0.0,-2.0)
            }
            popup_menu: <PopupMenu> {
                // 自定义菜单背景
                draw_bg: { uniform color: #333, uniform border_color: #666,
                    uniform border_size: 1.0 }
                menu_item: <PopupMenuItem> {
                    // 自定义菜单项
                    padding: {left: 20, top: 8, bottom: 8, right: 10}
                    draw_bg: {
                        color: #333
                        color_hover: #555 // 悬停背景
                        color_active: #888 // 选中背景
                    }
                    draw_text: {
                        color: #EEE // 默认文字颜色
                        color_hover: #FFF // 悬停文字颜色
                        color_active: #FFF // 选中文字颜色
                    }
                }
            }
        }
        carton_input = <MolyTextInput> {
            empty_text: "请输入箱号...."
            width: Fill, height: 40
            padding: 10,
            draw_text: {
                text_style: <REGULAR_FONT>{
                    font_size: 12
                }
                color: #000
            }
            draw_bg: {
                uniform border_radius: 5.0
                uniform border_size: 1.0
            }
            draw_cursor: {
                uniform color: #FFF
            }
        }
        query_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "箱号查询"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size:16
                }
            }
            draw_bg: {
                uniform border_size: 1.0
                uniform border_radius: 5.0
                uniform color: #FF7F50
                uniform color_hover: #FFB6C1
                uniform color_disabled: #A9A9A9
            }
        }
        export_btn = <Button> {
            width: Fit
            height: 40
            padding: {left: 20, right: 20, top: 0, bottom: 0}
            text: "数据导出"
            draw_text: {
                color: #000000,
                text_style: {
                    font_size:16
                }
            }
            draw_bg: {
                uniform border_size: 1.0
                uniform border_radius: 5.0
                uniform color: #AFEEEE
                uniform color_hover: #9370DB
                uniform color_disabled: #DCDCDC
            }
        }


        qty_label = <Label> {
            padding: {
                top:5
            }
            text: "总数量: 0 PCS"
            draw_text: {
                color: #000,
                text_style: {
                    font_size:16
                }
            }
        }
    }

    pub ExportScreen = {{ExportScreen}} {
        <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            padding: 15,
            spacing: 10,
            <FirstRow> {}
            <ExTable> {}
        }
    }
}
#[derive(Live, Widget)]
pub struct ExportScreen {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust(None)] // 默认初始化为 None
    pub export_processor: Option<Arc<dyn Exportable>>,
}
#[derive(Clone, Debug, Default)]
pub struct ExportAction {
    pub data: Vec<HashMap<String, String>>,
}

impl LiveHook for ExportScreen {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.export_processor = Some(crate::export::new_export_processor());
    }
}
impl Widget for ExportScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(store) = scope.data.get::<Store>() {
            self.view
                .drop_down(ids!(type_selector))
                .set_labels(cx, store.setting_store.types.clone());
            if store.datas_store.export_datas.is_empty() {
                self.view.button(ids!(export_btn)).set_disabled(cx, true);
            } else {
                self.view.button(ids!(export_btn)).set_disabled(cx, false);
            }
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ExportScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let input = self.view.text_input(ids!(carton_input));
        let query_btn = self.view.button(ids!(query_btn));
        let export_btn = self.view.button(ids!(export_btn));
        let type_name = self.view.drop_down(ids!(type_selector));
        let qty_label = self.label(ids!(qty_label));
        let rt = self.rt.handle().clone();
        for action in actions {
            if let Some(data_action) = action.downcast_ref::<ExportAction>() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.datas_store.export_datas = data_action.data.clone();
                    let qty = format!("总数量: {} PCS", data_action.data.len());
                    qty_label.set_text(cx, &qty);
                    enqueue_popup_notification(PopupItem {
                        kind: PopupKind::Success,
                        auto_dismissal_duration: Some(2.5),
                        message: "查询完成，可以进行导出.".to_string(),
                    });
                }
            }
        }
        if input.text().is_empty() {
            query_btn.set_text(cx, "批量查询");
        } else {
            query_btn.set_text(cx, "箱号查询");
        }

        if query_btn.clicked(actions) {
            let mut pool = None;
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.datas_store.export_datas.clear();
                qty_label.set_text(cx, "总数量: 0 PCS");
                pool = store.pool.clone();
            }
            let processor = self.export_processor.as_ref().unwrap().clone();
            let carton = input.text().clone();
            let is_multi = query_btn.text() == "批量查询";
            let type_name = type_name.selected_label().clone();
            rt.spawn(async move {
                let res = processor
                    .carton_query(carton, type_name, is_multi, &pool.unwrap())
                    .await;

                match res {
                    Ok(data) => {
                        Cx::post_action(ExportAction { data });
                    }
                    Err(e) => {
                        Cx::post_action(e);
                    }
                }
            });
        }
        if export_btn.clicked(actions) {
            info!("开始导出数据...");
            let processor = self.export_processor.as_ref().unwrap().clone();
            if let Some(store) = scope.data.get_mut::<Store>() {
                info!("导出数据数量: {}", store.datas_store.export_datas.len());
                if !store.datas_store.export_datas.is_empty() {
                    let type_name = type_name.clone().selected_label();
                    let data = store.datas_store.export_datas.clone();
                    rt.spawn(async move {
                        let res = processor.export(type_name, data).await;
                        match res {
                            Ok(_) => {
                                enqueue_popup_notification(PopupItem {
                                    kind: PopupKind::Success,
                                    auto_dismissal_duration: Some(2.5),
                                    message: "数据导出完成.".to_string(),
                                });
                            }
                            Err(e) => {
                                Cx::post_action(e);
                            }
                        }
                    });
                }
            }
        }
    }
}
