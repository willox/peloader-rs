use crate::win32;

extern "system" fn window_proc(
    hwnd: win32::HWND,
    message: u32,
    w_param: win32::WPARAM,
    l_param: win32::LPARAM
) -> win32::LRESULT {
    let mut ps = win32::PAINTSTRUCT::default();

    println!("{}", message);

    if message == win32::WM_SIZE {
        let l_param = l_param.0 as u64;
        println!("WM_SIZE {} {}", (l_param & 0xFFFFFFFF00000000) << 32, (l_param & 0xFFFFFFFF));
    }

    if message == win32::WM_PAINT {
        unsafe {
            let hdc = win32::BeginPaint(hwnd, &mut ps);
            win32::TextOutA(hdc, 0, 0, "Step 1) Delete internet explorer Step 2) ??? Step 3) Profit", 57);
            win32::EndPaint(hwnd, &mut ps);

        }
        return win32::LRESULT::default();
    }

    unsafe {
        win32::DefWindowProcA(hwnd, message, w_param, l_param)
    }
}

pub fn create(parent: win32::HWND) -> win32::HWND {
    unsafe {
        win32::CreateWindowExA(
            win32::WINDOW_EX_STYLE::WS_EX_CLIENTEDGE,
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
        )
    }
}

pub fn init() {
    let x = unsafe {
        win32::WNDCLASSA {
            lpszClassName: std::mem::transmute(b"DreamLoader_WebBrowser\0".as_ptr()),
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        }
    };

    unsafe {
        win32::RegisterClassA(&x);
    }
}
