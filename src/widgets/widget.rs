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


    pub MyDropdown = <DropDownFlat> {
        width: Fit
        height: Fit

        padding: { left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

        draw_text: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
        }

        draw_bg: {
            border_size: 1.0
            border_radius: 6.0
            color_dither: 0.0

            color: #FFFFFF
            color_hover: #FAFAFA
            color_focus: #FFFFFF
            color_down: #F5F5F5
            color_disabled: #F5F5F5

            border_color: #3
            border_color_hover: #0284c7
            border_color_focus: #0284c7
            border_color_down: #0284c7
            border_color_disabled: #f5f5f5

            border_color_2: #3
            border_color_2_hover: #0284c7
            border_color_2_focus: #0284c7
            border_color_2_down: #0284c7
            border_color_2_disabled: #f5f5f5

            arrow_color: #666666
            arrow_color_hover: #333333
            arrow_color_focus: #4A90D9
            arrow_color_down: #4A90D9
            arrow_color_disabled: #9E9E9E
        }

        popup_menu: <PopupMenuFlat> {
            draw_bg: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: 6.0
            }

            menu_item: <PopupMenuItem> {
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }
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
