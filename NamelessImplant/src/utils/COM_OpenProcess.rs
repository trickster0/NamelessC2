use com::interfaces::IUnknown;
use com::sys::HRESULT;
use std::mem::MaybeUninit;
use std::process::id;
use windows_sys::core::GUID;
use windows_sys::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_LOCAL_SERVER, CoUninitialize};
use core::ptr::null_mut;

com::interfaces! {
    #[uuid("27EB33A5-77F9-4AFE-AE05-6FDBBE720EE7")]
    pub unsafe interface ICoreShellComServerRegistrar : IUnknown {
        pub fn RegisterCOMServer(&self, one: u32, two: u32, three: u32) -> HRESULT;
        pub fn UnregisterCOMServer(&self, one: u32) -> HRESULT;
        pub fn DuplicateHandle(&self, one: u32, two: u32, three: u32, four: u32, five: u32, six: u32, seven: u32) -> HRESULT;
        pub fn OpenProcess(&self, dwDesiredAccess: u32, bInheritHandle: u32, SourceProcessId:u32, dwTargetProcessId: u32, lpTargetHandle: *mut isize) -> HRESULT;
    }
}

const GUID_ICoreShellComServerRegistrar: GUID = GUID {
    data1: 0x27EB33A5,
    data2: 0x77F9,
    data3: 0x4AFE,
    data4: [0xAE, 0x05, 0x6F, 0xDB, 0xBE, 0x72, 0x0E, 0xE7],
};

const GUID_ICoreShellComServerRegistrar_CLASS: GUID = GUID {
    data1: 0x54E14197,
    data2: 0x88B0,
    data3: 0x442F,
    data4: [0xB9, 0xA3, 0x86, 0x83, 0x70, 0x61, 0xE2, 0xFB],
};

macro_rules! out_param {
    ($param:ident) => {
        &mut $param as *mut _ as *mut _
    };
}

pub fn COM_OpenProcess(process_id: u32, access_mask:u32) -> isize{
    let mut proc_open: MaybeUninit<ICoreShellComServerRegistrar> = MaybeUninit::uninit();
    unsafe {
    let mut hr: HRESULT = CoInitialize(null_mut());
    let mut hProcess: isize = 0;
    hr = CoCreateInstance(
        &GUID_ICoreShellComServerRegistrar_CLASS,
        null_mut(),
        CLSCTX_LOCAL_SERVER,
        &GUID_ICoreShellComServerRegistrar,
        out_param!(proc_open),
    );
    proc_open.assume_init().OpenProcess(
        access_mask,
        0,
        process_id,
        id(),
        &mut hProcess,
    ); 
    CoUninitialize();
    hProcess
    }
}