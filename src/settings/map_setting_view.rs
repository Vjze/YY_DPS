use std::collections::HashMap;

use makepad_widgets::*;
use tokio::runtime::Runtime;

use crate::{
    configs::column_map_config::{
        delete_template_map, get_template_map_config, update_template_map,
    },
    settings::delete_modal::DeleteModalAction,
    store::Store,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::settings::map_grid::MapGrid;
    use crate::settings::delete_modal::DeleteModal;

    pub MapView = {{MapView}} <RoundedShadowView> {
        width: Fill, height: Fill
        align: {x: 0.0, y: 0.0}
        padding: {left: 15, right: 15, bottom: 15, top:15}
        show_bg: true
        flow: Overlay
        draw_bg: {
            color: (MAIN_BG_COLOR_DARK)
            border_radius: 4.5,
            uniform shadow_color: #0002
            shadow_radius: 8.0,
            shadow_offset: vec2(0.0,-1.5)
        }

        content = <View> {
            flow: Down, spacing: 20

            <View> {
                    width: Fill, height: Fit,
                    spacing:15,

                    <Labelbold> {
                        padding:6,
                        text: "选择模板:"
                        draw_text: {
                            text_style: {
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                    template_selector = <DropDown> {
                        width: 200,height:40
                        labels:["template_1","template_2","template_3"],
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
                    clear_template_btn = <Button> {
                        width: 120, height: 40,
                        text: "数据清空",
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 15,

                            }
                            color: #000
                        }
                        draw_bg: {
                            color: #ADD8E6,
                            uniform border_radius: 5.0,
                            uniform border_size: 1.0,
                            uniform border_color_1: #333
                        }
                    }
                }


                <View> {
                        height: Fill,
                        width: Fill,
                        spacing: 10,
                        padding: {top: 150}
                        <MapGrid> {}
                }
                <View> {
                    spacing: 10,
                    align: {x: 0.5, y: 1.0}
                    width: Fill,
                    height: Fit
                    update_template_btn = <Button> {
                        width: 120, height: 40,
                        text: "模板更新",
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 15,

                            }
                            color: #000
                        }
                        draw_bg: {
                            color: #ADD8E6,
                            uniform border_radius: 5.0,
                            uniform border_size: 1.0,
                            uniform border_color_1: #333
                        }
                    }
                    delete_template_btn = <Button> {
                        width: 120, height: 40,
                        text: "模板删除",
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 15,

                            }
                            color: #000
                        }
                        draw_bg: {
                            color: #ADD8E6,
                            uniform border_radius: 5.0,
                            uniform border_size: 1.0,
                            uniform border_color_1: #333
                        }
                    }
                }
            }

        delete_modal = <Modal> {
            content: {
                <DeleteModal> {}
            }
        }
    }

}

// TODO: Rename into MapView
#[derive(Widget, LiveHook, Live)]
struct MapView {
    #[deref]
    view: View,
    #[rust(Runtime::new().unwrap())]
    rt: Runtime,
}

impl Widget for MapView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(store) = scope.data.get_mut::<Store>() {
            if !store.templates.is_empty() {
                self.view
                    .drop_down(id!(template_selector))
                    .set_labels(cx, store.templates.clone());
            };
        }
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MapView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let select = self.view.drop_down(id!(template_selector));
        let update_btn = self.view.button(id!(update_template_btn));
        let delete_btn = self.view.button(id!(delete_template_btn));
        let clear_btn = self.view.button(id!(clear_template_btn));
        if let Some(value) = select.changed_label(actions) {
            let rt = self.rt.handle().clone();
            let _guard = rt.enter();
            let map_infos = rt.block_on(async move {
                let res = get_template_map_config(value).await;
                match res {
                    Ok(res) => res,
                    Err(e) => {
                        Cx::post_action(e);
                        HashMap::default()
                    }
                }
            });
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.map_infos = map_infos;
                let grid_area = self.view.area(); // 或者通过其他方式获取目标 Area
                store.grid_area = grid_area;
                let trigger = Trigger {
                    id: live_id!(update_map_grid),
                    from: Area::Empty, // 来源可以是 Empty，除非需要特定来源
                };
                cx.send_trigger(grid_area, trigger);
            }
        }

        if update_btn.clicked(actions) {
            let template_name = select.text().clone();
            let rt = self.rt.handle().clone();
            if let Some(store) = scope.data.get_mut::<Store>() {
                let map_infos = store.map_infos.clone();
                let _guard = rt.enter();
                rt.block_on(async move {
                    match update_template_map(template_name, map_infos).await {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
                        }
                        Err(e) => {
                            Cx::post_action(e);
                        }
                    }
                });
            };
        }
        if delete_btn.clicked(actions) {
            self.modal(id!(delete_modal)).open(cx);
        }
        if clear_btn.clicked(actions) {
            if let Some(store) = scope.data.get_mut::<Store>() {
                store.map_infos = HashMap::default();
                let grid_area = self.view.area(); // 或者通过其他方式获取目标 Area
                store.grid_area = grid_area;
                let trigger = Trigger {
                    id: live_id!(map_infos),
                    from: Area::Empty, // 来源可以是 Empty，除非需要特定来源
                };
                cx.send_trigger(grid_area, trigger);
            }
        }
        for action in actions {
            if let Some(DeleteModalAction::Close) = action.downcast_ref() {
                self.modal(id!(delete_modal)).close(cx);
            }
            if let Some(DeleteModalAction::Action) = action.downcast_ref() {
                let template_name = select.text().clone();
                let rt = self.rt.handle().clone();
                let _guard = rt.enter();
                rt.block_on(async move {
                    match delete_template_map(template_name).await {
                        Ok(res) => {
                            Cx::post_action(res);
                            Store::init().await;
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
