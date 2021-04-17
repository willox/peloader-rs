mod constants;
mod document;
mod script;
mod structs;
mod window;
pub mod event_queue;

use std::{convert::TryInto, rc::Rc, sync::{Arc, Mutex}};
use std::{cell::{RefCell, RefMut}, ffi::c_void};
use com::production::ClassAllocation;
use detour::RawDetour;

use crate::win32;

#[repr(C)]
#[derive(PartialEq)]
struct CLSID(u32, u16, u16, u16, [u8; 6]);

static CLSID_NULL: CLSID = CLSID(0, 0, 0, 0, [0;6]);

static CLSID_WEB_BROWSER: CLSID = CLSID(0x8856F961, 0x340A, 0x11D0, 0x6BA9, [0x00, 0xC0, 0x4F, 0xD7, 0x05, 0xA2]);

pub fn init() {
    com::runtime::init_runtime().unwrap();
    window::init();

    let co_get_class_object = unsafe {
        let module = win32::LoadLibraryA("ole32.dll");
        win32::GetProcAddress(module, "CoGetClassObject").unwrap()
    };

    unsafe {
        let detour = RawDetour::new(co_get_class_object as _, co_get_class_object_hook as _).unwrap();
        detour.enable().unwrap();
        CO_GET_CLASS_OBJECT = std::mem::transmute(detour.trampoline());
        std::mem::forget(detour);
    }
}

static mut CO_GET_CLASS_OBJECT: Option<extern "stdcall" fn(clsid: *const CLSID, b: u32, c: u32, riid: *const CLSID, e: *mut *const c_void) -> u32> = None;

unsafe extern "stdcall" fn co_get_class_object_hook(clsid: *const CLSID, b: u32, c: u32, riid: *const CLSID, e: *mut *const c_void) -> u32 {
    // TODO: Move to another hook somewhere in byond?
    // We do our late initialization here because doing it before BYOND has loaded makes MFC sad somehow.
    // Could be investigated but it's no big deal.
    if crate::cef::init() {
        unreachable!()
    }

    if *clsid == CLSID_WEB_BROWSER {
        let x = WebBrowserClassFactory::allocate();
        let res = x.QueryInterface(std::mem::transmute(riid), std::mem::transmute(e));
        assert_eq!(res, com::sys::S_OK);
        std::mem::forget(x);
        return 0 as u32;
    }

    CO_GET_CLASS_OBJECT.unwrap()(clsid, b, c, riid, e)
}

com::interfaces! {
    #[uuid("34A715A0-6587-11D0-924A-0020AFC7AC4D")]
    pub unsafe interface DWebBrowserEvents2 : $IDispatch {}

    #[uuid("B196B286-BAB4-101A-B69C-00AA00341D07")]
    pub unsafe interface IConnectionPoint : com::interfaces::IUnknown {
        fn GetConnectionInterface(&self);
        fn GetConnectionPointContainer(&self);
        fn Advise(&self);
        fn Unadvise(&self);
        fn EnumConnections(&self);
    }

    #[uuid("B196B284-BAB4-101A-B69C-00AA00341D07")]
    pub unsafe interface IConnectionPointContainer : com::interfaces::IUnknown {
        fn EnumConnectionPoints(&self);
        fn FindConnectionPoint(&self, riid: *const com::sys::IID, connection_point: u32) -> com::sys::HRESULT;
    }

    #[uuid("0000010c-0000-0000-C000-000000000046")]
    pub unsafe interface IPersist : com::interfaces::IUnknown {
        fn GetClassID(&self, clsid: *mut com::sys::GUID) -> com::sys::HRESULT;
    }

    #[uuid("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
    pub unsafe interface IPersistMemory : IPersist {
        fn IsDirty(&self);
        fn Save(&self);
        fn GetSizeMax(&self);
        fn InitNew(&self);
    }

    #[uuid("CF51ED10-62FE-11CF-BF86-00A0C9034836")]
    pub unsafe interface IQuickActivate : com::interfaces::IUnknown {
        fn QuickActivate(&self, container: *const structs::QAContainer, control: *mut structs::QAControl) -> com::sys::HRESULT;
        fn SetContentExtent(&self, size: *const structs::Size) -> com::sys::HRESULT;
        fn GetContentExtent(&self, size: *const structs::Size) -> com::sys::HRESULT;
    }

    #[uuid("B196B283-BAB4-101A-B69C-00AA00341D07")]
    pub unsafe interface IProvideClassInfo : com::interfaces::IUnknown {
        fn GetClassInfo(&self);
    }

    #[uuid("A6BC3AC0-DBAA-11CE-9DE3-00AA004BB851")]
    pub unsafe interface IProvideClassInfo2 : IProvideClassInfo {
        fn GetGUID(&self, kind: u32, out: *mut com::sys::IID) -> com::sys::HRESULT;
    }

    #[uuid("00000118-0000-0000-C000-000000000046")]
    pub unsafe interface IOleClientSite : com::interfaces::IUnknown {
        fn SaveObject(&self) -> com::sys::HRESULT;
        fn GetMoniker(&self, dwAssign: u32, dwWhichMoniker: u32, ppmk: u32) -> com::sys::HRESULT;
        fn GetContainer(&self, ppContainer: u32) -> com::sys::HRESULT;
        fn ShowObject(&self) -> com::sys::HRESULT;
        fn OnShowWindow(&self, fShow: bool) -> com::sys::HRESULT;
        fn RequestNewObjectLayout(&self) -> com::sys::HRESULT;
    }

    #[uuid("00000112-0000-0000-C000-000000000046")]
    pub unsafe interface IOleObject : com::interfaces::IUnknown {
        fn SetClientSite(&self, site: IOleClientSite) -> com::sys::HRESULT;
        fn GetClientSite(&self, site: IOleClientSite) -> com::sys::HRESULT;
        fn SetHostNames(&self, szContainerApp: u32, szContainerObj: u32) -> com::sys::HRESULT;
        fn Close(&self, dwSaveOption: u32) -> com::sys::HRESULT;
        fn SetMoniker(&self, dwWhichMoniker: u32, pmk: u32) -> com::sys::HRESULT;
        fn GetMoniker(&self, dwAssign: u32, dwWhichMoniker: u32, ppmk: u32) -> com::sys::HRESULT;
        fn InitFromData(&self, pDataObject: u32, fCreation: u32, dwReserved: u32) -> com::sys::HRESULT;
        fn GetClipboardData(&self, dwReserved: u32, ppDataObject: u32) -> com::sys::HRESULT;
        fn DoVerb(&self, iVerb: structs::VerbId, lpmsg: u32, pActiveSite: u32, lindex: u32, hwndParent: u32, lprcPosRect: u32) -> com::sys::HRESULT;
        fn EnumVerbs(&self, ppEnumOleVerb: u32) -> com::sys::HRESULT;
        fn Update(&self) -> com::sys::HRESULT;
        fn IsUpToDate(&self) -> com::sys::HRESULT;
        fn GetUserClassID(&self, pClsid: u32) -> com::sys::HRESULT;
        fn GetUserType(&self, dwFormofType: u32, pszUserType: u32) -> com::sys::HRESULT;
        fn SetExtent(&self, aspect: structs::DataViewAspect, size: *const structs::Size) -> com::sys::HRESULT;
        fn GetExtent(&self, aspect: structs::DataViewAspect, size: *mut structs::Size) -> com::sys::HRESULT;
        fn Advise(&self, pAdvSink: u32, pdwConnection: u32) -> com::sys::HRESULT;
        fn Unadvise(&self, dwConnection: u32) -> com::sys::HRESULT;
        fn EnumAdvise(&self, ppenumAdvise: u32) -> com::sys::HRESULT;
        fn GetMiscStatus(&self, dwAspect: u32, pdwStatus: *mut u32) -> com::sys::HRESULT;
        fn SetColorScheme(&self, pLogpal: u32) -> com::sys::HRESULT;
    }

    #[uuid("B196B288-BAB4-101A-B69C-00AA00341D07")]
    pub unsafe interface IOleControl : com::interfaces::IUnknown {
        fn GetControlInfo(&self, pCI: u32) -> com::sys::HRESULT;
        fn OnMnemonic(&self, pMsg: u32) -> com::sys::HRESULT;
        fn OnAmbientPropertyChange(&self, dispID: u32) -> com::sys::HRESULT;
        fn FreezeEvents(&self, bFreeze: bool) -> com::sys::HRESULT;
    }

    #[uuid("b722bccb-4e68-101b-a2bc-00aa00404770")]
    pub unsafe interface IOleCommandTarget : com::interfaces::IUnknown {
        fn QueryStatus(&self, pguidCmdGroup: u32, cCmds: u32, prgCmds: u32, pCmdText: u32) -> com::sys::HRESULT;
        fn Exec(&self, pguidCmdGroup: u32, nCmdID: u32, nCmdexecopt: u32, pvaIn: u32, pvaOut: u32) -> com::sys::HRESULT;
    }

    #[uuid("00000114-0000-0000-C000-000000000046")]
    pub unsafe interface IOleWindow : com::interfaces::IUnknown {
        fn GetWindow(&self, phwnd: *mut win32::HWND) -> com::sys::HRESULT;
        fn ContextSensitiveHelp(&self, fEnterMode: bool) -> com::sys::HRESULT;
    }

    #[uuid("00000119-0000-0000-C000-000000000046")]
    pub unsafe interface IOleInPlaceSite : IOleWindow {
        fn CanInPlaceActivate(&self);
        fn OnInPlaceActivate(&self);
        fn OnUIActivate(&self);
        fn GetWindowContext(&self);
        fn Scroll(&self);
        fn OnUIDeactivate(&self);
        fn OnInPlaceDeactivate(&self);
        fn DiscardUndoState(&self);
        fn DeactivateAndUndo(&self);
        fn OnPosRectChange(&self);
    }

    #[uuid("00000113-0000-0000-C000-000000000046")]
    pub unsafe interface IOleInPlaceObject : IOleWindow {
        fn InPlaceDeactivate(&self) -> com::sys::HRESULT;
        fn UIDeactive(&self) -> com::sys::HRESULT;
        fn SetObjectRects(&self, pos: *const structs::Size, rect: *const structs::Size) -> com::sys::HRESULT;
        fn ReactiveAndUndo(&self) -> com::sys::HRESULT;
    }

    #[uuid("EAB22AC1-30C1-11CF-A7EB-0000C05BAE0B")]
    pub unsafe interface IWebBrowser : $IDispatch {
        fn GoBack(&self) -> com::sys::HRESULT;
        fn GoFoward(&self) -> com::sys::HRESULT;
        fn GoHome(&self) -> com::sys::HRESULT;
        fn GoSearch(&self) -> com::sys::HRESULT;

        #[id(104)]
        fn Navigate(&self, URL: com::BString, Flags: u32, TargetFrameName: u32, PostData: u32, Headers: u32) -> com::sys::HRESULT;


        fn Refresh(&self) -> com::sys::HRESULT;
        fn Refresh2(&self, level: u32) -> com::sys::HRESULT;
        fn Stop(&self) -> com::sys::HRESULT;
        fn get_Application(&self, ppDisp: u32) -> com::sys::HRESULT;
        fn get_Parent(&self, ppDisp: *mut *mut u32) -> com::sys::HRESULT;
        fn get_Container(&self, ppDisp: u32) -> com::sys::HRESULT;

        #[get]
        #[id(203)]
        fn get_Document(
            &self,
        ) -> *const c_void;

        fn get_TopLevelContainer(&self, ppDisp: u32) -> com::sys::HRESULT;
        fn get_Type(&self, Type: u32) -> com::sys::HRESULT;
        fn get_Left(&self, pl: u32) -> com::sys::HRESULT;
        fn put_Left(&self, left: u32) -> com::sys::HRESULT;
        fn get_Top(&self, pl: u32) -> com::sys::HRESULT;
        fn put_Top(&self, top: u32) -> com::sys::HRESULT;
        fn get_Width(&self, pl: u32) -> com::sys::HRESULT;
        fn put_Width(&self, width: u32) -> com::sys::HRESULT;
        fn get_Height(&self, pl: u32) -> com::sys::HRESULT;
        fn put_Height(&self, height: u32) -> com::sys::HRESULT;
        fn get_LocationName(&self, LocationName: u32) -> com::sys::HRESULT;
        fn get_LocationURL(&self, LocationURL: u32) -> com::sys::HRESULT;
        fn get_Busy(&self, pBool: u32) -> com::sys::HRESULT;
    }

    #[uuid("0002DF05-0000-0000-C000-000000000046")]
    pub unsafe interface IWebBrowserApp : IWebBrowser {
        #[id(300)]
        fn Quit(&self) -> com::sys::HRESULT;

        #[id(301)]
        fn ClientToWindow(&self) -> com::sys::HRESULT;

        #[id(302)]
        fn PutProperty(&self) -> com::sys::HRESULT;

        #[id(303)]
        fn GetProperty(&self) -> com::sys::HRESULT;

        #[get]
        #[id(0)]
        fn get_Name(&self) -> com::sys::HRESULT;

        #[get]
        #[id(-515)]
        fn get_HWND(&self) -> com::sys::HRESULT;

        #[get]
        #[id(400)]
        fn get_FullName(&self) -> com::sys::HRESULT;

        #[get]
        #[id(401)]
        fn get_Path(&self) -> com::sys::HRESULT;

        #[get]
        #[id(402)]
        fn get_Visible(&self, out: *mut structs::VariantBool) -> com::sys::HRESULT;

        #[set]
        #[id(402)]
        fn set_Visible(&self, vis: structs::VariantBool) -> com::sys::HRESULT;

        #[get]
        #[id(403)]
        fn get_StatusBar(&self) -> com::sys::HRESULT;

        #[set]
        #[id(403)]
        fn set_StatusBar(&self) -> com::sys::HRESULT;

        #[get]
        #[id(404)]
        fn get_StatusText(&self) -> com::sys::HRESULT;

        #[get]
        #[id(404)]
        fn set_StatusText(&self) -> com::sys::HRESULT;

        #[get]
        #[id(405)]
        fn get_ToolBar(&self) -> com::sys::HRESULT;

        #[set]
        #[id(405)]
        fn set_ToolBar(&self) -> com::sys::HRESULT;

        #[get]
        #[id(406)]
        fn get_MenuBar(&self) -> com::sys::HRESULT;

        #[set]
        #[id(406)]
        fn set_MenuBar(&self) -> com::sys::HRESULT;

        #[get]
        #[id(407)]
        fn get_FullScreen(&self) -> com::sys::HRESULT;

        #[set]
        #[id(407)]
        fn set_FullScreen(&self) -> com::sys::HRESULT;
    }

    #[uuid("D30C1661-CDAF-11d0-8A3E-00C04FC9E26E")]
    pub unsafe interface IWebBrowser2 : IWebBrowserApp {
        fn Navigate2(&self);
        fn QueryStatusWB(&self);
        fn ExecWB(&self);
        fn ShowBrowserBar(&self);
        fn get_ReadyState(&self);
        fn get_Offline(&self);
        fn set_Offline(&self);

        #[get]
        #[id(551)]
        fn get_Silent(&self, out: *mut structs::VariantBool) -> com::sys::HRESULT;

        #[set]
        #[id(551)]
        fn set_Silent(&self, vis: structs::VariantBool) -> com::sys::HRESULT;

        fn get_RegisterAsBrowser(&self);
        fn set_RegisterAsBrowser(&self);
        fn get_RegisterAsDropTarget(&self);
        fn set_RegisterAsDropTarget(&self);
        fn get_TheaterMode(&self);
        fn set_TheaterMode(&self);
        fn get_AddressBar(&self);
        fn set_AddressBar(&self);
        fn get_Resizable(&self);
        fn set_Resizable(&self);
    }
}

pub struct WebBrowserState {
    pub unknown: Option<com::interfaces::IUnknown>,
    pub width: i32,
    pub height: i32,
    pub visible: bool,
    pub silent: bool,
    pub document: Option<ClassAllocation<document::HtmlDocument>>,
    pub client_site: Option<IOleClientSite>,
    pub client_sink: Option<com::interfaces::IDispatch>,
    pub in_place_site: Option<IOleInPlaceSite>,
    pub window: Option<win32::HWND>,
    pub url: Option<String>,
    pub scripts: Vec<String>,
    pub browser: Option<cef::CefBrowser>,
    pub command_queue: Arc<Mutex<Vec<String>>>,
    pub event_receiver: event_queue::Receiver,
    pub event_sender: Option<event_queue::Sender>,
}

impl WebBrowserRef {
    fn on_size_invalidated(&self) {
        // SetWindowPos can be re-entrant
        let (width, height, window, browser) = {
            let state = self.inner.borrow_mut();
            (
                state.width,
                state.height,
                state.window.clone(),
                state.browser.clone()
            )
        };

        if let Some(window) = window {
            unsafe {
                assert_ne!(win32::SetWindowPos(window, win32::HWND::default(), 0, 0, width, height, win32::SetWindowPos_uFlags::SWP_NOZORDER), false);
            }
        }

        if let Some(browser) = browser {
            let task = ::cef::CefTask::new(crate::cef::Resizer {
                browser: browser.to_owned(),
                w: width,
                h: height,
            });

            ::cef::cef_post_task(::cef::CefThreadId::UI, task);
        }
    }

    fn process_events(&self) {
        // Pull all of our events out first so we can process them without holding a reference
        let events: Vec<_> = {
            let state = self.inner.borrow_mut();
            state.event_receiver.try_iter().collect()
        };

        for event in events {
            match event {
                event_queue::Event::BrowserCreated(browser) => {
                    self.browser_created(browser);
                }

                event_queue::Event::UrlNavigate(url) => {
                    self.url_navigated(url);
                }
            }
        }
    }

    fn url_navigated(&self, url: String) {
        let unk = {
            let state = self.inner.borrow();
            state.unknown.clone().unwrap()
        };

        let sink = {
            let state = self.inner.borrow();
            state.client_sink.clone()
        };

        let var_type: com::TypeDescVarType = unsafe {
            std::mem::transmute(12u16 | 0x4000u16)
        };

        let bool_var_type: com::TypeDescVarType = unsafe {
            std::mem::transmute(11u16 | 0x4000u16)
        };

        let array_var_type: com::TypeDescVarType = unsafe {
            std::mem::transmute(8192u16 | 17u16)
        };

        let str1 = com::BString::from(url.as_str());
        let str2 = com::BString::from(url.as_str());
        let str3 = com::BString::from(url.as_str());

        let str_arg1 = structs::Variant {
            var_type: com::TypeDescVarType::BStr,
            _1: 0,
            _2: 0,
            _3: 0,
            string: unsafe { std::mem::transmute(str1) },
            _4: 0,
        };

        let str_arg2 = structs::Variant {
            var_type: com::TypeDescVarType::BStr,
            _1: 0,
            _2: 0,
            _3: 0,
            string: unsafe { std::mem::transmute(str2) },
            _4: 0,
        };

        let str_arg3 = structs::Variant {
            var_type: com::TypeDescVarType::BStr,
            _1: 0,
            _2: 0,
            _3: 0,
            string: unsafe { std::mem::transmute(str3) },
            _4: 0,
        };

        let bool_arg: u16 = 0;

        let i4_arg = structs::Variant {
            var_type: com::TypeDescVarType::I4,
            _1: 0,
            _2: 0,
            _3: 0,
            string: unsafe { std::mem::transmute(0) },
            _4: 0,
        };

        let array = com::SafeArray::new(com::TypeDescVarType::Ui1);

        let array_arg = structs::Variant {
            var_type: array_var_type,
            _1: 0,
            _2: 0,
            _3: 0,
            string: unsafe { std::mem::transmute(array) },
            _4: 0,
        };

        unsafe { unk.AddRef(); }
        unsafe { unk.AddRef(); }

        let args: [structs::Variant; 7] = [
            structs::Variant {
                var_type: bool_var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: unsafe { std::mem::transmute(&bool_arg as *const _) },
                _4: 0,
            },
            structs::Variant {
                var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: &str_arg1 as *const _,
                _4: 0,
            },
            structs::Variant {
                var_type: var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: &array_arg as *const _,
                _4: 0,
            },
            structs::Variant {
                var_type: var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: &str_arg2 as *const _,
                _4: 0,
            },
            structs::Variant {
                var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: &i4_arg as *const _,
                _4: 0,
            },
            structs::Variant {
                var_type,
                _1: 0,
                _2: 0,
                _3: 0,
                string: &str_arg3 as *const _,
                _4: 0,
            },
            structs::Variant {
                var_type: com::TypeDescVarType::Dispatch,
                _1: 0,
                _2: 0,
                _3: 0,
                string: unsafe { std::mem::transmute(unk.clone()) },
                _4: 0,
            },
        ];

        let mut result = structs::Variant {
            var_type: com::TypeDescVarType::Empty,
            _1: 0,
            _2: 0,
            _3: 0,
            string: std::ptr::null(),
            _4: 0,
        };

        let disp_params = structs::DispParams {
            args: &args as *const _,
            named_args: std::ptr::null(),
            arg_count: 7,
            named_arg_count: 0,

        };

        if let Some(sink) = &sink {
            unsafe {
                let x = sink.Invoke(250, (&CLSID_NULL) as *const _ as *const com::sys::IID, 0, 1, &disp_params as *const _ as *const u32, &mut result as *mut _ as *mut u32, std::ptr::null_mut(), std::ptr::null_mut());
                println!("Invoke ret: {}", x);
            }
        }
    }

    fn activate(&self, unknown: com::interfaces::IUnknown) {
        let mut state = self.inner.borrow_mut();
        state.unknown = Some(unknown);
        let in_place_site: IOleInPlaceSite = state.client_site.as_ref().unwrap().query_interface().unwrap();
        let mut parent = win32::HWND::default();

        unsafe {
            in_place_site.GetWindow(&mut parent);
        }

        let window = window::create(parent, self);

        unsafe {
            win32::ShowWindow(window, win32::SHOW_WINDOW_CMD::SW_SHOWNOACTIVATE);
        }

        let event_sender = state.event_sender.take().unwrap();

        std::mem::drop(state);
        crate::cef::create(window, event_sender);
        let mut state = self.inner.borrow_mut();

        state.in_place_site = Some(in_place_site);
        state.window = Some(window);
    }

    pub fn browser_created(&self, browser: cef::CefBrowser) {
        {
            let mut state = self.inner.borrow_mut();

            assert!(state.browser.is_none());

            let frame = browser.get_main_frame().unwrap();

            if let Some(url) = &state.url {
                println!("Cached Navigate {}", url);
                frame.load_url(&cef::CefString::new(url));
            }

            for code in &state.scripts {
                frame.execute_java_script(&cef::CefString::new(code), Some(&cef::CefString::new("_byond.js")), 0);
            }

            state.browser = Some(browser);
        }

        // TODO: This is not in main thread!
        self.on_size_invalidated();
    }
}

#[derive(Clone)]
pub struct WebBrowserRef {
    pub inner: Rc<RefCell<WebBrowserState>>,
}

impl Default for WebBrowserState {
    fn default() -> Self {
        let (event_sender, event_receiver) = event_queue::channel();

        Self {
            unknown: None,
            width: 0,
            height: 0,
            visible: false,
            silent: false,
            document: None,
            client_site: None,
            client_sink: None,
            in_place_site: None,
            window: None,
            url: None,
            scripts: vec![],
            browser: None,
            command_queue: Arc::new(Mutex::new(vec![])),
            event_sender: Some(event_sender),
            event_receiver,
        }
    }
}

impl Default for WebBrowserRef {
    fn default() -> Self {
        let state = WebBrowserState {
            ..Default::default()
        };

        let rc = Rc::new(RefCell::new(state));
        {
            let mut state = rc.borrow_mut();
            state.document = Some(document::HtmlDocument::allocate(Self {
                inner: Rc::clone(&rc),
            }));
        }

        Self {
            inner: rc,
        }
    }
}

impl WebBrowser {
    fn state(&self) -> RefMut<WebBrowserState> {
        self.state_ref.inner.borrow_mut()
    }
}

com::class! {
    class WebBrowser
        : IConnectionPointContainer
        , IPersistMemory(IPersist)
        , IQuickActivate
        , IProvideClassInfo2(IProvideClassInfo)
        , IOleObject
        , IOleControl
        , IOleCommandTarget
        , IOleInPlaceObject(IOleWindow)
        , IWebBrowser2(IWebBrowserApp(IWebBrowser($IDispatch))) {
        state_ref: WebBrowserRef,
    }

    impl IConnectionPointContainer for WebBrowser {
        fn EnumConnectionPoints(&self) {
            unimplemented!()
        }
        fn FindConnectionPoint(&self, riid: *const com::sys::IID, _connection_point: u32) -> com::sys::HRESULT {
            // IID_INotifyDBEvents
            let expected: com::sys::IID = com::sys::IID {
                data1: 0xDB526CC0,
                data2: 0xD188,
                data3: 0x11CD,
                data4: [
                    0xAD, 0x48, 0x00, 0xAA, 0x00, 0x3C, 0x9C, 0xB6
                ]
            };

            unsafe {
                assert_eq!(*riid, expected)
            }

            unsafe {
                std::mem::transmute(0x80040200u32)
            }
        }
    }

    impl IPersist for WebBrowser {
        fn GetClassID(&self, _clsid: *mut com::sys::GUID) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IPersistMemory for WebBrowser {
        fn IsDirty(&self) {
            unimplemented!()
        }
        fn Save(&self) {
            unimplemented!()
        }
        fn GetSizeMax(&self) {
            unimplemented!()
        }
        fn InitNew(&self) {
            unimplemented!()
        }
    }

    impl IQuickActivate for WebBrowser {
        fn QuickActivate(&self, container: *const structs::QAContainer, control: *mut structs::QAControl) -> com::sys::HRESULT {
            // TODO: This is barely implemented. Most stuff in container is ignored and control is not populated _at all_.
            let mut state = self.state();
            state.client_site = unsafe {
                Some(std::mem::transmute((*container).client_site))
            };

            let sink: com::interfaces::IUnknown = unsafe {
                std::mem::transmute((*container).event_sink)
            };

            // Apparently the sink doesn't expose the interface that it expects us to later call.
            // Weird, but keep this here in case it shows up at some point and just fetch IDispatch instead.
            assert!(sink.query_interface::<DWebBrowserEvents2>().is_none());

            if let Some(sink) = sink.query_interface::<com::interfaces::IDispatch>() {
                state.client_sink = Some(sink);
            }

            unsafe {
                (*control).misc_status = 0;
                (*control).view_status = 0;
                (*control).event_cookie = 0;
                (*control).prop_notify_cookie = 0;
                (*control).pointer_activation_policy = 0;
            }

            com::sys::S_OK
        }
        fn SetContentExtent(&self, _size: *const structs::Size) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetContentExtent(&self, _size: *const structs::Size) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IProvideClassInfo for WebBrowser {
        fn GetClassInfo(&self) {
            unimplemented!()
        }
    }

    impl IProvideClassInfo2 for WebBrowser {
        fn GetGUID(&self, kind: u32, out: *mut com::sys::IID) -> com::sys::HRESULT {
            // TODO: Come on this is lazy
            // GUIDKIND_DEFAULT_SOURCE_DISP_IID
            if kind == 1 {
                unsafe {
                    (*out).data1 = 0xD30C1661;
                    (*out).data2 = 0xCDAF;
                    (*out).data3 = 0x11d0;
                    (*out).data4 = [0x8A, 0x3E, 0x00, 0xC0, 0x4F, 0xC9, 0xE2, 0x6E];
                }
                return com::sys::S_OK;
            }

            unimplemented!()
        }
    }

    impl IOleObject for WebBrowser {
        fn SetClientSite(&self, _site: IOleClientSite) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetClientSite(&self, _site: IOleClientSite) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetHostNames(&self, _szContainerApp: u32, _szContainerObj: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Close(&self, _dwSaveOption: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetMoniker(&self, _dwWhichMoniker: u32, _pmk: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetMoniker(&self, _dwAssign: u32, _dwWhichMoniker: u32, _ppmk: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn InitFromData(&self, _pDataObject: u32, _fCreation: u32, _dwReserved: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetClipboardData(&self, _dwReserved: u32, _ppDataObject: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn DoVerb(&self, iVerb: structs::VerbId, _lpmsg: u32, _pActiveSite: u32, _lindex: u32, _hwndParent: u32, _lprcPosRect: u32) -> com::sys::HRESULT {
            // TODO: Handle all of these
            match iVerb {
                constants::OLEIVERB_SHOW |
                constants::OLEIVERB_OPEN |
                constants::OLEIVERB_HIDE => {
                    println!("Builtin Verb({:?})", iVerb);
                    com::sys::S_OK
                }

                constants::OLEIVERB_INPLACEACTIVATE => {
                    println!("InPlaceActivate");

                    let unk: com::interfaces::IUnknown = unsafe {
                        std::mem::transmute(self)
                    };
                    unsafe {
                        unk.AddRef();
                    }

                    self.state_ref.activate(unk);
                    self.state_ref.on_size_invalidated();
                    com::sys::S_OK
                }

                _ => {
                    println!("Unknown Verb({:?})", iVerb);

                    if iVerb > 0 {
                        constants::OLEOBJ_S_INVALIDVERB
                    } else {
                        constants::E_NOTIMPL
                    }
                }
            }
        }
        fn EnumVerbs(&self, _ppEnumOleVerb: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Update(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn IsUpToDate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetUserClassID(&self, _pClsid: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetUserType(&self, _dwFormofType: u32, _pszUserType: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetExtent(&self, aspect: structs::DataViewAspect, size: *const structs::Size) -> com::sys::HRESULT {
            if aspect != structs::DataViewAspect::Content {
                unimplemented!();
            }

            // TODO: Is this always the correct modifier?
            let (w, h) = unsafe {
                (((*size).width as f64 * 0.037795280352161).ceil() as i32,
                ((*size).height as f64 * 0.037795280352161).ceil() as i32)
            };

            {
                let mut state = self.state();
                state.width = w;
                state.height = h;
            }

            self.state_ref.on_size_invalidated();
            com::sys::S_OK
        }
        fn GetExtent(&self, aspect: structs::DataViewAspect, size: *mut structs::Size) -> com::sys::HRESULT {
            if aspect != structs::DataViewAspect::Content {
                unimplemented!();
            }

            let state = self.state();
            unsafe {
                (*size).width = state.width;
                (*size).height = state.height;
            }

            com::sys::S_OK
        }
        fn Advise(&self, _pAdvSink: u32, _pdwConnection: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Unadvise(&self, _dwConnection: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn EnumAdvise(&self, _ppenumAdvise: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetMiscStatus(&self, _dwAspect: u32, pdwStatus: *mut u32) -> com::sys::HRESULT {
            // TODO: What are we returning here?
            unsafe {
                *pdwStatus = 1;
            }
            com::sys::S_OK
        }
        fn SetColorScheme(&self, _pLogpal: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IOleControl for WebBrowser {
        fn GetControlInfo(&self, _pCI: u32) -> com::sys::HRESULT {
            constants::E_NOTIMPL
        }
        fn OnMnemonic(&self, _pMsg: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn OnAmbientPropertyChange(&self, _dispID: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn FreezeEvents(&self, _bFreeze: bool) -> com::sys::HRESULT {
            com::sys::S_OK
        }
    }

    impl IOleCommandTarget for WebBrowser {
        fn QueryStatus(&self, _pguidCmdGroup: u32, _cCmds: u32, _prgCmds: u32, _pCmdText: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Exec(&self, _pguidCmdGroup: u32, _nCmdID: u32, _nCmdexecopt: u32, _pvaIn: u32, _pvaOut: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IOleWindow for WebBrowser {
        fn GetWindow(&self, phwnd: *mut win32::HWND) -> com::sys::HRESULT {
            let state = self.state();
            unsafe {
                *phwnd = state.window.unwrap();
            }
            com::sys::S_OK
        }
        fn ContextSensitiveHelp(&self, _fEnterMode: bool) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IOleInPlaceObject for WebBrowser {
        fn InPlaceDeactivate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn UIDeactive(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetObjectRects(&self, _pos: *const structs::Size, _rect: *const structs::Size) -> com::sys::HRESULT {
            // Always (0, 0, 0, 0). Just ignore them, I guess...
            com::sys::S_OK
        }
        fn ReactiveAndUndo(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IWebBrowser for WebBrowser {
        fn GoBack(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GoFoward(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GoHome(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GoSearch(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Navigate(&self, URL: com::BString, _Flags: u32, _TargetFrameName: u32, _PostData: u32, _Headers: u32) -> com::sys::HRESULT {
            // TODO: The other parameters
            let url: String = (&URL).try_into().unwrap();

            let mut state = self.state();

            if let Some(browser) = &state.browser {
                println!("Navigate {}", url);
                browser.get_main_frame().unwrap().load_url(&cef::CefString::new(&url));
                return com::sys::S_OK;
            }

            state.url = Some(url);
            com::sys::S_OK
        }
        fn Refresh(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Refresh2(&self, _level: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Stop(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Application(&self, _ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Parent(&self, ppDisp: *mut *mut u32) -> com::sys::HRESULT {
            let state = self.state();

            let dispatch: com::interfaces::IDispatch = state.client_site.as_ref().unwrap().query_interface().unwrap();
            unsafe {
                *ppDisp = std::mem::transmute(dispatch);
            }

            com::sys::S_OK
        }
        fn get_Container(&self, _ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Document(&self) -> *const c_void {
            let state = self.state();

            let dispatch: com::interfaces::IDispatch = state.document.as_ref().unwrap().query_interface().unwrap();
            unsafe {
                std::mem::transmute(dispatch)
            }
        }
        fn get_TopLevelContainer(&self, _ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Type(&self, _Type: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Left(&self, _pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Left(&self, _left: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Top(&self, _pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Top(&self, _top: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Width(&self, _pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Width(&self, _width: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Height(&self, _pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Height(&self, _height: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_LocationName(&self, _LocationName: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_LocationURL(&self, _LocationURL: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Busy(&self, _pBool: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IWebBrowserApp for WebBrowser {
        fn Quit(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn ClientToWindow(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn PutProperty(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetProperty(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Name(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_HWND(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_FullName(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Path(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Visible(&self, out: *mut structs::VariantBool) -> com::sys::HRESULT {
            let state = self.state();
            unsafe {
                *out = state.visible.into();
            }
            com::sys::S_OK
        }
        fn set_Visible(&self, vis: structs::VariantBool) -> com::sys::HRESULT {
            let mut state = self.state();
            state.visible = vis.into();
            com::sys::S_OK
        }
        fn get_StatusBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn set_StatusBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_StatusText(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn set_StatusText(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_ToolBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn set_ToolBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_MenuBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn set_MenuBar(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_FullScreen(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn set_FullScreen(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IWebBrowser2 for WebBrowser {
        fn Navigate2(&self) {
            unimplemented!()
        }
        fn QueryStatusWB(&self) {
            unimplemented!()
        }
        fn ExecWB(&self) {
            unimplemented!()
        }
        fn ShowBrowserBar(&self) {
            unimplemented!()
        }
        fn get_ReadyState(&self) {
            unimplemented!()
        }
        fn get_Offline(&self) {
            unimplemented!()
        }
        fn set_Offline(&self) {
            unimplemented!()
        }

        fn get_Silent(&self, out: *mut structs::VariantBool) -> com::sys::HRESULT {
            let state = self.state();
            unsafe {
                *out = state.silent.into();
            }
            com::sys::S_OK
        }

        fn set_Silent(&self, silent: structs::VariantBool) -> com::sys::HRESULT {
            let mut state = self.state();
            state.silent = silent.into();
                com::sys::S_OK
        }


        fn get_RegisterAsBrowser(&self) {
            unimplemented!()
        }
        fn set_RegisterAsBrowser(&self) {
            unimplemented!()
        }
        fn get_RegisterAsDropTarget(&self) {
            unimplemented!()
        }
        fn set_RegisterAsDropTarget(&self) {
            unimplemented!()
        }
        fn get_TheaterMode(&self) {
            unimplemented!()
        }
        fn set_TheaterMode(&self) {
            unimplemented!()
        }
        fn get_AddressBar(&self) {
            unimplemented!()
        }
        fn set_AddressBar(&self) {
            unimplemented!()
        }
        fn get_Resizable(&self) {
            unimplemented!()
        }
        fn set_Resizable(&self) {
            unimplemented!()
        }
    }
}
