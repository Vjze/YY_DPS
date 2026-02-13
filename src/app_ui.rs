use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_CHAT = crate_resource("self://resources/icons/chat.svg")
    let ICON_LOCAL = crate_resource("self://resources/icons/local.svg")
    let ICON_BAND_VIEW = crate_resource("self://resources/icons/cloud.svg")
    let ICON_CLOUD = crate_resource("self://resources/icons/cloud.svg")
    let ICON_MOLYSERVER = crate_resource("self://resources/images/logo.png")

    let SidebarMenuButton = RadioButton {
        width: 150,
        height: 80,
        padding: 8, margin: 0,
        flow: Right, spacing: 8.0, align: Align{x: 0.5, y: 0.5}

        icon_walk: Walk {
            margin: 0,
            width: 25,
            height: 25
        }
        label_walk: Walk{margin: 0}

        draw_bg +: {
            radio_type: Tab,

            border_size: 0.0
            border_color: uniform(#0000)
            inset: vec4(0.0, 0.0, 0.0, 0.0)
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

            // text_style: {font_size: 15}

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

        draw_icon +: {
            color: #1A2533
            color_hover: uniform(#f9f9f9)
            color_active: uniform(#f9f9f9)
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

    let ApplicationPages = RoundedShadowView {
        width: Fill, height: Fill
        margin: Inset{top: 12, right: 12, bottom: 12}
        padding: 3.
        flow: Overlay
        // show_bg: true
        draw_bg +: {
            color: instance(#f9f9f9),
            border_radius: uniform(8.5),
            shadow_color: instance(#0003),
            shadow_radius: uniform(18.0),
            shadow_offset: vec2(0.0,-1.5)
        }
        // export_frame = <ExportScreen> {visible: true}
        // querys_frame = <QueryScreen> {visible: false}
        // box_band_frame = <BoxBandView> {visible: false}
        // data_import_db_frame = <DataImportDb> {visible: false}
        // providers_frame = <ProvidersScreen> {visible: false}
        //
    }
    let SidebarMenu = RoundedView {
        width: 90, height: Fill,
        flow: Down, spacing: 15.0,
        padding: Inset{ top: 40, bottom: 10, left: 10, right: 10 },

        align: Align{x: 0.5, y: 0.0},
        // show_bg: true,
        draw_bg +: {
            color: #f2f2f2,
            border_radius: 0.0,
        }
        logo := View {
            width: Fit, height: Fit
            padding: Inset{bottom:20}
            Image {
                width: 50, height: 50,
                src: ICON_MOLYSERVER,
            }
        }
        seprator := View {
            width: Fill, height: 1.6,
            margin: Inset{left: 15, right: 15, bottom: 10}
            // show_bg: true
            draw_bg +: {
                color: #dadada,
            }
        }

        export_tab := SidebarMenuButton {
            animator: Animator{active : {default: @on}}
            text: "查询导出",
            draw_icon +: {
                svg_file: (ICON_CHAT),
            }
        }
        sn_tab := SidebarMenuButton {
            text: "数据查询",
            draw_icon +: {
                svg_file: (ICON_LOCAL),
            }
        }
        box_band_tab := SidebarMenuButton {
            text: "盒号绑定",
            draw_icon +: {
                svg_file: (ICON_BAND_VIEW),
            }
        }
        data_import_db_tab := SidebarMenuButton {
            text: "外协数据导入",
            draw_icon +: {
                svg_file: (ICON_BAND_VIEW),
            }
        }
        Filler{}
        View {
            align: Align{y: 1.0}
            visible: false
            providers_tab := SidebarMenuButton {
                text: "设置",
                draw_icon +: {
                    svg_file: (ICON_CLOUD),
                }
            }
        }

    }
    mod.widgets.AppUI = Window {
        caption_bar +: {
            margin: Inset{top: 2 left: -190}
            caption_label +: {
                label +: {
                    text: "DPS"
                }
            }
            windows_buttons +: {
                min +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                max +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                close +: { draw_bg +: {color: #0, color_hover: #E81123, color_down: #FF0015} }
            }
        }
        window.inner_size: vec2(1600 900)

        show_bg: true
        draw_bg +: {
            color: #F3F3F3
            // pixel: fn() {
            //     return theme.color_bg_app
            // }
        }

        body +: {
            flow: Overlay
            width: Fill,
            height: Fill,
            padding: 0

            root := View {
                width: Fill,
                height: Fill,
                // show_bg: true,
                draw_bg +: {
                    color: #f2f2f2,
                }

                root_adaptive_view := View {
                    visible: false

                        sidebar_menu := SidebarMenu {}
                        application_pages := ApplicationPages {}
                }
                login_view := View {
                    visible: true
                login_screen := mod.widgets.LoginScreen {}

                }
            }
            // <PopupList> {}
        }
        // dialog_ui := Modal {
        //     content : {
        //         dialog_ui_inner = ErrorDialog {
        //         }
        //     }
        // }
    }
}
