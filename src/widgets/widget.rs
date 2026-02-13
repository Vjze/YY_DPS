use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*


    mod.widgets.Line = mod.widgets.View {
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


    mod.widgets.MyDropdown = mod.widgets.DropDownFlat {
        width: Fit
        height: Fit

        padding: Inset{ left: 12, right: 24, top: 8, bottom: 8 }
        popup_menu_position: BelowInput

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

            menu_item : mod.widgets.PPopupMenuItem {
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



}
