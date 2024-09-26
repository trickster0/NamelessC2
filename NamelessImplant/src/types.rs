use core::ffi::c_void;
use windows_sys::core::PCSTR;
use windows_sys::core::PSTR;

pub type HINTERNET = *const c_void;
pub type HCONNECT = *const c_void;
pub type InternetOpenA = unsafe extern "system" fn (PCSTR, u32, PCSTR, PCSTR, u32) -> *mut c_void ;
pub type LoadLibraryA = unsafe extern "system" fn (PCSTR) -> *mut u8;
pub type InternetConnectA = unsafe extern "system" fn (HINTERNET, PCSTR, u16, PCSTR, PCSTR, u32, u32, usize) -> *mut c_void;
pub type HttpSendRequestA = unsafe extern "system" fn (HINTERNET, PCSTR, u32, *const c_void, u32) -> i32;
pub type HttpOpenRequestA = unsafe extern "system" fn (HCONNECT, PCSTR, PCSTR, PCSTR, PCSTR, *const PSTR, u32, usize) -> *mut c_void;
pub type InternetReadFile = unsafe extern "system" fn (HINTERNET, *mut u8, u32, *mut u32) -> i32;
pub type InternetCloseHandle = unsafe extern "system" fn (HINTERNET) -> i32;
pub type InternetSetOptionA = unsafe extern "system" fn (HINTERNET, u32, *const u8, u32) -> i32;
pub type HttpAddRequestHeadersA = unsafe extern "system" fn (HINTERNET, PCSTR, u32, u32) -> i32;
pub type HttpQueryInfoA = unsafe extern "system" fn (HINTERNET, u32, *mut u8, &u32, *mut u32) -> i32;