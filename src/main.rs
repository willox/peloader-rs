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
    extern "stdcall" fn(handle: usize, buffer: *mut u8, buffer_len: usize) -> usize,
> = None;

extern "stdcall" fn get_module_file_name_w_hook(
    handle: usize,
    buffer: *mut u8,
    buffer_len: usize,
) -> usize {
    unsafe {
        if handle == HANDLE {
            if buffer_len < 16 {
                // TODO: implement
                return 0;
            }

            *(buffer.offset(0)) = b'd';
            *(buffer.offset(1)) = b'r';
            *(buffer.offset(2)) = b'e';
            *(buffer.offset(3)) = b'a';
            *(buffer.offset(4)) = b'm';
            *(buffer.offset(5)) = b's';
            *(buffer.offset(6)) = b'e';
            *(buffer.offset(7)) = b'e';
            *(buffer.offset(8)) = b'k';
            *(buffer.offset(9)) = b'e';
            *(buffer.offset(10)) = b'r';
            *(buffer.offset(11)) = b'.';
            *(buffer.offset(12)) = b'e';
            *(buffer.offset(13)) = b'x';
            *(buffer.offset(14)) = b'e';
            *(buffer.offset(15)) = b'\0';

            return 15;
        }

        (GET_ORIGINAL.unwrap())(handle, buffer, buffer_len)
    }
}

fn main() {
    if std::env::args().any(|x| x.contains("--type=")) {
        assert!(cef::init(false));
        return;
    }

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
