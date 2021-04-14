::windows::include_bindings!();

pub use Windows::Win32::SystemServices::LoadLibraryA;
pub use Windows::Win32::SystemServices::GetProcAddress;
pub use Windows::Win32::WindowsAndMessaging::{
    CreateWindowExA,
    HWND,
    WINDOW_STYLE,
    WINDOW_EX_STYLE
};
pub use Windows::Win32::MenusAndResources::{
    HMENU,
};
pub use Windows::Win32::SystemServices::HINSTANCE;
