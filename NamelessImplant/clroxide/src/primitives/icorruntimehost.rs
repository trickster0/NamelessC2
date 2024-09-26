use std::{ffi::c_void, ops::Deref, ptr};
use windows::core::BSTR;

use crate::primitives::{
    Class, IUnknown, IUnknownVtbl, Interface, _AppDomain, GUID, HANDLE, HINSTANCE, HRESULT,
};

#[repr(C)]
pub struct ICorRuntimeHost {
    pub vtable: *const ICorRuntimeHostVtbl,
}

#[repr(C)]
pub struct ICorRuntimeHostVtbl {
    pub parent: IUnknownVtbl,
    pub CreateLogicalThreadState: unsafe extern "system" fn(this: *mut ICorRuntimeHost) -> HRESULT,
    pub DeleteLogicalThreadState: unsafe extern "system" fn(this: *mut ICorRuntimeHost) -> HRESULT,
    pub SwitchInLogicalThreadState:
        unsafe extern "system" fn(this: *mut ICorRuntimeHost, pFiberCookie: *mut u32) -> HRESULT,
    pub SwitchOutLogicalThreadState: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pFiberCookie: *mut *mut u32,
    ) -> HRESULT,
    pub LocksHeldByLogicalThread:
        unsafe extern "system" fn(this: *mut ICorRuntimeHost, pCount: *mut u32) -> HRESULT,
    pub MapFile: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        hFile: HANDLE,
        hMapAddress: *mut HINSTANCE,
    ) -> HRESULT,
    pub GetConfiguration: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pConfiguration: *mut *mut c_void,
    ) -> HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ICorRuntimeHost) -> HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ICorRuntimeHost) -> HRESULT,
    pub CreateDomain: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pwzFriendlyName: *const u16,
        pIdentityArray: *mut IUnknown,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
    pub GetDefaultDomain: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
    pub EnumDomains:
        unsafe extern "system" fn(this: *mut ICorRuntimeHost, hEnum: *mut *mut c_void) -> HRESULT,
    pub NextDomain: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        hEnum: *mut c_void,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
    pub CloseEnum:
        unsafe extern "system" fn(this: *mut ICorRuntimeHost, hEnum: *mut c_void) -> HRESULT,
    pub CreateDomainEx: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pwzFriendlyName: *const u16,
        pSetup: *mut IUnknown,
        pEvidence: *mut IUnknown,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
    pub CreateDomainSetup: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
    pub CreateEvidence: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pEvidence: *mut *mut IUnknown,
    ) -> HRESULT,
    pub UnloadDomain:
        unsafe extern "system" fn(this: *mut ICorRuntimeHost, pAppDomain: *mut IUnknown) -> HRESULT,
    pub CurrentDomain: unsafe extern "system" fn(
        this: *mut ICorRuntimeHost,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT,
}

impl ICorRuntimeHost {
    pub fn start(&self) -> Result<(), String> {
        return match unsafe { (*self).Start().ok() } {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Could not start runtime host: {:?}", e)),
        };
    }

    pub fn get_default_domain(&self) -> Result<*mut _AppDomain, String> {
        let mut unknown: *mut IUnknown = ptr::null_mut();

        let hr = unsafe { (*self).GetDefaultDomain(&mut unknown) };

        if hr.is_err() {
            return Err(format!("Could not retrieve default app domain: {:?}", hr));
        }

        if unknown.is_null() {
            return Err("Could not retrieve default app domain".into());
        }

        let mut app_domain: *mut _AppDomain = ptr::null_mut();

        let hr = unsafe {
            (*unknown).QueryInterface(
                &_AppDomain::IID,
                &mut app_domain as *mut *mut _ as *mut *mut c_void,
            )
        };

        if hr.is_err() {
            return Err(format!("Could not retrieve default app domain: {:?}", hr));
        }

        if app_domain.is_null() {
            return Err("Could not retrieve default app domain".into());
        }

        Ok(app_domain)
    }

    pub fn create_domain(&self, domain: &str) -> Result<*mut _AppDomain, String> {
        let domain_name = BSTR::from(domain);
        let unknown_array: *mut IUnknown = ptr::null_mut();
        let mut unknown: *mut IUnknown = ptr::null_mut();

        let hr = unsafe {
            (*self).CreateDomain(
                domain_name.into_raw() as *const _ as *const u16,
                unknown_array,
                &mut unknown,
            )
        };

        if hr.is_err() {
            return Err(format!(
                "Could not create app domain `{}`: {:?}",
                domain, hr
            ));
        }

        if unknown.is_null() {
            return Err(format!("Could not create app domain `{}`", domain));
        }

        let mut app_domain: *mut _AppDomain = ptr::null_mut();

        let hr = unsafe {
            (*unknown).QueryInterface(
                &_AppDomain::IID,
                &mut app_domain as *mut *mut _ as *mut *mut c_void,
            )
        };

        if hr.is_err() {
            return Err(format!(
                "Could not create app domain `{}`: {:?}",
                domain, hr
            ));
        }

        if app_domain.is_null() {
            return Err(format!("Could not create app domain `{}`", domain));
        }

        Ok(app_domain)
    }

    #[inline]
    pub unsafe fn CreateLogicalThreadState(&self) -> HRESULT {
        ((*self.vtable).CreateLogicalThreadState)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn DeleteLogicalThreadState(&self) -> HRESULT {
        ((*self.vtable).DeleteLogicalThreadState)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn SwitchInLogicalThreadState(&self, pFiberCookie: *mut u32) -> HRESULT {
        ((*self.vtable).SwitchInLogicalThreadState)(self as *const _ as *mut _, pFiberCookie)
    }

    #[inline]
    pub unsafe fn SwitchOutLogicalThreadState(&self, pFiberCookie: *mut *mut u32) -> HRESULT {
        ((*self.vtable).SwitchOutLogicalThreadState)(self as *const _ as *mut _, pFiberCookie)
    }

    #[inline]
    pub unsafe fn LocksHeldByLogicalThread(&self, pCount: *mut u32) -> HRESULT {
        ((*self.vtable).LocksHeldByLogicalThread)(self as *const _ as *mut _, pCount)
    }

    #[inline]
    pub unsafe fn MapFile(&self, hFile: HANDLE, hMapAddress: *mut HINSTANCE) -> HRESULT {
        ((*self.vtable).MapFile)(self as *const _ as *mut _, hFile, hMapAddress)
    }

    #[inline]
    pub unsafe fn GetConfiguration(&self, pConfiguration: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).GetConfiguration)(self as *const _ as *mut _, pConfiguration)
    }

    #[inline]
    pub unsafe fn Start(&self) -> HRESULT {
        ((*self.vtable).Start)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn Stop(&self) -> HRESULT {
        ((*self.vtable).Stop)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn CreateDomain(
        &self,
        pwzFriendlyName: *const u16,
        pIdentityArray: *mut IUnknown,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT {
        ((*self.vtable).CreateDomain)(
            self as *const _ as *mut _,
            pwzFriendlyName,
            pIdentityArray,
            pAppDomain,
        )
    }

    #[inline]
    pub unsafe fn GetDefaultDomain(&self, pAppDomain: *mut *mut IUnknown) -> HRESULT {
        ((*self.vtable).GetDefaultDomain)(self as *const _ as *mut _, pAppDomain)
    }

    #[inline]
    pub unsafe fn EnumDomains(&self, hEnum: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).EnumDomains)(self as *const _ as *mut _, hEnum)
    }

    #[inline]
    pub unsafe fn NextDomain(&self, hEnum: *mut c_void, pAppDomain: *mut *mut IUnknown) -> HRESULT {
        ((*self.vtable).NextDomain)(self as *const _ as *mut _, hEnum, pAppDomain)
    }

    #[inline]
    pub unsafe fn CloseEnum(&self, hEnum: *mut c_void) -> HRESULT {
        ((*self.vtable).CloseEnum)(self as *const _ as *mut _, hEnum)
    }

    #[inline]
    pub unsafe fn CreateDomainEx(
        &self,
        pwzFriendlyName: *const u16,
        pSetup: *mut IUnknown,
        pEvidence: *mut IUnknown,
        pAppDomain: *mut *mut IUnknown,
    ) -> HRESULT {
        ((*self.vtable).CreateDomainEx)(
            self as *const _ as *mut _,
            pwzFriendlyName,
            pSetup,
            pEvidence,
            pAppDomain,
        )
    }

    #[inline]
    pub unsafe fn CreateDomainSetup(&self, pAppDomain: *mut *mut IUnknown) -> HRESULT {
        ((*self.vtable).CreateDomainSetup)(self as *const _ as *mut _, pAppDomain)
    }

    #[inline]
    pub unsafe fn CreateEvidence(&self, pEvidence: *mut *mut IUnknown) -> HRESULT {
        ((*self.vtable).CreateEvidence)(self as *const _ as *mut _, pEvidence)
    }

    #[inline]
    pub unsafe fn UnloadDomain(&self, pAppDomain: *mut IUnknown) -> HRESULT {
        ((*self.vtable).UnloadDomain)(self as *const _ as *mut _, pAppDomain)
    }

    #[inline]
    pub unsafe fn CurrentDomain(&self, pAppDomain: *mut *mut IUnknown) -> HRESULT {
        ((*self.vtable).CurrentDomain)(self as *const _ as *mut _, pAppDomain)
    }
}

impl Interface for ICorRuntimeHost {
    const IID: GUID = GUID::from_values(
        0xCB2F6722,
        0xAB3A,
        0x11d2,
        [0x9C, 0x40, 0x00, 0xC0, 0x4F, 0xA3, 0x0A, 0x3E],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Class for ICorRuntimeHost {
    const CLSID: GUID = GUID::from_values(
        0xcb2f6723,
        0xab3a,
        0x11d2,
        [0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e],
    );
}

impl Deref for ICorRuntimeHost {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ICorRuntimeHost as *const IUnknown) }
    }
}
