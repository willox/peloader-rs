fn main() {
    cc::Build::new()
        .cpp(true)
        .file("test.cpp")
        .compile("test.lib");

    windows::build!(
        Windows::Win32::KeyboardAndMouseInput::{
            GetFocus,
            EnableWindow,
        },
        Windows::Win32::SystemServices::LoadLibraryA,
        Windows::Win32::SystemServices::GetProcAddress,
        Windows::Win32::SystemServices::PSTR,
        Windows::Win32::SystemServices::GetModuleHandleA,
        Windows::Win32::DisplayDevices::RECT,
        Windows::Win32::WindowsAndMessaging::{
            CreateWindowExA,
            GetClassNameA,
            GetParent,
            SetWindowPos,
            ShowWindow,
            RegisterClassA,
            DefWindowProcA,
            SendNotifyMessageA,
            SendMessageA,
            PostMessageA,
            SetWindowLongA,
            GetWindowLongA,
            SetTimer,
            KillTimer,
            TranslateMessage,
            DispatchMessageA,
            WM_PAINT,
            WM_SIZE,
            WM_MOVE,
            WM_NCCALCSIZE,
            WM_TIMER,
            WM_CLOSE,
            WM_USER,
            HWND_MESSAGE,
            WINDOW_STYLE,
            MSG,
        },
        Windows::Win32::Gdi::{
            BeginPaint,
            EndPaint,
            InvalidateRect,
            TextOutA,
        },
    );
}
