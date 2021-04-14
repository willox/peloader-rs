fn main() {
    cc::Build::new()
        .cpp(true)
        .file("test.cpp")
        .compile("test.lib");

    windows::build!(
        Windows::Win32::SystemServices::LoadLibraryA,
        Windows::Win32::SystemServices::GetProcAddress,
        Windows::Win32::SystemServices::PSTR,
        Windows::Win32::SystemServices::GetModuleHandleA,
        Windows::Win32::WindowsAndMessaging::{
            CreateWindowExA,
            ShowWindow,
            RegisterClassA,
            DefWindowProcA,
            WM_PAINT
        },
        Windows::Win32::Gdi::{
            BeginPaint,
            EndPaint,
            InvalidateRect,
            TextOutA,
        },
    );
}
