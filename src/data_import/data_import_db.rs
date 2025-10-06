use std::path::PathBuf;

use makepad_widgets::*;
use tokio::runtime::Runtime;
use tracing::info;

use crate::{
    data_import::work::{
        extract_data::{ImportDBDatas, extract_data},
        select_file::select_file,
        write_data::write_data_to_db,
    },
    store::Store,
    utils::error::MyError,
    widgets::{
        dialog::ErrprModalAction,
        popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
    },
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::widgets::table::*;
    use crate::widgets::popup_list::*;
    ICON_LOGO = dep("crate://self/resources/images/logo.png")

    pub DataImportDb = {{DataImportDb}} {
                    <RoundedShadowView> {
                        width: Fill, height: Fill
                        show_bg: true
                        draw_bg: {
                            color: #DCDCDC,
                            border_radius: 8.5,
                            uniform shadow_color: #0003
                            shadow_radius: 18.0,
                            shadow_offset: vec2(0.0,-1.5)
                        }
                        spacing: 15.0,
                        padding: 10,
                        flow: Overlay,
                        <View> {
                            flow: Down,
                            spacing: 10,
                            <View> {
                                height:60,
                                align:{y:0.5},
                                logo = <Image> {
                                    width: 50, height: 50,
                                    source: (ICON_LOGO),
                                }
                                <H1> {
                                    width: Fit,
                                    height: Fit,
                                    margin: {left:10},
                                    text: "数据写入工具",
                                    draw_text: {
                                        color: #000000,
                                        text_style: {
                                            font_size: 24
                                        }
                                    }
                                }
                                <View> {
                                    width: Fill,
                                    height:Fit
                                    }

                                <Label> {
                                    width: Fit,
                                    height: Fit,
                                    text: "当前版本:",
                                    draw_text: {
                                        color: #000000,
                                        text_style: {
                                            font_size: 16
                                        }
                                    }
                                }
                                version = <Label> {
                                    width: Fit,
                                    height: Fit,
                                    text: "v0.0.1",
                                    draw_text: {
                                        color: #000000,
                                        text_style: {
                                            font_size: 16
                                        }
                                    }
                                }
                            }
                            <View> {
                                width: Fill,
                                height:Fit
                                spacing:20,
                                align: {y:0.5}
                                select_btn = <Button> {
                                    width: 100, height: 40,
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
                                    padding: {left: 10,top:8}
                                }
                                <Label> {
                                    width: Fit,
                                    height: Fit,
                                    text: "总数量:",
                                    draw_text: {
                                        color: #000000,
                                        text_style: <THEME_FONT_REGULAR> {
                                            font_size: 16
                                        }
                                    }
                                }
                                qty = <Label> {
                                    width: Fit,
                                    height: Fit,
                                    text: "0",
                                    draw_text: {
                                        color: #000000,
                                        text_style: <THEME_FONT_REGULAR> {
                                            font_size: 16
                                        }
                                    }
                                }
                                action_btn = <Button> {
                                    width: 100, height: 40,
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
                            <InfosTable> {}
                        }
                        <PopupList> {}
                    }
                }

}

#[derive(Live, LiveHook, Widget)]
pub struct DataImportDb {
    #[deref]
    pub view: View,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust]
    pub datas: DbData,
}

#[derive(Debug, Default, Clone)]
pub struct DbData {
    pub data: Vec<ImportDBDatas>,
    pub pn: String,
    pub file_path: PathBuf,
}
impl Widget for DataImportDb {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataImportDb {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let select_btn = self.view.button(id!(select_btn));
        let action_btn = self.view.button(id!(action_btn));
        if self.datas.data.is_empty() {
            action_btn.set_enabled(cx, false);
        }
        let rt = self.rt.handle().clone();
        let ui = self.ui_runner().clone();
        if select_btn.clicked(actions) {
            info!("开始选择文件");
            let _guard = rt.enter();

            rt.spawn(async move {
                let result = select_file().await;
                info!("选择文件结果: {:?}", result);
                ui.defer(move |app, cx, _| match result {
                    Ok(p) => {
                        app.datas.file_path = p.clone();
                        info!("开始提取数据 from file: {}", p.display());
                        if let Some(file_name) = p.file_name().and_then(|s| s.to_str()) {
                            let pn = file_name[..8].to_string();
                            app.datas.pn = pn.clone();
                            app.view.text_input(id!(pn)).set_text(cx, &app.datas.pn);
                            let rt = app.rt.handle().clone();
                            rt.spawn(async move {
                                let data_result = extract_data(p.to_str().unwrap()).await;
                                ui.defer(move |app, cx, scope| match data_result {
                                    Ok(data_vec) => {
                                        if let Some(store) = scope.data.get_mut::<Store>() {
                                            let qty = data_vec.len();
                                            let pn = pn;
                                            app.datas.data = data_vec.clone();
                                            let file_path = p;
                                            app.view.label(id!(qty)).set_text(cx, &qty.to_string());
                                            enqueue_popup_notification(PopupItem {
                                                kind: PopupKind::Success,
                                                auto_dismissal_duration: Some(2.5),
                                                message: "数据提取完成".to_string(),
                                            });
                                            let datas = DbData{
                                                data: data_vec,
                                                pn,
                                                file_path: file_path,
                                            };
                                            store.import_datas = datas;
                                        }
                                    }
                                    Err(e) => {
                                        Cx::post_action(e);
                                    }
                                });
                            });
                        } else {
                            let content = "文件名无效或不存在".to_string();
                            Cx::post_action(MyError::Zdyknown(content));
                        }
                    }
                    Err(e) => {
                        Cx::post_action(e);
                    }
                });
            });
        }
        if action_btn.clicked(actions) {
            info!("开始写入数据");
            let data = self.datas.clone();
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            rt.spawn(async move {
                let res = write_data_to_db(data).await;
                match res {
                    Ok(_) => {
                        enqueue_popup_notification(PopupItem {
                            kind: PopupKind::Success,
                            auto_dismissal_duration: Some(3.0),
                            message: "数据写入成功".to_string(),
                        });
                        info!("数据写入成功");
                    }
                    Err(e) => {
                        Cx::post_action(e);
                    }
                };
            });
        }
        cx.redraw_all();
    }
}
