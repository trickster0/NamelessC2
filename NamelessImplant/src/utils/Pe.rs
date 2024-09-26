use windows_sys::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_DATA_DIRECTORY};
use windows_sys::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY};
use windows_sys::Win32::Foundation::*;
use std::os::raw::c_ulong;
use crate::utils::hashstring::dbj2_hash;

pub fn PeGetFuncEat(module_handle: *mut u8, function_name_hash: u32) -> FARPROC {
    let mut address_array: u64;
    let mut name_array: u64;
    let mut name_ordinals: u64;
    let nt_headers: *const IMAGE_NT_HEADERS64;
    let data_directory: *const IMAGE_DATA_DIRECTORY;
    let export_directory: *const IMAGE_EXPORT_DIRECTORY;
    let dos_headers: *const IMAGE_DOS_HEADER;
    unsafe {
        dos_headers = module_handle as *const IMAGE_DOS_HEADER;
        nt_headers = (module_handle as u64 + (*dos_headers).e_lfanew as u64) as *const IMAGE_NT_HEADERS64;
        data_directory = (&(*nt_headers).OptionalHeader.DataDirectory[0]) as *const IMAGE_DATA_DIRECTORY;
        export_directory = (module_handle as u64 + (*data_directory).VirtualAddress as u64) as *const IMAGE_EXPORT_DIRECTORY;
        address_array = (module_handle as u64 + (*export_directory).AddressOfFunctions as u64) as u64;
        name_array = (module_handle as u64 + (*export_directory).AddressOfNames as u64) as u64;
        name_ordinals = (module_handle as u64 + (*export_directory).AddressOfNameOrdinals as u64) as u64;
        loop {
            let name_offest: u32 = *(name_array as *const u32);
            let current_function_name = std::ffi::CStr::from_ptr(
                (module_handle as u64 + name_offest as u64) as *const i8
            ).to_str().unwrap();

            if dbj2_hash(current_function_name.as_bytes()) == function_name_hash {
                address_array = address_array + (*(name_ordinals as *const u16) as u64 * (std::mem::size_of::<c_ulong>() as u64));
                let fun_addr: FARPROC = std::mem::transmute(module_handle as u64 + *(address_array as *const u32) as u64);
                return fun_addr;
            }
            name_array = name_array + std::mem::size_of::<c_ulong>() as u64;
            name_ordinals = name_ordinals + std::mem::size_of::<u16>() as u64;
        }
    }
}