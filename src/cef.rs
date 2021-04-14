use std::{cell::RefCell, rc::Rc};

use cef::*;
use crate::win32;

pub struct Resizer {
    pub browser: CefBrowser,
    pub w: i32,
    pub h: i32,
}

impl Task for Resizer {
    fn execute(&mut self) {
        let window = self.browser.get_host().unwrap().get_window_handle();

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
}

impl Client for MyClient {
    fn get_life_span_handler(&mut self) -> Option<CefLifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }
}

struct MyLifeSpanHandler {
    parent: win32::HWND,
    state_ref: crate::browser::WebBrowserRef,
}
impl LifeSpanHandler for MyLifeSpanHandler {
    fn on_after_created(&mut self, browser: CefBrowser) {

        /*
        let window_info = unsafe {
            CefWindowInfo::default()
                .set_style(win32::WINDOW_STYLE::WS_VISIBLE.0)
                .set_x(-1)
                .set_y(-1)
                .set_width(1024)
                .set_height(512)
                .set_window_name("Dev Tools")
                .set_parent_window(std::mem::transmute(self.parent))
                .build()
        };

        browser.get_host().unwrap().show_dev_tools(Some(&window_info), None, None, None);
        */

        self.state_ref.browser_created(browser);
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

pub fn create(state_ref: crate::browser::WebBrowserRef, mut window: win32::HWND) {
    let window_info = unsafe {
        CefWindowInfo::default()
            .set_style(win32::WINDOW_STYLE::WS_VISIBLE.0 | win32::WINDOW_STYLE::WS_CHILD.0)// | win32::WINDOW_STYLE::WS_DLGFRAME.0)
            .set_x(-1)
            .set_y(-1)
            .set_width(512)
            .set_height(512)
            .set_window_name("hello, world!")
            .set_parent_window(std::mem::transmute(window))
            .build()
    };

    let client = CefClient::new(MyClient {
        life_span_handler: MyLifeSpanHandler {
            parent: window,
            state_ref
        }.into(),
    });

    let browser_settings = CefBrowserSettings::default();

    let mut host = CefBrowserHost::create_browser(
        &window_info,
        Some(client.clone()),
        Some(&cef::CefString::new("https://html5test.com/")),
        &browser_settings,
        None,
        None,
    );

    std::mem::forget(host);
}

pub fn shutdown() {
    cef_shutdown();
}
