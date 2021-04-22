use crate::win32;

unsafe fn is_child(parent: win32::HWND, mut child: win32::HWND) -> bool {
    while child != win32::HWND(0) {
        child = win32::GetParent(child);
        if parent == child {
            return true
        }
    }
    false
}

extern "system" fn window_proc(
    hwnd: win32::HWND,
    message: u32,
    w_param: win32::WPARAM,
    l_param: win32::LPARAM,
) -> win32::LRESULT {
    // TODO: Here lies an awful hack to send focus events to CEF when it is focused
    // Yeah that sucks. A better hack would be to hook into the CEF widget's messages.
    if message == win32::WM_SIZE {
        unsafe {
            win32::SetTimer(hwnd, 0, 1, None);
        }
    } else if message == win32::WM_TIMER {
        let focused = unsafe { win32::GetFocus() };
        println!("Focus = {}", focused.0);

        let is_child = unsafe {
            is_child(hwnd, focused)
        };

        let state_ref: &crate::browser::WebBrowserRef = unsafe {
            let ptr = win32::GetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX::default())
                as *const crate::browser::WebBrowserRef;
            std::mem::transmute(ptr)
        };

        state_ref.focus_update(is_child);
    }

    // TODO: lazy
    if message == 0x0400 {
        let state_ref: &crate::browser::WebBrowserRef = unsafe {
            let ptr = win32::GetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX::default())
                as *const crate::browser::WebBrowserRef;
            std::mem::transmute(ptr)
        };

        state_ref.process_events();
    }

    unsafe { win32::DefWindowProcA(hwnd, message, w_param, l_param) }
}

pub fn create(parent: win32::HWND, state_ref: &crate::browser::WebBrowserRef) -> win32::HWND {
    unsafe {
        let hwnd = win32::CreateWindowExA(
            win32::WINDOW_EX_STYLE::from(win32::WINDOW_EX_STYLE::WS_EX_NOACTIVATE.0),
            "DreamLoader_WebBrowser",
            "fuk",
            win32::WINDOW_STYLE::from(
                win32::WINDOW_STYLE::WS_CHILD.0 | win32::WINDOW_STYLE::WS_VISIBLE.0,
            ),
            0,
            0,
            32,
            32,
            parent,
            win32::HMENU::default(),
            win32::HINSTANCE::default(),
            std::ptr::null_mut(),
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
