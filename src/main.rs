mod win32;

use std::{ffi::c_void, os::raw::c_char, path::Path};
use pelite::{ImageMap, Result};
use pelite::pe32::{Pe, PeView};
use region::Protection;
use detour::RawDetour;


// WHAT IS AUXTOOLS DOING HERE
use auxtools::sigscan;

type InsertedFunctionTableTy = extern "fastcall" fn(image_base: *const c_char, image_size: usize);

extern "stdcall" fn get_module_file_name_w_hook(handle: usize, buffer: *mut u8, buffer_len: usize) -> usize {
    unsafe {
        *buffer = b'f';
        *buffer = b'u';
        *buffer = b'k';
        *buffer = b'\0';
    }

    3
}

fn main() {
    let scanner = sigscan::Scanner::for_module("ntdll.dll").unwrap();

    let get_module_file_name_w = unsafe {
        let module = win32::LoadLibraryA("kernel32.dll");
        win32::GetProcAddress(module, "GetModuleFileNameW").unwrap()
    };

    unsafe {
        let detour = RawDetour::new(get_module_file_name_w as _, get_module_file_name_w_hook as _).unwrap();
        detour.enable().unwrap();
        std::mem::forget(detour);
    }

    // trahs
    let scan = scanner.find(sigscan::convert_signature!("8B FF 55 8B EC 83 EC 0C 53 56 57 8D 45 F8 8B FA")).unwrap();

    let insert_func_table: InsertedFunctionTableTy = unsafe {
        std::mem::transmute(scan)
    };

    let path = Path::new(r"E:\byond_builds\514.1552_byond\byond\bin\dreamdaemon.exe");

    let map = ImageMap::open(path).unwrap();
    let view = PeView::from_bytes(&map).unwrap();

    let base: &[u8] = map.as_ref();
    let base = base.as_ptr();

    let entry_point = base.wrapping_offset(view.optional_header().AddressOfEntryPoint as isize);

    //
    // RELOCATIONS
    //
    for x in view.base_relocs().unwrap().iter_blocks() {
        for word in x.words() {
            let ptr = base.wrapping_offset(x.rva_of(word) as isize) as *mut u32;
            let type_of = x.type_of(word);

            match type_of {
                0x00 => {}
                0x03 => {
                    unsafe {
                        region::protect(ptr as *const u8, 4, Protection::READ_WRITE_EXECUTE).unwrap();
                        *ptr += (base as u32) - view.optional_header().ImageBase;
                    }
                }

                _ => unimplemented!(),
            }
        }
    }


    //
    // IMPORTS
    //
    for import in view.imports().unwrap() {
        let dll_name = import.dll_name().unwrap();

        let dll_handle = unsafe {
            win32::LoadLibraryA(dll_name.to_string())
        };

        for (iat, int) in import.iat().unwrap().zip(import.int().unwrap()) {
            let int = int.unwrap();

            let func = match int {
                pelite::pe32::imports::Import::ByName { hint, name } => {
                    unsafe {
                        win32::GetProcAddress(dll_handle, name.to_string()).unwrap()
                    }
                }

                pelite::pe32::imports::Import::ByOrdinal { ord } => {
                    unsafe {
                        win32::GetProcAddress(dll_handle, win32::Windows::Win32::SystemServices::PSTR(ord as *mut u8)).unwrap()
                    }
                }
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

    let entry_point: extern "cdecl" fn() = unsafe {
        std::mem::transmute(entry_point)
    };

    insert_func_table(base as *const _, view.optional_header().SizeOfImage as usize);

    entry_point();
}
