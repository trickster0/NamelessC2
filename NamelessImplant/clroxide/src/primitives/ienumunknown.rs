use crate::primitives::{IUnknown, IUnknownVtbl, Interface, GUID, HRESULT};
use std::{ffi::c_void, ops::Deref};

#[repr(C)]
pub struct IEnumUnknown {
    pub vtable: *const IEnumUnknownVtbl,
}

#[repr(C)]
pub struct IEnumUnknownVtbl {
    pub parent: IUnknownVtbl,
    pub Next: unsafe extern "system" fn(
        this: *mut c_void,
        celt: u32,
        rgelt: *mut *mut IUnknown,
        pceltFetched: *mut u32,
    ) -> HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut c_void, celt: u32) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub Clone:
        unsafe extern "system" fn(this: *mut c_void, ppenum: *mut *mut IEnumUnknown) -> HRESULT,
}

impl IEnumUnknown {
    #[inline]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut *mut IUnknown,
        pceltFetched: *mut u32,
    ) -> HRESULT {
        ((*self.vtable).Next)(self as *const _ as *mut _, celt, rgelt, pceltFetched)
    }

    #[inline]
    pub unsafe fn Skip(&self, celt: u32) -> HRESULT {
        ((*self.vtable).Skip)(self as *const _ as *mut _, celt)
    }

    #[inline]
    pub unsafe fn Reset(&self) -> HRESULT {
        ((*self.vtable).Reset)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn Clone(&self, ppenum: *mut *mut IEnumUnknown) -> HRESULT {
        ((*self.vtable).Clone)(self as *const _ as *mut _, ppenum)
    }
}

impl Interface for IEnumUnknown {
    const IID: GUID = GUID::from_values(
        0x00000100,
        0x0000,
        0x0000,
        [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for IEnumUnknown {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const IEnumUnknown as *const IUnknown) }
    }
}
