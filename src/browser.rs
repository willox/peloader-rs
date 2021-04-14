mod constants;
mod document;
mod script;
mod structs;
mod window;

use std::{convert::TryInto, rc::Rc};
use std::{cell::{RefCell, RefMut}, ffi::c_void, pin::Pin};
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

static CLSID_MINE: CLSID = CLSID(0x13333337, 0x1337, 0x1337, 0x1337, [0x13, 0x33, 0x33, 0x33, 0x33, 0x37]);
static CLSID_MINE2: com::sys::CLSID = com::sys::CLSID{
    data1: 0x13333337,
    data2: 0x1337,
    data3: 0x1337,
    data4: [0x37, 0x13, 0x13, 0x33, 0x33, 0x33, 0x33, 0x37]
};

unsafe extern "stdcall" fn co_get_class_object_hook(clsid: *const CLSID, b: u32, c: u32, riid: *const CLSID, e: *mut *const c_void) -> u32 {
    if crate::cef::init() {
        std::process::abort();
        return 0;
    }

    //::cef::cef_do_message_loop_work();

    if *clsid == CLSID_WEB_BROWSER {
        let x = WebBrowserClassFactory::allocate();
        let res: i32 = std::mem::transmute(x.QueryInterface(std::mem::transmute(riid), std::mem::transmute(e)));
        std::mem::forget(x);
        return 0 as u32;
    }

    CO_GET_CLASS_OBJECT.unwrap()(clsid, b, c, riid, e)
}

unsafe extern "stdcall" fn pure_virtual<const N: usize>() -> u32 {
    let x = N;
    println!("{}", x);
    1337
}

unsafe extern "stdcall" fn get_type_info_count(browser: *mut WebBrowser_Old, count: *mut u32) -> u32 {
    *count = 0;
    0
}

com::interfaces! {


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
        fn get_Parent(&self, ppDisp: u32) -> com::sys::HRESULT;
        fn get_Container(&self, ppDisp: u32) -> com::sys::HRESULT;

        #[get]
        #[id(203)]
        fn get_Document(
            &self,
        ) -> u32;

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
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub silent: bool,
    pub document: Option<ClassAllocation<document::HtmlDocument>>,
    pub client_site: Option<IOleClientSite>,
    pub in_place_site: Option<IOleInPlaceSite>,
    pub window: Option<win32::HWND>,
    pub url: Option<String>,
    pub scripts: Vec<String>,
    pub browser: Option<cef::CefBrowser>,
}

impl WebBrowserRef {
    fn activate(&self) {
        let mut state = self.inner.borrow_mut();

        let in_place_site: IOleInPlaceSite = state.client_site.as_ref().unwrap().query_interface().unwrap();
        let mut parent = win32::HWND::default();

        unsafe {
            in_place_site.GetWindow(&mut parent);
        }

        let window = window::create(parent);

        unsafe {
            win32::ShowWindow(window, win32::SHOW_WINDOW_CMD::SW_NORMAL);
        }

        crate::cef::create(self.clone(), window);

        state.in_place_site = Some(in_place_site);
        state.window = Some(window);
    }

    pub fn browser_created(&mut self, browser: cef::CefBrowser) {
        let mut state = self.inner.borrow_mut();

        assert!(state.browser.is_none());

        let frame = browser.get_main_frame().unwrap();

        if let Some(url) = &state.url {
            println!("Cached Navigate {}", url);
            frame.load_url(&cef::CefString::new(url));
        }

        for code in &state.scripts {
            frame.execute_java_script(&cef::CefString::new(code), None, 0);
        }

        state.browser = Some(browser);
    }
}

#[derive(Clone)]
pub struct WebBrowserRef {
    inner: Rc<RefCell<WebBrowserState>>,
}

impl Default for WebBrowserState {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            visible: false,
            silent: false,
            document: None,
            client_site: None,
            in_place_site: None,
            window: None,
            url: None,
            scripts: vec![],
            browser: None,
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
        : IPersistMemory(IPersist)
        , IQuickActivate
        , IProvideClassInfo2(IProvideClassInfo)
        , IOleObject
        , IOleControl
        , IOleCommandTarget
        , IOleInPlaceObject(IOleWindow)
        , IWebBrowser2(IWebBrowserApp(IWebBrowser($IDispatch))) {
        state_ref: WebBrowserRef,
    }

    impl IPersist for WebBrowser {
        fn GetClassID(&self, clsid: *mut com::sys::GUID) -> com::sys::HRESULT {
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
            let mut state = self.state();
            state.client_site = unsafe {
                Some(std::mem::transmute((*container).client_site))
            };

            unsafe {
                (*control).misc_status = 0;
                (*control).view_status = 0;
                (*control).event_cookie = 0;
                (*control).prop_notify_cookie = 0;
                (*control).pointer_activation_policy = 0;
            }

            com::sys::S_OK
        }
        fn SetContentExtent(&self, size: *const structs::Size) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetContentExtent(&self, size: *const structs::Size) -> com::sys::HRESULT {
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
        fn SetClientSite(&self, site: IOleClientSite) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetClientSite(&self, site: IOleClientSite) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetHostNames(&self, szContainerApp: u32, szContainerObj: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Close(&self, dwSaveOption: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetMoniker(&self, dwWhichMoniker: u32, pmk: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetMoniker(&self, dwAssign: u32, dwWhichMoniker: u32, ppmk: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn InitFromData(&self, pDataObject: u32, fCreation: u32, dwReserved: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetClipboardData(&self, dwReserved: u32, ppDataObject: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn DoVerb(&self, iVerb: structs::VerbId, lpmsg: u32, pActiveSite: u32, lindex: u32, hwndParent: u32, lprcPosRect: u32) -> com::sys::HRESULT {
            println!("Verb({:?})", iVerb);
            match iVerb {
                constants::OLEIVERB_SHOW |
                constants::OLEIVERB_OPEN |
                constants::OLEIVERB_HIDE => {
                    com::sys::S_OK
                }

                constants::OLEIVERB_INPLACEACTIVATE => {
                    self.state_ref.activate();
                    com::sys::S_OK
                }

                _ if iVerb > 0 => constants::OLEOBJ_S_INVALIDVERB,
                _  => constants::E_NOTIMPL,
            }
        }
        fn EnumVerbs(&self, ppEnumOleVerb: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Update(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn IsUpToDate(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetUserClassID(&self, pClsid: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetUserType(&self, dwFormofType: u32, pszUserType: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn SetExtent(&self, aspect: structs::DataViewAspect, size: *const structs::Size) -> com::sys::HRESULT {
            if aspect != structs::DataViewAspect::Content {
                unimplemented!();
            }

            let (w, h) = unsafe {
                (((*size).width as f64 * 0.037795280352161) as u32,
                ((*size).height as f64 * 0.037795280352161) as u32)
            };

            let mut state = self.state();

            state.width = w;
            state.height = h;

            println!("{}, {}", w, h);

            if let Some(window) = state.window {
                unsafe {
                    assert_ne!(win32::SetWindowPos(window, win32::HWND::default(), 0, 0, w as i32, h as i32, win32::SetWindowPos_uFlags::SWP_NOZORDER), false);
                }
            }

            if let Some(browser) = &state.browser {
                let task = ::cef::CefTask::new(crate::cef::Resizer {
                    browser: browser.to_owned(),
                    w: w as i32,
                    h: h as i32,
                });

                ::cef::cef_post_task(::cef::CefThreadId::UI, task);
            }

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
        fn Advise(&self, pAdvSink: u32, pdwConnection: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Unadvise(&self, dwConnection: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn EnumAdvise(&self, ppenumAdvise: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn GetMiscStatus(&self, dwAspect: u32, pdwStatus: *mut u32) -> com::sys::HRESULT {
            unsafe {
                *pdwStatus = 1;
            }
            com::sys::S_OK
        }
        fn SetColorScheme(&self, pLogpal: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
    }

    impl IOleControl for WebBrowser {
        fn GetControlInfo(&self, pCI: u32) -> com::sys::HRESULT {
            constants::E_NOTIMPL
        }
        fn OnMnemonic(&self, pMsg: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn OnAmbientPropertyChange(&self, dispID: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn FreezeEvents(&self, bFreeze: bool) -> com::sys::HRESULT {
            com::sys::S_OK
        }
    }

    impl IOleCommandTarget for WebBrowser {
        fn QueryStatus(&self, pguidCmdGroup: u32, cCmds: u32, prgCmds: u32, pCmdText: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Exec(&self, pguidCmdGroup: u32, nCmdID: u32, nCmdexecopt: u32, pvaIn: u32, pvaOut: u32) -> com::sys::HRESULT {
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
        fn ContextSensitiveHelp(&self, fEnterMode: bool) -> com::sys::HRESULT {
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
        fn SetObjectRects(&self, pos: *const structs::Size, rect: *const structs::Size) -> com::sys::HRESULT {

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
        fn Navigate(&self, URL: com::BString, Flags: u32, TargetFrameName: u32, PostData: u32, Headers: u32) -> com::sys::HRESULT {
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
        fn Refresh2(&self, level: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn Stop(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Application(&self, ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Parent(&self, ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Container(&self, ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Document(&self) -> u32 {
            let state = self.state();
            unsafe {
                std::mem::transmute(state.document.as_ref().unwrap().query_interface::<com::interfaces::IDispatch>())
            }
        }
        fn get_TopLevelContainer(&self, ppDisp: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Type(&self, Type: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Left(&self, pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Left(&self, left: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Top(&self, pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Top(&self, top: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Width(&self, pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Width(&self, width: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Height(&self, pl: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn put_Height(&self, height: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_LocationName(&self, LocationName: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_LocationURL(&self, LocationURL: u32) -> com::sys::HRESULT {
            unimplemented!()
        }
        fn get_Busy(&self, pBool: u32) -> com::sys::HRESULT {
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

const WEB_BROWSER_VTABLE: [*const c_void; 256] = [
    pure_virtual::<0> as _,
    pure_virtual::<1> as _,
    pure_virtual::<2> as _,
    get_type_info_count as _,
    pure_virtual::<4> as _,
    pure_virtual::<5> as _,
    pure_virtual::<6> as _,
    pure_virtual::<7> as _,
    pure_virtual::<8> as _,
    pure_virtual::<9> as _,
    pure_virtual::<10> as _,
    pure_virtual::<11> as _,
    pure_virtual::<12> as _,
    pure_virtual::<13> as _,
    pure_virtual::<14> as _,
    pure_virtual::<15> as _,
    pure_virtual::<16> as _,
    pure_virtual::<17> as _,
    pure_virtual::<18> as _,
    pure_virtual::<19> as _,
    pure_virtual::<20> as _,
    pure_virtual::<21> as _,
    pure_virtual::<22> as _,
    pure_virtual::<23> as _,
    pure_virtual::<24> as _,
    pure_virtual::<25> as _,
    pure_virtual::<26> as _,
    pure_virtual::<27> as _,
    pure_virtual::<28> as _,
    pure_virtual::<29> as _,
    pure_virtual::<30> as _,
    pure_virtual::<31> as _,
    pure_virtual::<32> as _,
    pure_virtual::<33> as _,
    pure_virtual::<34> as _,
    pure_virtual::<35> as _,
    pure_virtual::<36> as _,
    pure_virtual::<37> as _,
    pure_virtual::<38> as _,
    pure_virtual::<39> as _,
    pure_virtual::<40> as _,
    pure_virtual::<41> as _,
    pure_virtual::<42> as _,
    pure_virtual::<43> as _,
    pure_virtual::<44> as _,
    pure_virtual::<45> as _,
    pure_virtual::<46> as _,
    pure_virtual::<47> as _,
    pure_virtual::<48> as _,
    pure_virtual::<49> as _,
    pure_virtual::<50> as _,
    pure_virtual::<51> as _,
    pure_virtual::<52> as _,
    pure_virtual::<53> as _,
    pure_virtual::<54> as _,
    pure_virtual::<55> as _,
    pure_virtual::<56> as _,
    pure_virtual::<57> as _,
    pure_virtual::<58> as _,
    pure_virtual::<59> as _,
    pure_virtual::<60> as _,
    pure_virtual::<61> as _,
    pure_virtual::<62> as _,
    pure_virtual::<63> as _,
    pure_virtual::<64> as _,
    pure_virtual::<65> as _,
    pure_virtual::<66> as _,
    pure_virtual::<67> as _,
    pure_virtual::<68> as _,
    pure_virtual::<69> as _,
    pure_virtual::<70> as _,
    pure_virtual::<71> as _,
    pure_virtual::<72> as _,
    pure_virtual::<73> as _,
    pure_virtual::<74> as _,
    pure_virtual::<75> as _,
    pure_virtual::<76> as _,
    pure_virtual::<77> as _,
    pure_virtual::<78> as _,
    pure_virtual::<79> as _,
    pure_virtual::<80> as _,
    pure_virtual::<81> as _,
    pure_virtual::<82> as _,
    pure_virtual::<83> as _,
    pure_virtual::<84> as _,
    pure_virtual::<85> as _,
    pure_virtual::<86> as _,
    pure_virtual::<87> as _,
    pure_virtual::<88> as _,
    pure_virtual::<89> as _,
    pure_virtual::<90> as _,
    pure_virtual::<91> as _,
    pure_virtual::<92> as _,
    pure_virtual::<93> as _,
    pure_virtual::<94> as _,
    pure_virtual::<95> as _,
    pure_virtual::<96> as _,
    pure_virtual::<97> as _,
    pure_virtual::<98> as _,
    pure_virtual::<99> as _,
    pure_virtual::<100> as _,
    pure_virtual::<101> as _,
    pure_virtual::<102> as _,
    pure_virtual::<103> as _,
    pure_virtual::<104> as _,
    pure_virtual::<105> as _,
    pure_virtual::<106> as _,
    pure_virtual::<107> as _,
    pure_virtual::<108> as _,
    pure_virtual::<109> as _,
    pure_virtual::<110> as _,
    pure_virtual::<111> as _,
    pure_virtual::<112> as _,
    pure_virtual::<113> as _,
    pure_virtual::<114> as _,
    pure_virtual::<115> as _,
    pure_virtual::<116> as _,
    pure_virtual::<117> as _,
    pure_virtual::<118> as _,
    pure_virtual::<119> as _,
    pure_virtual::<120> as _,
    pure_virtual::<121> as _,
    pure_virtual::<122> as _,
    pure_virtual::<123> as _,
    pure_virtual::<124> as _,
    pure_virtual::<125> as _,
    pure_virtual::<126> as _,
    pure_virtual::<127> as _,
    pure_virtual::<128> as _,
    pure_virtual::<129> as _,
    pure_virtual::<130> as _,
    pure_virtual::<131> as _,
    pure_virtual::<132> as _,
    pure_virtual::<133> as _,
    pure_virtual::<134> as _,
    pure_virtual::<135> as _,
    pure_virtual::<136> as _,
    pure_virtual::<137> as _,
    pure_virtual::<138> as _,
    pure_virtual::<139> as _,
    pure_virtual::<140> as _,
    pure_virtual::<141> as _,
    pure_virtual::<142> as _,
    pure_virtual::<143> as _,
    pure_virtual::<144> as _,
    pure_virtual::<145> as _,
    pure_virtual::<146> as _,
    pure_virtual::<147> as _,
    pure_virtual::<148> as _,
    pure_virtual::<149> as _,
    pure_virtual::<150> as _,
    pure_virtual::<151> as _,
    pure_virtual::<152> as _,
    pure_virtual::<153> as _,
    pure_virtual::<154> as _,
    pure_virtual::<155> as _,
    pure_virtual::<156> as _,
    pure_virtual::<157> as _,
    pure_virtual::<158> as _,
    pure_virtual::<159> as _,
    pure_virtual::<160> as _,
    pure_virtual::<161> as _,
    pure_virtual::<162> as _,
    pure_virtual::<163> as _,
    pure_virtual::<164> as _,
    pure_virtual::<165> as _,
    pure_virtual::<166> as _,
    pure_virtual::<167> as _,
    pure_virtual::<168> as _,
    pure_virtual::<169> as _,
    pure_virtual::<170> as _,
    pure_virtual::<171> as _,
    pure_virtual::<172> as _,
    pure_virtual::<173> as _,
    pure_virtual::<174> as _,
    pure_virtual::<175> as _,
    pure_virtual::<176> as _,
    pure_virtual::<177> as _,
    pure_virtual::<178> as _,
    pure_virtual::<179> as _,
    pure_virtual::<180> as _,
    pure_virtual::<181> as _,
    pure_virtual::<182> as _,
    pure_virtual::<183> as _,
    pure_virtual::<184> as _,
    pure_virtual::<185> as _,
    pure_virtual::<186> as _,
    pure_virtual::<187> as _,
    pure_virtual::<188> as _,
    pure_virtual::<189> as _,
    pure_virtual::<190> as _,
    pure_virtual::<191> as _,
    pure_virtual::<192> as _,
    pure_virtual::<193> as _,
    pure_virtual::<194> as _,
    pure_virtual::<195> as _,
    pure_virtual::<196> as _,
    pure_virtual::<197> as _,
    pure_virtual::<198> as _,
    pure_virtual::<199> as _,
    pure_virtual::<200> as _,
    pure_virtual::<201> as _,
    pure_virtual::<202> as _,
    pure_virtual::<203> as _,
    pure_virtual::<204> as _,
    pure_virtual::<205> as _,
    pure_virtual::<206> as _,
    pure_virtual::<207> as _,
    pure_virtual::<208> as _,
    pure_virtual::<209> as _,
    pure_virtual::<210> as _,
    pure_virtual::<211> as _,
    pure_virtual::<212> as _,
    pure_virtual::<213> as _,
    pure_virtual::<214> as _,
    pure_virtual::<215> as _,
    pure_virtual::<216> as _,
    pure_virtual::<217> as _,
    pure_virtual::<218> as _,
    pure_virtual::<219> as _,
    pure_virtual::<220> as _,
    pure_virtual::<221> as _,
    pure_virtual::<222> as _,
    pure_virtual::<223> as _,
    pure_virtual::<224> as _,
    pure_virtual::<225> as _,
    pure_virtual::<226> as _,
    pure_virtual::<227> as _,
    pure_virtual::<228> as _,
    pure_virtual::<229> as _,
    pure_virtual::<230> as _,
    pure_virtual::<231> as _,
    pure_virtual::<232> as _,
    pure_virtual::<233> as _,
    pure_virtual::<234> as _,
    pure_virtual::<235> as _,
    pure_virtual::<236> as _,
    pure_virtual::<237> as _,
    pure_virtual::<238> as _,
    pure_virtual::<239> as _,
    pure_virtual::<240> as _,
    pure_virtual::<241> as _,
    pure_virtual::<242> as _,
    pure_virtual::<243> as _,
    pure_virtual::<244> as _,
    pure_virtual::<245> as _,
    pure_virtual::<246> as _,
    pure_virtual::<247> as _,
    pure_virtual::<248> as _,
    pure_virtual::<249> as _,
    pure_virtual::<250> as _,
    pure_virtual::<251> as _,
    pure_virtual::<252> as _,
    pure_virtual::<253> as _,
    pure_virtual::<254> as _,
    pure_virtual::<255> as _,
];

#[repr(C)]
struct WebBrowser_Old {
    vtable: *const [*const c_void; 256],
    bad_data: [u8; 0xFF],
}

impl WebBrowser_Old {
    fn new() -> Self {
        Self {
            vtable: &WEB_BROWSER_VTABLE as _,
            bad_data: [0xBA; 0xFF],
        }
    }
}
