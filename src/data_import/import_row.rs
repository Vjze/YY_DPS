use makepad_widgets:: *;

use crate::data_import::work::extract_data::ImportDBDatas;


live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub Line = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #1C1C1C,
        }
    }
    Col = <View> {
        width: Fit
        align: {x: 0.5, y: 0.5}
        label = <Label> {
            draw_text: {
                text_style: {font_size: 12},
                color: #1C1C1C,
            }
        }
    }
    pub ImportRow = {{ImportRow}} {
        flow: Overlay,
        width: Fill,
        height: Fit,
        <View> {
            height: 45,
            flow: Down
            width: Fill
            align: {x: 0.0, y: 0.5}
            h_wrapper = <View> {
                flow: Right
                width: Fill
                spacing: 30
                sn = <Col> {width: Fill}
                condition_unit =  <Col> {width: 100}
                ith =  <Col> {width: 100}
                se =  <Col> {width: 100}
                po =  <Col> {width: 100}
                vf =  <Col> {width: 100}
                im =  <Col> {width: 100}
                sen =  <Col> {width: 100}
                icc = <Col> {width: 100}
            }
            separator_line = <Line> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImportRow {
    #[deref]
    view: View,
}

impl Widget for ImportRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some(data) = scope.data.get::<ImportDBDatas>() {
            let sns = data.sn.clone();
            let label = self.label(id!(sn.label));
            label.set_text(cx, &sns);

            let condition_unit = data.condition_unit.clone();
            let label = self.label(id!(condition_unit.label));
            label.set_text(cx, &condition_unit);

            let iths = data.ith.clone();
            let label = self.label(id!(ith.label));
            label.set_text(cx, &iths);

            let ses = data.se.clone();
            let label = self.label(id!(se.label));
            label.set_text(cx, &ses);

            let pos = data.po.clone();
            let label = self.label(id!(po.label));
            label.set_text(cx, &pos);

            let vfs = data.vf.clone();
            let label = self.label(id!(vf.label));
            label.set_text(cx, &vfs);

            let ims = data.im.clone();
            let label = self.label(id!(im.label));
            label.set_text(cx, &ims);

            let sens = data.sen.clone();
            let label = self.label(id!(sen.label));
            label.set_text(cx, &sens);

            let iccs = data.icc.clone();
            let label = self.label(id!(icc.label));
            label.set_text(cx, &iccs);
        };
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ImportRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}