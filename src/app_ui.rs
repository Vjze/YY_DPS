use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let ICON_CHAT = crate_resource("self://resources/icons/chat.svg")
    let ICON_LOCAL = crate_resource("self://resources/icons/local.svg")
    let ICON_BAND_VIEW = crate_resource("self://resources/icons/cloud.svg")
    let ICON_CLOUD = crate_resource("self://resources/icons/cloud.svg")
    let ICON_MOLYSERVER = crate_resource("self://resources/images/logo.png")

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
        export_frame := ExportScreen {visible: true}
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
        View {
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
            width: Fill
            height: Fit
            align: Align {x:0.5}
            // animator: active : {default: @on}
            text: "查询导出",
            draw_icon +: {
                svg: (ICON_CHAT),
            }
        }
        sn_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "数据查询",
            draw_icon +: {
                svg: (ICON_LOCAL),
            }
        }
        box_band_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "盒号绑定",
            draw_icon +: {
                svg: (ICON_BAND_VIEW),
            }
        }
        data_import_db_tab := SidebarMenuButton {
            width: Fill
            height: Fit
            align: Align {x:0.5}
            text: "外协数据导入",
            draw_icon +: {
                svg: (ICON_BAND_VIEW),
            }
        }
        Filler{}
        View {
            align: Align{x: 0.5 y: 1.0}
            // visible: false
            providers_tab := SidebarMenuButton {
                text: "设置",
                draw_icon +: {
                    svg: (ICON_CLOUD),
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
                            draw_text +: {
                                color: #000000
                            }
                        }
                    }
                    windows_buttons +: {
                        min +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                        max +: { draw_bg +: {color: #0, color_hover: #9, color_down: #3} }
                        close +: { draw_bg +: {color: #0, color_hover: #E81123, color_down: #FF0015} }
                    }
                }
                pass.clear_color: vec4(0.95 0.95 0.95 1.0)
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
                            login_screen := LoginScreen {}

                        }
                    }
                    // <PopupList> {}
                    dialog_ui := Modal {
                        can_dismiss: false
                        content +: {
                            dialog_ui_inner := ErrorDialog {
                            }
                        }
                    }
                
                
            
        }
    }
}
