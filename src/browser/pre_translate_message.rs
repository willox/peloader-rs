/// This is a hook for BYOND's CVHTMLCtrl::PreTranslateMessage to stop it from intercepting
/// messages meant for the Chromium windows. It's may break some niche features such as
/// special key binds that CVHtmlCtrl::PreTranslateMessage tries to implement for the control.
use std::{ffi::c_void, ffi::CStr, os::raw::c_char};

use crate::win32;
use detour::RawDetour;

static mut ORIGINAL: Option<
    extern "fastcall" fn(this: *mut c_void, edx: u32, msg: *const win32::MSG) -> u32,
> = None;

unsafe extern "fastcall" fn hook(this: *mut c_void, edx: u32, msg: *const win32::MSG) -> u32 {
    let window = (*msg).hwnd;

    if window == win32::HWND(0) {
        return (ORIGINAL.unwrap())(this, edx, msg);
    }

    let mut class: [u8; 128] = [0; 128];
    assert!(win32::GetClassNameA(window, win32::PSTR(class.as_mut_ptr()), 128) > 0);

    let terminator_idx = class.iter().position(|&c| c == 0).unwrap();

    let str = std::str::from_utf8(&class[..terminator_idx]).unwrap();

    if str.starts_with("Chrome_") || str.starts_with("Cef") {
        win32::TranslateMessage(msg);
        win32::DispatchMessageA(msg);
        return 1;
    }

    (ORIGINAL.unwrap())(this, edx, msg)
}

pub fn init() {
    unsafe {
        let byondwin = win32::LoadLibraryA("byondwin.dll");
        let original = win32::GetProcAddress(
            byondwin,
            "?PreTranslateMessage@CVHTMLCtrl@@UAEHPAUtagMSG@@@Z",
        )
        .unwrap();

        let detour = RawDetour::new(original as _, hook as _).unwrap();

        detour.enable().unwrap();
        ORIGINAL = std::mem::transmute(detour.trampoline());

        std::mem::forget(detour);
    }
}
