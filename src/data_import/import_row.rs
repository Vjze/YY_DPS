use makepad_widgets::*;

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
                sn = <Col> {width: Fill {weight: 1.0}}
                condition_unit =  <Col> {width: Fill {weight: 0.5}}
                ith =  <Col> {width: Fill {weight: 0.5}}
                se =  <Col> {width: Fill {weight: 0.5}}
                po =  <Col> {width: Fill {weight: 0.5}}
                vf =  <Col> {width: Fill {weight: 0.5}}
                im =  <Col> {width: Fill {weight: 0.5}}
                sen =  <Col> {width: Fill {weight: 0.5}}
                icc = <Col> {width: Fill {weight: 0.5}}
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
            let label = self.label(ids!(sn.label));
            label.set_text(cx, &sns);

            let condition_unit = data.condition_unit.clone();
            let label = self.label(ids!(condition_unit.label));
            label.set_text(cx, &condition_unit);

            let iths = data.ith.clone();
            let label = self.label(ids!(ith.label));
            label.set_text(cx, &iths);

            let ses = data.se.clone();
            let label = self.label(ids!(se.label));
            label.set_text(cx, &ses);

            let pos = data.po.clone();
            let label = self.label(ids!(po.label));
            label.set_text(cx, &pos);

            let vfs = data.vf.clone();
            let label = self.label(ids!(vf.label));
            label.set_text(cx, &vfs);

            let ims = data.im.clone();
            let label = self.label(ids!(im.label));
            label.set_text(cx, &ims);

            let sens = data.sen.clone();
            let label = self.label(ids!(sen.label));
            label.set_text(cx, &sens);

            let iccs = data.icc.clone();
            let label = self.label(ids!(icc.label));
            label.set_text(cx, &iccs);
        };
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ImportRow {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {}
}
