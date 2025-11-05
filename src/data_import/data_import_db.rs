use std::{path::PathBuf, sync::Arc};

use makepad_widgets::*;
use tokio::runtime::Runtime;
use tracing::info;

use crate::{
    data_import::{DataImport, work::extract_data::ImportDBDatas},
    store::Store,
    widgets::popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::data_import::import_table::*;
    use crate::widgets::popup_list::*;
    ICON_LOGO = dep("crate://self/resources/images/logo.png")

    pub DataImportDb = {{DataImportDb}} {
        <View> {
            width: Fill,
            height: Fill
            spacing: 15.0,
            padding: 10,
            flow: Overlay,
            <View> {
                flow: Down,
                spacing: 10,

                <View> {
                    width: Fill,
                    height: Fit
                    spacing: 20,
                    align: {y: 0.5}
                    select_btn = <Button> {
                        width: 100,
                        height: 40,
                        text: "选择文件",
                        draw_text: {
                            color: #000000,
                            color_hover: #000000,
                            text_style: <THEME_FONT_REGULAR> {
                                font_size: 14
                            }
                        }
                        draw_bg: {
                            color: #708090,
                            color_hover: #B0E0E6,
                            color_down: #A0A0A0,
                            uniform border_radius: 5.0,
                        }
                    }
                    <Label> {
                        width: Fit,
                        height: Fit,
                        text: "料号:",
                        draw_text: {
                            color: #000000,
                            text_style: <THEME_FONT_REGULAR> {
                                font_size: 16
                            }
                        }
                    }
                    pn = <TextInput> {
                        height: Fill,
                        is_read_only: true,
                        empty_text: "选择文件后自动获取",
                        draw_text: {
                            color: #000000,
                            color_hover: #000000,
                            color_focus: #000000,
                            color_empty_focus: #000000,
                            color_empty_hover: #000000,
                            color_empty: #808080,
                            color_disabled: #A9A9A9,
                            text_style: <THEME_FONT_REGULAR> {
                                font_size: 14
                            }
                        }
                        draw_bg: {
                            color: #A9A9A9,
                            color_hover: #A9A9A9,
                            color_focus: #A9A9A9,
                            color_empty: #A9A9A9,
                            color_down: #A9A9A9,
                            uniform border_radius: 5.0,
                            uniform border_size: 1.0,
                            uniform border_color: #A9A9A9,
                            uniform border_color_focus: #4169E1,
                        }
                        padding: {left: 10, top: 8}
                    }
                    qty_label = <Label> {
                        width: Fit,
                        height: Fit,
                        text: "总数量: 0 PCS",
                        draw_text: {
                            color: #000000,
                            text_style: <THEME_FONT_REGULAR> {
                                font_size: 16
                            }
                        }
                    }
                    action_btn = <Button> {
                        width: 100,
                        height: 40,
                        text: "写入数据",
                        draw_text: {
                            color: #000000,
                            color_hover: #000000,
                            text_style: <THEME_FONT_REGULAR> {
                                font_size: 14
                            }
                        }
                        draw_bg: {
                            color: #708090,
                            color_hover: #B0E0E6,
                            color_down: #A0A0A0,
                            color_disabled: #615c5cff,
                            uniform border_radius: 5.0,
                        }
                    }
                }
                <ImportTable> {}
            }
            <PopupList> {}
        }
    }
}

#[derive(Live, Widget)]
pub struct DataImportDb {
    #[deref]
    pub view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust(None)] // 默认初始化为 None
    pub import_processor: Option<Arc<dyn DataImport>>,
}
#[derive(Clone, Debug, Default)]
pub struct DataExtractedAction {
    data: DbData,
}
#[derive(Debug, Default, Clone)]
pub struct DbData {
    pub data: Vec<ImportDBDatas>,
    pub pn: String,
    pub file_path: PathBuf,
}
impl LiveHook for DataImportDb {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.import_processor = Some(crate::data_import::new_import_processor());
    }
}
impl Widget for DataImportDb {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for DataImportDb {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let select_btn = self.button(ids!(select_btn));
        let action_btn = self.button(ids!(action_btn));
        let qty_label = self.label(ids!(qty_label));
        for action in actions {
            if let Some(data_action) = action.downcast_ref::<DataExtractedAction>() {
                if let Some(store) = scope.data.get_mut::<Store>() {
                    let qty = data_action.data.data.len();
                    store.import_store.import_datas = data_action.data.clone();
                    info!(
                        "数据提取完成，提取数据量: {}",
                        store.import_store.import_datas.data.len()
                    );

                    // UI 刷新
                    self.view
                        .text_input(ids!(pn))
                        .set_text(cx, &store.import_store.import_datas.pn);
                    qty_label.set_text(cx, &format!("总数量: {} PCS", &qty));

                    enqueue_popup_notification(PopupItem {
                        kind: PopupKind::Success,
                        auto_dismissal_duration: Some(2.5),
                        message: "数据提取完成".to_string(),
                    });
                }
            }
        }

        let rt = self.rt.handle().clone();
        if select_btn.clicked(actions) {
            info!("开始选择文件 (异步)");
            if let Some(scope) = scope.data.get_mut::<Store>() {
                scope.import_store.import_datas = DbData::default();
                qty_label.set_text(cx, "总数量: 0 PCS");
            }
            let procrssor = self.import_processor.as_ref().unwrap().clone();

            // 3. 启动异步任务，不阻塞 UI
            rt.spawn(async move {
                // 1. 异步文件选择
                let path_result = procrssor.select_file().await;
                info!("选择文件结果: {:?}", path_result);
                // let path_result = select_file_sync();

                let p = match path_result {
                    Ok(p) => p,
                    Err(e) => {
                        Cx::post_action(e); // 从异步线程 post 错误
                        return;
                    }
                };

                // 2. 异步提取数据
                let file_name = match p.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        info!("Invalid file path encoding");
                        // TODO: Post 一个特定的错误 Action
                        return;
                    }
                };
                let res = procrssor.extract(&file_name).await;
                match res {
                    Ok(r) => {
                        // 3. 准备数据
                        let f = p.file_name().unwrap().display().to_string();
                        let pn = f[..8].to_string();
                        let file_path = p;
                        let data = DbData {
                            data: r.clone(),
                            pn,
                            file_path,
                        };

                        // 4. Post 包含数据的成功 Action 回 UI 线程
                        Cx::post_action(DataExtractedAction { data });
                    }
                    Err(e) => {
                        Cx::post_action(e); // Post 错误
                    }
                }
            });
        }

        if action_btn.clicked(actions) {
            info!("开始写入数据");
            let processor = self.import_processor.as_ref().unwrap().clone();
            if let Some(store) = scope.data.get::<Store>() {
                let data = store.import_store.import_datas.clone();
                let _ = rt.spawn(async move {
                    let res = processor.write(data).await;
                    match res {
                        Ok(_) => {
                            // enqueue_popup_notification 可能是线程安全的（因为它不接受 &mut Cx）
                            enqueue_popup_notification(PopupItem {
                                kind: PopupKind::Success,
                                auto_dismissal_duration: Some(3.0),
                                message: "数据写入成功".to_string(),
                            });
                            info!("数据写入成功");
                        }
                        Err(e) => {
                            Cx::post_action(e); // 确保使用线程安全的 post
                        }
                    };
                });
            }
        }
        cx.redraw_all();
    }
}
