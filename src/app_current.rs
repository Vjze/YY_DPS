use crate::{
    store::Store,
    utils::error::{LoginResult, MyError, MyTip},
    widgets::{
        popup_list::{PopupItem, PopupKind, enqueue_popup_notification},
    },
};
use makepad_widgets::*;
use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
pub struct AppStartedAction {
    pub store: Store,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::querys::querys_view::QueryScreen;
    use crate::export::export_view::ExportScreen;
    use crate::settings::providers_screen::ProvidersScreen;
    use crate::widgets::dialog::*;
    use crate::login_view::LoginScreen;
    use crate::box_band::box_band_view::BoxBandView;
    use crate::data_import::data_import_db::DataImportDb;
    use crate::widgets::popup_list::*;

    App = {{App}} {
        ui: <View> {
            width: Fill,
            height: Fill,
            flow: Overlay,
            padding: 0

            root = <View> {
                width: Fill,
                height: Fill,
                show_bg: true,
                draw_bg: {
                    color: (MAIN_BG_COLOR_DARK),
                }

                root_adaptive_view = <View> {
                    visible: false
                    sidebar_menu = <SidebarMenu> {}
                    application_pages = <ApplicationPages> {}
                }

                login_view = <View> {
                    visible: true
                    login_screen = <LoginScreen> {}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust(Runtime::new().unwrap())]
    pub rt: Runtime,
    #[rust]
    pub store: Store,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // 处理启动事件
        if let Event::Startup = event {
            self.handle_startup(cx);
            return;
        }
        
        // 处理UI事件
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let rt = self.rt.handle().clone();
        self.ui.view(ids!(root)).set_visible(cx, false);
        
        // 非阻塞异步初始化Store
        rt.spawn(async move {
            let store = crate::store::Store::init().await;
            Cx::post_action(AppStartedAction { store });
        });
    }
}