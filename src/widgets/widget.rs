use makepad_widgets::*;

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
    pub MAIN_BG_COLOR = #f9f9f9
    pub MAIN_BG_COLOR_DARK = #f2f2f2
    pub SIDEBAR_FONT_COLOR = #1A2533
    pub SIDEBAR_FONT_COLOR_HOVER = (MAIN_BG_COLOR)
    pub SIDEBAR_FONT_COLOR_SELECTED = (MAIN_BG_COLOR)

    pub SIDEBAR_BG_COLOR_SELECTED = #344054
    pub SIDEBAR_BG_COLOR_HOVER = #677483

    pub SidebarMenuButton = <RadioButton> {
        width: 150,
        height: 80,
        padding: 8, margin: 0,
        flow: Right, spacing: 8.0, align: {x: 0.5, y: 0.5}

        icon_walk: {margin: 0, width: 25, height: 25}
        label_walk: {margin: 0}

        draw_bg: {
            radio_type: Tab,

            instance border_size: 0.0
            instance border_color_1: #0000
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            instance border_radius: 3.5

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (MAIN_BG_COLOR_DARK),
                        (SIDEBAR_BG_COLOR_HOVER),
                        self.hover
                    ),
                    (SIDEBAR_BG_COLOR_SELECTED),
                    self.active
                )
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
            color: (SIDEBAR_FONT_COLOR)
            // color_hover: (SIDEBAR_FONT_COLOR_HOVER)
            // color_active: (SIDEBAR_FONT_COLOR_SELECTED)

            text_style: <THEME_FONT_BOLD>{font_size: 15}

            // fn get_color(self) -> vec4 {
            //     return mix(
            //         mix(
            //             self.color,
            //             self.color_hover,
            //             self.hover
            //         ),
            //         self.color_active,
            //         self.active
            //     )
            // }
        }

        draw_icon: {
            instance color: (SIDEBAR_FONT_COLOR)
            instance color_hover: (SIDEBAR_FONT_COLOR_HOVER)
            instance color_active: (SIDEBAR_FONT_COLOR_SELECTED)
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color,
                        self.color_hover,
                        self.focus
                    ),
                    self.color_active,
                    self.active
                )
            }
        }
    }


}
