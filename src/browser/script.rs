use com::interfaces::IDispatch;
use std::convert::TryInto;

com::interfaces! {
    #[uuid("00000000-0000-0000-A731-00A0C9082637")]
    pub unsafe interface IScript : IDispatch {
        pub fn eval(&self) -> com::sys::HRESULT;
    }
}

#[repr(C)]
pub struct VariantHack {
    pub var_type: com::TypeDescVarType,
    pub _1: u16,
    pub _2: u16,
    pub _3: u16,
    pub string: com::BString,
}

// TODO: This is way too bare-bones
com::class! {
    pub class Script : IScript(IDispatch) {
        state_ref: crate::browser::WebBrowserRef,
    }

    impl IDispatch for Script {
        pub fn GetTypeInfoCount(&self) {
            unimplemented!()
        }
        pub fn GetTypeInfo(&self) {
            unimplemented!()
        }
        pub fn GetIDsOfNames(&self, _id: *const com::sys::IID, _names: *const *const u16, _count: u32, _lcid: u32, out: *mut u32) -> com::sys::HRESULT {
            unsafe {
                *out = 1;
            }
            com::sys::S_OK
        }
        pub fn Invoke(
            &self,
            _disp_id: u32,
            _riid: *const com::sys::IID,
            _lcid: u32,
            _flags: u16,
            params: *const u32,
            _result: *mut u32,
            _excep_info: *mut u32,
            _arg_err: *mut u32,
        ) -> com::sys::HRESULT {
            let mut state = self.state_ref.inner.borrow_mut();

            let params = params as *const *const VariantHack;
            let data: String = unsafe {
                (&(**params).string).try_into().unwrap()
            };

            if let Some(browser) = &state.browser {
                let frame = browser.get_main_frame().unwrap();
                frame.execute_java_script(&cef::CefString::new(&data), Some(&cef::CefString::new("_byond.js")), 0);
                return com::sys::S_OK;
            }

            state.scripts.push(data);
            com::sys::S_OK
        }
    }

    impl IScript for Script {
        pub fn eval(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }
}
