#![windows_subsystem = "console"]

mod browser;
mod cef;
mod message_loop;
mod win32;

use detour::RawDetour;
use pelite::pe32::{Pe, PeView};
use pelite::ImageMap;
use region::Protection;
use std::{ffi::c_void, os::raw::c_char, path::Path};

// WHAT IS AUXTOOLS DOING HERE
use auxtools::sigscan;

type InsertedFunctionTableTy = extern "fastcall" fn(image_base: *const c_char, image_size: usize);

static mut HANDLE: usize = 0;
static mut GET_ORIGINAL: Option<
    extern "stdcall" fn(handle: usize, buffer: *mut u16, buffer_len: usize) -> usize,
> = None;

extern "stdcall" fn get_module_file_name_w_hook(
    handle: usize,
    buffer: *mut u16,
    buffer_len: usize,
) -> usize {
    unsafe {
        if handle == HANDLE {
            // "dreamseeker.exe\0", probably
            let src: [u16; 16] = [
                0x64,
                0x72,
                0x65,
                0x61,
                0x6D,
                0x73,
                0x65,
                0x65,
                0x6B,
                0x65,
                0x62,
                0x2E,
                0x65,
                0x78,
                0x65,
                0x00,
            ];

            let len = buffer_len.min(src.len());

            let destination = std::slice::from_raw_parts_mut(buffer, len);
            destination.copy_from_slice(&src[..len]);

            // Ensure there's always a null terminator (in case we truncated)
            if let Some(last) = destination.last_mut() {
                *last = 0x00;
            }

            // TODO: This isn't a correct implementation of GetModuleFileNameW.
            return len - 1;
        }

        (GET_ORIGINAL.unwrap())(handle, buffer, buffer_len)
    }
}

fn main() {
    // Short cut for chromium sub-processes
    if std::env::args().any(|x| x.contains("--type=")) {
        assert_eq!(cef::init(false), true);
        return;
    }

    assert_eq!(cef::init(true), false);

    // TODO: Move this to a config or something
    std::env::set_current_dir("E:\\byond_builds\\514.1552_byond\\byond\\bin").unwrap();

    let scanner = sigscan::Scanner::for_module("ntdll.dll").unwrap();

    let get_module_file_name_w = unsafe {
        let module = win32::LoadLibraryA("kernel32.dll");
        win32::GetProcAddress(module, "GetModuleFileNameW").unwrap()
    };

    unsafe {
        let detour = RawDetour::new(
            get_module_file_name_w as _,
            get_module_file_name_w_hook as _,
        )
        .unwrap();
        detour.enable().unwrap();
        GET_ORIGINAL = std::mem::transmute(detour.trampoline());
        std::mem::forget(detour);
    }

    // trahs
    let scan = scanner
        .find(sigscan::convert_signature!(
            "8B FF 55 8B EC 83 EC 0C 53 56 57 8D 45 F8 8B FA"
        ))
        .unwrap();

    let insert_func_table: InsertedFunctionTableTy = unsafe { std::mem::transmute(scan) };

    let path = Path::new("dreamseeker.exe");

    let map = ImageMap::open(path).unwrap();
    let view = PeView::from_bytes(&map).unwrap();

    let base: &[u8] = map.as_ref();
    let base = base.as_ptr();

    unsafe {
        HANDLE = std::mem::transmute(base);
    }

    let entry_point = base.wrapping_offset(view.optional_header().AddressOfEntryPoint as isize);

    //
    // RELOCATIONS
    //
    for x in view.base_relocs().unwrap().iter_blocks() {
        for word in x.words() {
            let ptr = base.wrapping_offset(x.rva_of(word) as isize) as *mut i32;
            let type_of = x.type_of(word);

            match type_of {
                0x00 => {}
                0x03 => unsafe {
                    region::protect(ptr as *const u8, 4, Protection::READ_WRITE_EXECUTE).unwrap();
                    *ptr += (base as i32) - (view.optional_header().ImageBase as i32);
                },

                _ => unimplemented!(),
            }
        }
    }

    //
    // IMPORTS
    //
    for import in view.imports().unwrap() {
        let dll_name = import.dll_name().unwrap();

        let dll_handle = unsafe { win32::LoadLibraryA(dll_name.to_string()) };

        for (iat, int) in import.iat().unwrap().zip(import.int().unwrap()) {
            let int = int.unwrap();

            let func = match int {
                pelite::pe32::imports::Import::ByName { hint: _hint, name } => unsafe {
                    win32::GetProcAddress(dll_handle, name.to_string()).unwrap()
                },

                pelite::pe32::imports::Import::ByOrdinal { ord } => unsafe {
                    win32::GetProcAddress(
                        dll_handle,
                        win32::Windows::Win32::SystemServices::PSTR(ord as *mut u8),
                    )
                    .unwrap()
                },
            };

            unsafe {
                // UNDEFINED BEHAVIOUR
                let ptr: *mut *const c_void = std::mem::transmute(iat);
                region::protect(ptr as *const u8, 4, Protection::READ_WRITE_EXECUTE).unwrap();
                *ptr = std::mem::transmute(func);
            }
        }
    }

    // for tls in view.tls().unwrap().callbacks() {
    //     let x = tls;
    //     println!("{:#?}", x);
    // }

    let entry_point: extern "cdecl" fn() = unsafe { std::mem::transmute(entry_point) };

    insert_func_table(
        base as *const _,
        view.optional_header().SizeOfImage as usize,
    );

    browser::init();
    entry_point();
}
