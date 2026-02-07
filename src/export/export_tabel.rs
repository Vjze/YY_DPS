use crate::export::export_view::ExportAction;
use crate::store::Store;
use makepad_widgets::*;
use std::collections::HashMap;
use std::ops::Range;
use rangemap::RangeMap;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::export::export_row::*;
    use crate::widgets::fold_button_with_text::*;
    ExRowHeaderLabel = <View> {
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
    ExHeaderRow = <View> {
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

        <ExRowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "箱号"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "盒号"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "Sn"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Ith"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Po"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Se"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 0.8
                            }, label = {text: "Sen"} }
        <ExRowHeaderLabel> {width: Fill {
                                weight: 2.0
                            }, label = {text: "测试时间"} }
    }
    pub ExTable = {{ExTable}} <RoundedShadowView> {
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
            ExHeaderRow = <ExHeaderRow> {
                cursor: Default
            }
            list = <PortalList> {
                drag_scrolling: false

                ExItemRow = <ExDataRow> {
                    cursor: Default
                }

                Empty = <View> {
                    height: 0,
                    show_bg: false
                }
                BottomSpace = <View> {height: 100, show_bg: true, draw_bg: {color: #f}}

                FoldHeader = <FoldHeader> {
                    header: <View> {
                        width: Fill, height: Fit
                        align: {x: 0.0, y: 0.5}
                        padding: {left: 15}
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                return #E8EEF3;
                            }
                        }
                        <View> {
                            width: Fill, height: Fit
                            flow: Down
                            align: {x: 0.0, y: 0.5}
                            spacing: 10

                            // fold_button = <FoldButtonWithTextBase> {
                            //     open_text: "展开"
                            //     close_text: "收起"
                            // }
                            <View> {
                                width: Fill, height: Fit,
                                flow: Right,
                                align: {x: 0.5, y: 0.5},
                                padding: {top: 4},
                                fold_button = <FoldButtonWithText> {
                                    open_text: "展开"
                                    close_text: "收起"
                                }
                            }
                            summary_text = <Label> {
                                width: Fit
                                draw_text: {
                                    text_style: <THEME_FONT_BOLD>{
                                        font_size: 14,
                                    }
                                    color: #1C1C1C,
                                }
                                text: ""
                            }
                        }
                    }

                    body: <View> {
                        width: Fill, height: Fit
                        flow: Down
                        // Direct PortalList for rendering grouped items
                        <PortalList> {
                            height: Fit, width: Fill
                            ExItemRow = <ExDataRow> {}
                        }
                    }
                }
            }


    }
}

#[derive(Debug, Clone, Default)]
struct GroupMeta {
    key: String,
    count: usize,
}

#[derive(Default)]
struct GroupHeaderManager {
    group_ranges: RangeMap<usize, String>,
    groups_by_id: HashMap<String, GroupMeta>,
}

impl GroupHeaderManager {

    fn check_group_header_status(&self, item_id: usize) -> Option<Range<usize>> {
        for (range, _) in self.group_ranges.iter() {
            if range.contains(&item_id) {
                return Some(range.clone());
            }
        }
        None
    }

    fn get_group_at_item_id(&self, item_id: usize) -> Option<&GroupMeta> {
        self.group_ranges
            .iter()
            .find(|(range, _)| range.start == item_id)
            .and_then(|(_, header_id)| self.groups_by_id.get(header_id))
    }

    fn compute_groups(&mut self, data: &[HashMap<String, String>]) {
        self.group_ranges.clear();
        self.groups_by_id.clear();
        let mut i = 0;
        let empty_string = String::new();

        while i < data.len() {
            // Group by the first column (carton_no)
            let current_key = data[i].get("carton_no").unwrap_or(&empty_string).clone();
            if current_key.is_empty() {
                i += 1;
                continue;
            }

            let mut count = 1;

            // Count consecutive items with same key
            while i + count < data.len() {
                let next_key = data[i + count].get("carton_no").unwrap_or(&empty_string);
                if next_key == &current_key {
                    count += 1;
                } else {
                    break;
                }
            }

            // Only create groups for 3+ consecutive items
            if count >= 3 {
                let header_id = format!("{}_group_{}", current_key, i);
                let start_index = i;
                let end_index = i + count - 1;

                self.group_ranges.insert(start_index..end_index + 1, header_id.clone());
                self.groups_by_id.insert(
                    header_id,
                    GroupMeta {
                        key: current_key.clone(),
                        count,
                    },
                );
            }

            i += count;
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ExTable {
    #[deref]
    view: View,
    #[rust]
    group_manager: GroupHeaderManager,
}

impl Widget for ExTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Listen for ExportAction to know when data has changed
        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(export_action) = action.downcast_ref::<ExportAction>() {
                    // Data has changed, compute groups
                    self.group_manager.compute_groups(&export_action.data);
                    self.redraw(cx);
                }
            }
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            let entries_count = if let Some(store) = scope.data.get::<Store>() {
                store.datas_store.export_datas.len()
            } else {
                0
            };
            
            let last_item_id = if entries_count > 0 { entries_count } else { 0 };
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, last_item_id);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < last_item_id {
                        // Check if this item is part of a group
                        if let Some(range) = self.group_manager.check_group_header_status(item_id) {
                            if range.start == item_id {
                                // This is the start of a group - render FoldHeader
                                self.render_fold_header(cx, scope, &mut list, item_id, &range);
                            } else if range.contains(&item_id) {
                                // This item is within a group - render empty placeholder
                                list.item(cx, item_id, live_id!(Empty)).draw_all(cx, &mut Scope::empty());
                            }
                        } else {
                            // Normal ungrouped item
                            let template = live_id!(ExItemRow);
                            let item = list.item(cx, item_id, template);
                            if let Some(store) = scope.data.get::<Store>() {
                                let mut file_data = store.datas_store.export_datas[item_id].clone();
                                let mut scope = Scope::with_data(&mut file_data);
                                item.draw_all(cx, &mut scope);
                            }
                        }
                    } else {
                        let item = list.item(cx, item_id, live_id!(BottomSpace));
                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
            
        }
        DrawStep::done()
    }
}

impl ExTable {
    fn render_fold_header(
        &self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        list: &mut std::cell::RefMut<'_, PortalList>,
        item_id: usize,
        range: &Range<usize>,
    ) {
        use std::ops::DerefMut;

        let group_meta = self.group_manager.get_group_at_item_id(item_id).unwrap();

        // Get FoldHeader item from portal list
        let fold_item = list.item(cx, item_id, live_id!(FoldHeader));

        // Set header summary text
        fold_item
            .label(ids!(summary_text))
            .set_text(cx, &format!("箱号: {} ({} 条记录)", group_meta.key, group_meta.count));

        // Draw the FoldHeader and access the inner PortalList
        let mut walk = Walk::default();
        walk.height = Size::fit();
        while let Some(item) = fold_item.draw_walk(cx, scope, walk).step() {
            if let Some(mut list_ref) = item.as_portal_list().borrow_mut() {
                let inner_list = list_ref.deref_mut();

                // Directly render items in the range
                for tl_idx in range.start..range.end {
                    if let Some(store) = scope.data.get::<Store>() {
                        let mut file_data = store.datas_store.export_datas[tl_idx].clone();
                        let mut item_scope = Scope::with_data(&mut file_data);
                        let widget_item = inner_list.item(cx, tl_idx, live_id!(ExItemRow));
                        widget_item.draw_all(cx, &mut item_scope);
                    }
                }
            }
        }
    }
}

impl WidgetMatchEvent for ExTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _e: &Actions, _scope: &mut Scope) {}
}