use crate::primitives::{Interface, GUID, HRESULT};
use std::{ffi::c_void, mem::transmute_copy};

#[repr(C)]
pub struct IUnknown {
    pub vtable: *const IUnknownVtbl,
}

#[repr(C)]
pub struct IUnknownVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut c_void) -> u32,
}

impl IUnknown {
    #[inline]
    pub unsafe fn QueryInterface(&self, riid: *const GUID, ppvObject: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).QueryInterface)(self as *const _ as *mut _, riid, ppvObject)
    }

    #[inline]
    pub unsafe fn AddRef(&self) -> u32 {
        ((*self.vtable).AddRef)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn Release(&self) -> u32 {
        ((*self.vtable).Release)(self as *const _ as *mut _)
    }
}

impl Interface for IUnknown {
    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            self.AddRef();
        }

        unsafe { transmute_copy(self) }
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            self.Release();
        }
    }
}

impl std::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IUnknown").field(&self).finish()
    }
}
