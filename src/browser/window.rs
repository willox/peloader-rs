use crate::win32;

extern "system" fn window_proc(
    hwnd: win32::HWND,
    message: u32,
    w_param: win32::WPARAM,
    l_param: win32::LPARAM
) -> win32::LRESULT {
    // TODO: lazy
    if message == 0x0400 {
        let state_ref: &crate::browser::WebBrowserRef = unsafe {
            let ptr = win32::GetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX::default()) as *const crate::browser::WebBrowserRef;
            std::mem::transmute(ptr)
        };

        state_ref.process_events();
    }

    unsafe {
        win32::DefWindowProcA(hwnd, message, w_param, l_param)
    }
}

pub fn create(parent: win32::HWND, state_ref: &crate::browser::WebBrowserRef) -> win32::HWND {
    unsafe {
        let hwnd = win32::CreateWindowExA(
            win32::WINDOW_EX_STYLE::default(),
            "DreamLoader_WebBrowser",
            "fuk",
            win32::WINDOW_STYLE::WS_CHILD,
            0,
            0,
            512,
            512,
            parent,
            win32::HMENU::default(),
            win32::HINSTANCE::default(),
            std::ptr::null_mut()
        );

        let ptr: *const _ = state_ref;
        win32::SetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX::default(), ptr as _);

        hwnd
    }
}

pub fn init() {
    let x = unsafe {
        win32::WNDCLASSA {
            lpszClassName: std::mem::transmute(b"DreamLoader_WebBrowser\0".as_ptr()),
            lpfnWndProc: Some(window_proc),
            cbWndExtra: 4,
            ..Default::default()
        }
    };

    unsafe {
        win32::RegisterClassA(&x);
    }
}
