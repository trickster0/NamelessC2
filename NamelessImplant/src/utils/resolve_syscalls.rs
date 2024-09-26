
use std::arch::global_asm;
use crate::utils::Peb::PebGetModule;
use crate::utils::Pe::PeGetFuncEat;
use windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW;
pub type RtlGetVersion = unsafe extern "system" fn (*mut OSVERSIONINFOW) -> isize;

#[macro_export]
macro_rules! syscall {
    ($function_address:expr, $($y:expr), +) => {
        {
        let (ssn, addr) = $crate::utils::resolve_syscalls::get_syscall_instruction_address($function_address);
        let mut cnt:u32 = 0;
        $(
            let _ = $y;
            cnt += 1;
        )+
        do_syscall(ssn, addr, cnt, $($y), +)
    }}
}

#[cfg(target_arch = "x86_64")]
global_asm!(
    "
.global do_syscall

.section .text

do_syscall:
    mov [rsp - 0x8],  rsi
    mov [rsp - 0x10], rdi
    mov [rsp - 0x18], r12

    mov eax, ecx
    mov r12, rdx
    mov rcx, r8

    mov r10, r9
    mov  rdx,  [rsp + 0x28]
    mov  r8,   [rsp + 0x30]
    mov  r9,   [rsp + 0x38]

    sub rcx, 0x4
    jle skip

    lea rsi,  [rsp + 0x40]
    lea rdi,  [rsp + 0x28]

    rep movsq
skip:

    mov rcx, r12

    mov rsi, [rsp - 0x8]
    mov rdi, [rsp - 0x10]
    mov r12, [rsp - 0x18]

    jmp rcx
"
);

#[cfg(target_arch = "x86")]
global_asm!(
    "
.global _do_syscall

.section .text

_do_syscall:
    mov ecx, [esp + 0x0C]
    not ecx
    add ecx, 1
    lea edx, [esp + ecx * 4]

    mov ecx, [esp]
    mov [edx], ecx

    mov [edx - 0x04], esi
    mov [edx - 0x08], edi

    mov eax, [esp + 0x04]
    mov ecx, [esp + 0x0C]

    lea esi, [esp + 0x10]
    lea edi, [edx + 0x04]

    rep movsd

    mov esi, [edx - 0x04]
    mov edi, [edx - 0x08]
    mov ecx, [esp + 0x08]
    
    mov esp, edx

    mov edx, fs:[0xC0]
    test edx, edx
    je native

    mov edx, fs:[0xC0]
    jmp ecx

native:
    mov edx, ecx
    sub edx, 0x05
    push edx
    mov edx, esp
    jmp ecx
    ret

is_wow64:
"
);

#[cfg(target_arch = "x86_64")]
extern "C" {
    pub fn do_syscall(ssn: u16, syscall_addr: u64, n_args: u32, ...) -> i32;
}

#[cfg(target_arch = "x86")]
extern "C" {
    pub fn do_syscall(ssn: u16, n_args: u32, syscall_addr: u32, ...) -> i32;
}

const UP: isize = -32;
const DOWN: usize = 32;

pub unsafe fn get_syscall_instruction_address(function_address: *mut u8) -> (u16, u64) {

    let hNtdll = PebGetModule(0x1edab0ed);

    let dn_rtlGetVersion: RtlGetVersion = std::mem::transmute(PeGetFuncEat(hNtdll, 0xdde5cdd));

    let mut osversion: OSVERSIONINFOW = unsafe {std::mem::zeroed()};
    dn_rtlGetVersion(&mut osversion);

    let mut offset = 0;

    if osversion.dwMajorVersion < 10 {
        offset = 8;
    }else {
        offset = 18;
    }
    
    let NtAddBootEntry_SSN = std::mem::transmute::<Option<unsafe extern "system" fn() -> isize>, u64>(PeGetFuncEat(hNtdll, 0x8cfcc776)) + offset ;
    // 
    // Hell's gate for `syscall` instruction rather than `SSN`
    //

    // check if the assembly instruction are (0x0f, 0x05, 0xc3):
    // syscall
    // ret
    if function_address.add(18).read() == 0x0f && function_address.add(19).read() == 0x05 && function_address.add(20).read() == 0xc3  {
        let high = *(function_address).add(5) as u64;
        let low = *(function_address).add(4) as u64;
        return ((high << 8 | low) as u16, NtAddBootEntry_SSN);
    }

    //
    // Halo's Gate and Tartarus' Gate Patch for `syscall` instruction rather than `SSN`
    //
    if function_address.read() == 0xe9 || function_address.add(3).read() == 0xe9 {
        for idx in 1..500 {
            
            //
            // if hooked check the neighborhood to find clean syscall (downwards)
            //
            
            if function_address.add(18 + idx * DOWN).read() == 0x0f && function_address.add(19 + idx * DOWN).read() == 0x05 && function_address.add(20 + idx * DOWN).read() == 0xc3 {
                let high = *(function_address).add(5 + idx * DOWN) as u64;
                let low = *(function_address).add(4 + idx * DOWN) as u64;
                return  (((high << 8) | low - idx as u64) as u16, NtAddBootEntry_SSN);
            }
            
            //
            // if hooked check the neighborhood to find clean syscall (upwards)
            //
            if function_address.offset(18 + idx as isize * UP).read() == 0x0f && function_address.offset(19 + idx as isize * UP).read() == 0x05 && function_address.offset(20 + idx as isize * UP).read() == 0xc3 {
                let high = *(function_address).add(5 + idx * UP as usize) as u64;
                let low = *(function_address).add(4 + idx * UP as usize) as u64;
                return  (((high << 8) | low + idx as u64) as u16, NtAddBootEntry_SSN);
            }
        }
    }

    return (0, 0);
}