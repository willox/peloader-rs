fn main() {
    windows::build!(
        Windows::Win32::SystemServices::LoadLibraryA,
        Windows::Win32::SystemServices::GetProcAddress,
        Windows::Win32::WindowsAndMessaging::CreateWindowExA,
    );
}
