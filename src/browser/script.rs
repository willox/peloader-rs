use com::interfaces::IDispatch;
use std::{cell::RefCell, convert::TryInto, rc::Rc};

com::interfaces! {
    #[uuid("00000000-0000-0000-A731-00A0C9082637")]
    pub unsafe interface IScript : IDispatch {
        pub fn eval(&self) -> com::sys::HRESULT;
    }
}

#[repr(C)]
pub struct Variant {
    pub var_type: com::TypeDescVarType,
    pub _1: u16,
    pub _2: u16,
    pub _3: u16,
    pub string: com::BString,
}

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
        pub fn GetIDsOfNames(&self, id: *const com::sys::IID, names: *const *const u16, count: u32, lcid: u32, out: *mut u32) -> com::sys::HRESULT {
            unsafe {
                *out = 1;
            }
            com::sys::S_OK
        }
        pub fn Invoke(
            &self,
            disp_id: u32,
            riid: *const com::sys::IID,
            lcid: u32,
            flags: u16,
            params: *const u32,
            result: *mut u32,
            excep_info: *mut u32,
            arg_err: *mut u32,
        ) -> com::sys::HRESULT {
            let params = params as *const *const Variant;
            let data: String = unsafe {
                (&(**params).string).try_into().unwrap()
            };
            println!("exec({:?}", data);
            com::sys::S_OK
        }
    }

    impl IScript for Script {
        pub fn eval(&self) -> com::sys::HRESULT {
            unimplemented!()
        }
    }
}

