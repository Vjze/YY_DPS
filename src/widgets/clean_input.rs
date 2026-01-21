use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    ICON_CLOSE = dep("crate://self/resources/icons/close.svg")


    pub InputClean = {{InputClean}} {
        width: Fill,
        height: Fit,

        flow: Right,
        align: { y: 0.5 },
        // padding: { left: 12, top: 10, bottom: 10 }
        spacing: 8

        show_bg: true,
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0

            uniform border_radius: 6.0
            uniform border_width: 1.0

            uniform bg_color: #FFFFFF
            uniform bg_color_hover: #BDBDBD
            uniform bg_color_focus: #4A90D9

            uniform border_color: #E0E0E0
            uniform border_color_hover: #BDBDBD
            uniform border_color_focus: #4A90D9

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(
                    mix(self.bg_color, self.bg_color_hover, self.hover),
                    self.bg_color_focus,
                    self.focus
                );

                let border = mix(
                    mix(self.border_color, self.border_color_hover, self.hover),
                    self.border_color_focus,
                    self.focus
                );

                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.border_radius
                );

                sdf.fill_keep(bg);
                let border_w = mix(self.border_width, 2.0, self.focus);
                sdf.stroke(border, border_w);

                return sdf.result;
            }
        }

        // Password text input (borderless)
        input = <TextInput> {
            width: Fill,
            height: Fit,
            empty_text: "...",

            draw_bg: {
                color: #fff
                instance border_radius: 2.0
                instance border_size: 0.0
                instance border_color_1: #3
                instance inset: vec4(0.0, 0.0, 0.0, 0.0)

                fn get_color(self) -> vec4 {
                    return self.color
                }

                fn get_border_color(self) -> vec4 {
                    return self.border_color_1
                }

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    sdf.box(
                        self.inset.x + self.border_size,
                        self.inset.y + self.border_size,
                        self.rect_size.x - (self.inset.x + self.inset.z + self.border_size * 2.0),
                        self.rect_size.y - (self.inset.y + self.inset.w + self.border_size * 2.0),
                        max(1.0, self.border_radius)
                    )
                    sdf.fill_keep(self.get_color())
                    if self.border_size > 0.0 {
                        sdf.stroke(self.get_border_color(), self.border_size)
                    }
                    return sdf.result;
                }
            }

            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix(#333333, #9E9E9E, self.empty);
                }
            }

            draw_cursor: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 1.0);
                    sdf.fill(mix(#0000, #4A90D9, self.focus * (1.0 - self.blink)));
                    return sdf.result;
                }
            }

            draw_selection: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                    sdf.fill(#4A90D920);
                    return sdf.result;
                }
            }
        }

        // Eye icon for toggle visibility (clickable)
        close_icon = <ButtonFlatterIcon> {
            width: Fit,
            height: Fit,
            padding: 4,
            margin: 10,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }
            draw_icon: {
                svg_file: (ICON_CLOSE)
                color: #9E9E9E
                color_hover: #666666
            }
            icon_walk: { width: 18.0, height: 18.0 }
        }
    }

}

// Password input widget with toggle visibility
#[derive(Live, LiveHook, Widget)]
pub struct InputClean {
    #[deref]
    view: View,
}

impl Widget for InputClean {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for InputClean {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle eye icon button click
        if self.view.button(ids!(close_icon)).clicked(actions) {
            self.view.text_input(ids!(input)).set_text(cx, "");

            self.view.redraw(cx);
        }
    }
}

impl InputCleanRef {
    /// Get the current password text
    pub fn text(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.view.text_input(ids!(input)).text()
        } else {
            String::new()
        }
    }

    /// Set the password text
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(inner) = self.borrow() {
            inner.view.text_input(ids!(input)).set_text(cx, text);
        }
    }
}
