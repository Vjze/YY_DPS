use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*


    mod.widgets.Line = mod.widgets.View {
        width: Fill,
        height: 1,
        draw_bg +: {
            color: #1C1C1C,
        }
    }
    let MAIN_BG_COLOR = #f9f9f9
    let MAIN_BG_COLOR_DARK = #f2f2f2
    let SIDEBAR_FONT_COLOR = #1A2533
    let SIDEBAR_FONT_COLOR_HOVER = (MAIN_BG_COLOR)
    let SIDEBAR_FONT_COLOR_SELECTED = (MAIN_BG_COLOR)

    let SIDEBAR_BG_COLOR_SELECTED = #344054
    let SIDEBAR_BG_COLOR_HOVER = #677483


    mod.widgets.MyDropdown = mod.widgets.DropDownFlat {
        width: Fit
        height: Fit

        padding: Inset{ left: 12, right: 24, top: 8, bottom: 8 }
        // popup_menu_position: BelowInput

        draw_text +: {
            color: #0a0a0a
            color_hover: #0a0a0a
            color_focus: #0a0a0a
            color_down: #0a0a0a
            color_disabled: #737373

            text_style: theme.font_regular {
                font_size: 14.0
            }
        }

        draw_bg +: {
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

        popup_menu : mod.widgets.PopupMenuFlat {
            draw_bg +: {
                color: #FFFFFFFF
                border_color: #d4d4d4
                border_color_2: #d4d4d4
                border_radius: 6.0
            }

            menu_item : mod.widgets.PopupMenuItem {
                draw_text +: {
                    text_style: theme.font_regular { font_size: 14.0 }
                    color: #0a0a0a
                    color_hover: #0a0a0a
                    color_active: #0a0a0a
                    color_disabled: #737373
                }
                draw_bg +: {
                    color: #FFFFFFFF
                    color_hover: #f5f5f5
                    color_active: #EAEAEA
                    color_disabled: #FAFAFA
                }
            }
        }
    }

    mod.widgets.SidebarMenuButton = mod.widgets.RadioButton {
        width: 150,
        height: 80,
        padding: 8, margin: 0,
        flow: Down, spacing: 8.0, align: Align{x: 0.5, y: 0.5}

        icon_walk: Walk {
            width: 25,
            height: 25
        }
        label_walk: Walk{margin: 0}

        draw_bg +: {
            radio_type: Tab,

            border_size: uniform(0.0)
            border_color: uniform(#0000)
            border_radius: 3.5

            get_color: fn() {
                return mix(
                    mix(
                        #f2f2f2,
                        #677483,
                        self.hover
                    ),
                    #344054,
                    self.active
                )
            }

            get_border_color: fn() {
                return self.border_color
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    0.0 + self.border_size,
                    0.0 + self.border_size,
                    self.rect_size.x - (0.0 + 0.0 + self.border_size * 2.0),
                    self.rect_size.y - (0.0 + 0.0 + self.border_size * 2.0),
                    max(1.0, self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_size)
                }
                return sdf.result;
            }
        }

        draw_text +: {
            color: #1A2533
            color_hover: #f9f9f9
            color_active: #f9f9f9

            // text_style +: {font_size: 15}

            get_color: fn() {
                return mix(
                    mix(
                        self.color,
                        self.color_hover,
                        self.hover
                    ),
                    self.color_active,
                    self.active
                )
            }
        }

        draw_icon +: {
            color: #1A2533
            // color_hover: uniform(#f9f9f9)
            // color_active: uniform(#f9f9f9)
            // focus: instance(0.0)
            // get_color: fn() {
            //     return mix(
            //         mix(
            //             self.color,
            //             self.color_hover,
            //             self.focus
            //         ),
            //         self.color_active,
            //         self.active
            //     )
            // }
        }
    }
    mod.widgets.MyTextInput = mod.widgets.TextInput{
        draw_text +: {
            text_style +:{font_size: 12},
            get_color: fn()  {
                return #555
            }
        }

        // TODO find a way to override colors
        draw_cursor +: {
            focus: 0.0
            border_radius: 0.5
            pixel: fn()  {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(mix(#fff, #bbb, self.focus));
                return sdf.result
            }
        }

        // TODO find a way to override colors
        draw_selection +: {
            hover: 0.0
            focus: 0.0
            border_radius: 2.0
            pixel: fn()  {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(mix(#eee, #ddd, self.focus)); // Pad color
                return sdf.result
            }
        }

        draw_bg +: {
            color: #fff
            border_radius: 2.0
            border_size: 0.0
            border_color: #3

            get_color: fn() {
                return self.color
            }

            get_border_color: fn() {
                return self.border_color
            }

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.box(
                    0.0 + self.border_size,
                    0.0 + self.border_size,
                    self.rect_size.x - (0.0 + 0.0 + self.border_size * 2.0),
                    self.rect_size.y - (0.0 + 0.0 + self.border_size * 2.0),
                    max(1.0, self.border_radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_size > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_size)
                }
                return sdf.result;
            }
        }
    }
}
