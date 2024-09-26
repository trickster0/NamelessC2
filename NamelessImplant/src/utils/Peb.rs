use windows_sys::Win32::System::Kernel::LIST_ENTRY;
use windows_sys::Win32::System::WindowsProgramming::LDR_DATA_TABLE_ENTRY;
use windows_sys::Win32::System::Threading::PEB;
use core::arch::asm;
use crate::utils::hashstring::dbj2_hash;

#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn getpeb() -> u64 {
    let out: u64;
    asm!(
        "mov {}, gs:[0x60]",
        lateout(reg) out,
        options(nostack, pure, readonly),
    );
    out
}

#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn getpeb() -> u32 {
    let out: u32;
    asm!(
        "mov {}, fs:[0x30]",
        lateout(reg) out,
        options(nostack, pure, readonly),
    );
    out
}


pub fn PebGetModule(module_hash: u32) -> *mut u8 {
    unsafe {
        let peb_offset: *const u64 = getpeb() as *const u64;
        let rf_peb: *const PEB = peb_offset as * const PEB;
        let peb = *rf_peb;

        let mut p_ldr_data_table_entry: *const LDR_DATA_TABLE_ENTRY = (*peb.Ldr).InMemoryOrderModuleList.Flink as *const LDR_DATA_TABLE_ENTRY;
        let mut p_list_entry = &(*peb.Ldr).InMemoryOrderModuleList as *const LIST_ENTRY;

        loop {
            let buffer: &[u8] = core::slice::from_raw_parts(
                (*p_ldr_data_table_entry).FullDllName.Buffer as *const u8,
                (*p_ldr_data_table_entry).FullDllName.Length as usize);

            if module_hash == dbj2_hash(buffer) {
                return (*p_ldr_data_table_entry).Reserved2[0] as *mut u8;
            }

            if p_list_entry == (*peb.Ldr).InMemoryOrderModuleList.Blink {
                println!("Module not found!");
                return std::ptr::null_mut();
            }
            p_list_entry = (*p_list_entry).Flink;
            p_ldr_data_table_entry = (*p_list_entry).Flink as *const LDR_DATA_TABLE_ENTRY;
        }
    }
}