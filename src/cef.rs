use std::sync::{Arc, Mutex};

use crate::browser::event_queue;
use crate::win32;
use cef::*;

pub struct PosInvalidated {
    pub browser: CefBrowser,
    pub w: i32,
    pub h: i32,
}

impl Task for PosInvalidated {
    fn execute(&mut self) {
        let host = self.browser.get_host().unwrap();
        host.notify_move_or_resize_started();

        let window = host.get_window_handle();

        unsafe {
            assert_ne!(
                win32::SetWindowPos(
                    std::mem::transmute::<_, win32::HWND>(window),
                    win32::HWND::default(),
                    0,
                    0,
                    self.w,
                    self.h,
                    win32::SetWindowPos_uFlags::from(
                        win32::SetWindowPos_uFlags::SWP_NOZORDER.0
                            | win32::SetWindowPos_uFlags::SWP_NOACTIVATE.0
                    )
                ),
                false
            );
        }
    }
}

struct State {
    parent: win32::HWND,
    event_sender: event_queue::Sender,
}

impl State {
    fn send_event(&self, event: event_queue::Event) {
        // It's ok if the receiver has been destroyed - it just means we are about to be destroyed too!
        let _ = self.event_sender.send(event);

        unsafe {
            win32::PostMessageA(
                self.parent,
                0x0400,
                win32::WPARAM::default(),
                win32::LPARAM::default(),
            );
        }
    }
}

pub struct MyApp {
    browser_process_handler: Option<CefBrowserProcessHandler>,
}
impl App for MyApp {
    fn get_render_process_handler(&mut self) -> Option<CefRenderProcessHandler> {
        Some(CefRenderProcessHandler::new(MyRenderProcessHandler))
    }

    fn get_browser_process_handler(&mut self) -> Option<CefBrowserProcessHandler> {
        self.browser_process_handler.clone()
    }
}

struct MyFocusHandler;
impl FocusHandler for MyFocusHandler {
    fn on_set_focus(&mut self, _browser: CefBrowser, source: CefFocusSource) -> bool {
        // Don't let navigation force us to focus! Why would we want that in our beautiful space game.
        source == CefFocusSource::NAVIGATION
    }
}

struct MyClient {
    life_span_handler: CefLifeSpanHandler,
    request_handler: CefRequestHandler,
    focus_handler: CefFocusHandler,
    context_menu_handler: CefContextMenuHandler,
    state: Arc<Mutex<State>>,
}

impl Client for MyClient {
    fn get_life_span_handler(&mut self) -> Option<CefLifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }

    fn get_request_handler(&mut self) -> Option<CefRequestHandler> {
        Some(self.request_handler.clone())
    }

    fn get_focus_handler(&mut self) -> Option<CefFocusHandler> {
        Some(self.focus_handler.clone())
    }

    fn get_context_menu_handler(&mut self) -> Option<CefContextMenuHandler> {
        Some(self.context_menu_handler.clone())
    }

    fn on_process_message_received(
        &mut self,
        _browser: CefBrowser,
        _frame: CefFrame,
        _source_process: CefProcessId,
        message: CefProcessMessage,
    ) -> bool {
        if message.get_name().to_string() == "cef_to_byond" {
            let url = message
                .get_argument_list()
                .unwrap()
                .get_string(0)
                .to_string();
            self.state
                .lock()
                .unwrap()
                .send_event(event_queue::Event::UrlNavigate(url));
            return true;
        }

        false
    }
}

struct MyBrowserProcessHandler {
    window: win32::HWND,
}
impl BrowserProcessHandler for MyBrowserProcessHandler {
    fn on_schedule_message_pump_work(&mut self, delay_ms: i64) {
        // This can be ran from any thread. We're not going to pull the `&MessageLoop` in here because having its RefCells accessible by this thread is a no-no
        unsafe {
            win32::PostMessageA(
                self.window,
                crate::message_loop::WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK,
                win32::WPARAM(delay_ms as usize),
                win32::LPARAM(0),
            );
        }
    }
}

struct MyRequestHandler {
    state: Arc<Mutex<State>>,
}
impl RequestHandler for MyRequestHandler {
    fn on_before_browse(
        &mut self,
        _browser: CefBrowser,
        _frame: CefFrame,
        request: CefRequest,
        _user_gesture: bool,
        _is_redirect: bool,
    ) -> bool {
        let mut parts = CefURLParts::default();

        let url = request.get_url().to_string();
        let cef_url = CefString::new(&url);
        if !cef_parse_url(&cef_url, &mut parts) {
            return true;
        }

        let scheme = parts.scheme().to_ascii_lowercase();

        if scheme == "byond" {
            println!("UrlNavigate: {}", url);
            self.state
                .lock()
                .unwrap()
                .send_event(event_queue::Event::UrlNavigate(url));
            return true;
        }

        // BYOND relies on loading local files when using browse(null, ...)
        // if scheme != "http" && scheme != "https" {
        //     return true;
        // }

        false
    }
}

struct MyLifeSpanHandler {
    state: Arc<Mutex<State>>,
}
impl LifeSpanHandler for MyLifeSpanHandler {
    fn on_after_created(&mut self, browser: CefBrowser) {
        self.state
            .lock()
            .unwrap()
            .send_event(event_queue::Event::BrowserCreated(browser));
    }
}

struct MyContextMenuHandler;
impl ContextMenuHandler for MyContextMenuHandler {
    fn on_before_context_menu(&mut self, _browser: CefBrowser, _frame: CefFrame, _params: CefContextMenuParams, model: CefMenuModel) {
        assert_eq!(model.add_separator(), true);
        assert_eq!(model.add_item(1337, &CefString::new("Inspect Element")), true);
    }

    fn on_context_menu_command(&mut self, browser: CefBrowser, _frame: CefFrame, params: CefContextMenuParams, command_id: i32, _event_flags: CefEventFlags) -> bool {
        if command_id == 1337 {
            let host = browser.get_host().unwrap();
            let main_frame = browser.get_main_frame().unwrap();

            let window = CefWindowInfo::default()
                .set_window_name(&format!("{:#02x} - Developer Tools", main_frame.get_identifier()))
                .set_style(win32::WINDOW_STYLE::WS_OVERLAPPEDWINDOW.0 | win32::WINDOW_STYLE::WS_CLIPCHILDREN.0 | win32::WINDOW_STYLE::WS_CLIPSIBLINGS.0 | win32::WINDOW_STYLE::WS_VISIBLE.0)
                .set_x(-1)
                .set_y(-1)
                .set_width(1000)
                .set_height(600)
                .build();

            let point = CefPoint::default()
                .set_x(params.get_xcoord())
                .set_y(params.get_ycoord())
                .build();

            host.show_dev_tools(Some(&window), None, None, Some(&point));

            return true;
        }

        false
    }
}

struct MyV8Handler;
impl V8Handler for MyV8Handler {
    fn execute(
        &mut self,
        name: &CefString,
        _object: CefV8Value,
        arguments: &[CefV8Value],
        retval: &mut Option<CefV8Value>,
        exception: &mut CefString,
    ) -> bool {
        if name.to_string() != "cef_to_byond" {
            *exception = CefString::new("unknown function in MyV8Handler");
            return true;
        }

        if arguments.len() != 1 {
            *exception =
                CefString::new("incorrect number of arguments encountered in cef_to_byond");
            return true;
        }

        let arg = &arguments[0];
        if !arg.is_string() {
            *exception = CefString::new("non-string argument passed encountered in cef_to_byond");
            return true;
        }

        let context = CefV8Context::get_current_context().unwrap();

        match context.get_frame() {
            Some(frame) => {
                let url = CefString::new(&arg.get_string_value().to_string());

                let msg = CefProcessMessage::create(&CefString::new("cef_to_byond")).unwrap();
                msg.get_argument_list().unwrap().set_string(0, Some(&url));
                frame.send_process_message(CefProcessId::BROWSER, msg);
                *retval = Some(CefV8Value::create_null().unwrap());
                true
            }

            None => {
                *exception =
                    CefString::new("cef_to_byond called outside of a frame (in a web worker?)");
                true
            }
        }
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
        let globals = context.get_global().unwrap();

        let value =
            CefV8Value::create_function(&CefString::new("cef_to_byond"), MyV8Handler).unwrap();
        globals.set_value_bykey(
            Some(&CefString::new("cef_to_byond")),
            value,
            CefV8Propertyattribute::READONLY,
        );
    }
}

static mut INIT: bool = false;

// Returns true if we are a sub-process
pub fn init(is_main_process: bool) -> bool {
    unsafe {
        if INIT {
            return false;
        }

        INIT = true;
    }
    let main_args =
        unsafe { CefMainArgs::new(win32::GetModuleHandleA(win32::PSTR::default()) as _) };

    let app = if is_main_process {
        ::cef::CefApp::new(crate::cef::MyApp {
            browser_process_handler: Some(
                MyBrowserProcessHandler {
                    window: crate::message_loop::init(),
                }
                .into(),
            ),
        })
    } else {
        ::cef::CefApp::new(crate::cef::MyApp {
            browser_process_handler: None,
        })
    };

    if cef_execute_process(&main_args, Some(app.clone()), None) >= 0 {
        return true;
    }
    let settings = CefSettings::default()
        .set_no_sandbox(1)
        .set_external_message_pump(1)
        .set_log_severity(CefLogSeverity::ERROR)
        .build();
    assert!(cef_initialize(
        &main_args,
        &settings,
        Some(app.clone()),
        None
    ));

    std::mem::forget(main_args);
    std::mem::forget(settings);
    std::mem::forget(app);

    false
}

pub fn create(
    parent: win32::HWND,
    event_sender: event_queue::Sender,
    url: &str,
) -> (CefBrowser, win32::HWND) {
    let window_info = unsafe {
        CefWindowInfo::default()
            .set_style(win32::WINDOW_STYLE::WS_VISIBLE.0 | win32::WINDOW_STYLE::WS_CHILD.0) // | win32::WINDOW_STYLE::WS_DLGFRAME.0)
            .set_ex_style(0 /*win32::WINDOW_EX_STYLE::WS_EX_NOACTIVATE.0*/)
            .set_x(-1)
            .set_y(-1)
            .set_width(512)
            .set_height(512)
            .set_window_name("hello, world!")
            .set_parent_window(std::mem::transmute(parent))
            .build()
    };

    let state = Arc::new(Mutex::new(State {
        parent: parent,
        event_sender,
    }));

    let client = CefClient::new(MyClient {
        life_span_handler: MyLifeSpanHandler {
            state: state.clone(),
        }
        .into(),
        request_handler: MyRequestHandler {
            state: state.clone(),
        }
        .into(),
        focus_handler: MyFocusHandler.into(),
        context_menu_handler: MyContextMenuHandler.into(),
        state,
    });

    let browser_settings = CefBrowserSettings::default();

    let browser = CefBrowserHost::create_browser_sync(
        &window_info,
        Some(client.clone()),
        Some(&CefString::new(url)),
        &browser_settings,
        None,
        None,
    )
    .unwrap();

    unsafe {
        (
            browser.clone(),
            std::mem::transmute(browser.get_host().unwrap().get_window_handle()),
        )
    }
}
