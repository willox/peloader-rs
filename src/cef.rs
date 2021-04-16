use std::{sync::{Arc, Mutex}};

use cef::*;
use crate::win32;
use crate::browser::event_queue;

pub struct Resizer {
    pub browser: CefBrowser,
    pub w: i32,
    pub h: i32,
}

struct State {
    parent: win32::HWND,
    event_sender: event_queue::Sender,
}

impl State {
    fn send_event(&self, event: event_queue::Event) {
        self.event_sender.send(event).unwrap();
        unsafe {
            win32::SendNotifyMessageA(self.parent, 0x0400, win32::WPARAM::default(), win32::LPARAM::default());
        }
    }
}

impl Task for Resizer {
    fn execute(&mut self) {
        let host = self.browser.get_host().unwrap();
        host.notify_move_or_resize_started();

        let window = host.get_window_handle();

        unsafe {
            assert_ne!(win32::SetWindowPos(std::mem::transmute::<_, win32::HWND>(window), win32::HWND::default(), 0, 0, self.w, self.h, win32::SetWindowPos_uFlags::SWP_NOZORDER), false);
        }
    }
}

pub struct MyApp;
impl App for MyApp {
    fn on_before_command_line_processing(
        &mut self,
        process_type: Option<&CefString>,
        command_line: CefCommandLine,
    ) {
        let mut command_line = command_line;
        println!("on_before_command_line_processing {:?}", process_type);
        println!("{}", command_line.get_program());
    }

    fn get_render_process_handler(&mut self) -> Option<CefRenderProcessHandler> {
        Some(CefRenderProcessHandler::new(MyRenderProcessHandler))
    }
}

struct DevClient;

impl Client for DevClient {

}

struct MyClient {
    life_span_handler: CefLifeSpanHandler,
    request_handler: CefRequestHandler,
}

impl Client for MyClient {
    fn get_life_span_handler(&mut self) -> Option<CefLifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }

    fn get_request_handler(&mut self) -> Option<CefRequestHandler> {
        Some(self.request_handler.clone())
    }
}

struct MyRequestHandler {
    state: Arc<Mutex<State>>,
}
impl RequestHandler for MyRequestHandler {
    fn on_before_browse(&mut self, browser: CefBrowser, frame: CefFrame, request: CefRequest, user_gesture: bool, is_redirect: bool) -> bool {
        let mut parts = CefURLParts::default();

        let url = request.get_url().to_string();
        let cef_url = CefString::new(&url);
        if !cef_parse_url(&cef_url, &mut parts) {
            return true;
        }

        let scheme = parts.scheme();

        if scheme == "byond" {
            println!("UrlNavigate: {}", url);
            self.state.lock().unwrap().send_event(event_queue::Event::UrlNavigate(url));
            return true;
        }

        if scheme != "http" && scheme != "https" {
            return true;
        }

        false
    }
}

struct MyLifeSpanHandler {
    state: Arc<Mutex<State>>,
}
impl LifeSpanHandler for MyLifeSpanHandler {
    fn on_after_created(&mut self, browser: CefBrowser) {
        self.state.lock().unwrap().send_event(event_queue::Event::BrowserCreated(browser));
    }
}

struct MyRenderProcessHandler;
impl RenderProcessHandler for MyRenderProcessHandler {
    fn on_context_created(
        &mut self,
        _browser: CefBrowser,
        _frame: CefFrame,
        context: CefV8Context,
    ) -> () {
        let mut context = context;
        let mut globals = context.get_global().unwrap();

        let string = CefV8Value::create_string(Some(&CefString::new("test"))).unwrap();

        globals.set_value_bykey(
            Some(&CefString::new("test")),
            string,
            CefV8Propertyattribute::NONE,
        );
    }
}

static mut INIT: bool = false;

pub fn init() -> bool {
    unsafe {
        if INIT {
            return false;
        }

        INIT = true;
    }
    let main_args = unsafe { CefMainArgs::new(win32::GetModuleHandleA(win32::PSTR::default()) as _) };

    let app = ::cef::CefApp::new(crate::cef::MyApp);

    if cef_execute_process(&main_args, Some(app.clone()), None) >= 0 {
        return true;
    }
    let settings = CefSettings::default()
        .set_no_sandbox(1)
        .set_log_severity(CefLogSeverity::VERBOSE)
        .set_log_file("E:/log.txt")
        .set_multi_threaded_message_loop(1)
        .set_remote_debugging_port(1339)
        .build();
    assert!(cef_initialize(&main_args, &settings, Some(app.clone()), None));

    std::mem::forget(main_args);
    std::mem::forget(settings);
    std::mem::forget(app);

    false
}

pub fn create(parent: win32::HWND, event_sender: event_queue::Sender) {
    let window_info = unsafe {
        CefWindowInfo::default()
            .set_style(win32::WINDOW_STYLE::WS_VISIBLE.0 | win32::WINDOW_STYLE::WS_CHILD.0)// | win32::WINDOW_STYLE::WS_DLGFRAME.0)
            .set_x(-1)
            .set_y(-1)
            .set_width(512)
            .set_height(512)
            .set_window_name("hello, world!")
            .set_parent_window(std::mem::transmute(parent))
            .build()
    };

    let state = Arc::new(Mutex::new(State {
        parent,
        event_sender,
    }));

    let client = CefClient::new(MyClient {
        life_span_handler: MyLifeSpanHandler {
            state: state.clone(),
        }.into(),
        request_handler: MyRequestHandler {
            state,
        }.into(),
    });

    let browser_settings = CefBrowserSettings::default();

    assert!(CefBrowserHost::create_browser(
        &window_info,
        Some(client.clone()),
        Some(&cef::CefString::new("https://html5test.com/")),
        &browser_settings,
        None,
        None,
    ));
}

pub fn shutdown() {
    cef_shutdown();
}
