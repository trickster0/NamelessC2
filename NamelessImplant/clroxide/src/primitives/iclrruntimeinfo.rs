use crate::primitives::{
    Class, ICorRuntimeHost, IUnknown, IUnknownVtbl, Interface, BOOL, GUID, HANDLE, HRESULT,
};
use std::{ffi::c_void, fmt::Display, ops::Deref, ptr};
use windows::core::{BSTR, PWSTR};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeVersion {
    V2,
    V3,
    V4,
    UNKNOWN,
}

impl RuntimeVersion {
    pub fn to_str(&self) -> &str {
        match self {
            RuntimeVersion::V2 => "v2.0.50727",
            RuntimeVersion::V3 => "v3.0",
            RuntimeVersion::V4 => "v4.0.30319",
            RuntimeVersion::UNKNOWN => "UNKNOWN",
        }
    }

    pub fn to_bstr(&self) -> BSTR {
        BSTR::from(self.to_str())
    }
}

impl From<String> for RuntimeVersion {
    fn from(version: String) -> Self {
        match version.as_str() {
            "v2.0.50727" => RuntimeVersion::V2,
            "v3.0" => RuntimeVersion::V3,
            "v4.0.30319" => RuntimeVersion::V4,
            _ => RuntimeVersion::UNKNOWN,
        }
    }
}

impl Display for RuntimeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[repr(C)]
pub struct ICLRRuntimeInfo {
    pub vtable: *const ICLRRuntimeInfoVtbl,
}

#[repr(C)]
pub struct ICLRRuntimeInfoVtbl {
    pub parent: IUnknownVtbl,
    pub GetVersionString: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
    ) -> HRESULT,
    pub GetRuntimeDirectory: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
    ) -> HRESULT,
    pub IsLoaded: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        hndProcess: HANDLE,
        pbLoaded: *mut BOOL,
    ) -> HRESULT,
    pub LoadErrorString: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        iResourceID: u32,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
        iLocaleID: u32,
    ) -> HRESULT,
    pub LoadLibrary: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pwzDllName: *const u16,
        ppProc: *mut *mut c_void,
    ) -> HRESULT,
    pub GetProcAddress: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pszProcName: *const i8,
        ppProc: *mut *mut c_void,
    ) -> HRESULT,
    pub GetInterface: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        rclsid: *const GUID,
        riid: *const GUID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT,
    pub IsLoadable:
        unsafe extern "system" fn(this: *mut ICLRRuntimeInfo, pbLoadable: *mut BOOL) -> HRESULT,
    pub SetDefaultStartupFlags: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        dwStartupFlags: u32,
        pwzHostConfigFile: *const u16,
    ) -> HRESULT,
    pub GetDefaultStartupFlags: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pdwStartupFlags: *mut u32,
        pwzHostConfigFile: *mut u16,
        pcchHostConfigFile: *mut u32,
    ) -> HRESULT,
    pub BindAsLegacyV2Runtime: unsafe extern "system" fn(this: *mut ICLRRuntimeInfo) -> HRESULT,
    pub IsStarted: unsafe extern "system" fn(
        this: *mut ICLRRuntimeInfo,
        pbStarted: *mut BOOL,
        pdwStartupFlags: *mut u32,
    ) -> HRESULT,
}

impl ICLRRuntimeInfo {
    pub fn get_runtime_host(&self) -> Result<*mut ICorRuntimeHost, String> {
        let mut ppv: *mut ICorRuntimeHost = ptr::null_mut();

        let hr = unsafe {
            (*self).GetInterface(
                &ICorRuntimeHost::CLSID,
                &ICorRuntimeHost::IID,
                &mut ppv as *mut *mut _ as *mut *mut c_void,
            )
        };

        if hr.is_err() {
            return Err(format!("Could not retrieve ICorRuntimeHost: {:?}", hr));
        }

        if ppv.is_null() {
            return Err("Could not retrieve ICorRuntimeHost".into());
        }

        return Ok(ppv);
    }

    pub fn get_version(&self) -> Result<RuntimeVersion, String> {
        let dummy = ptr::null_mut();
        let mut length = 0;

        let _ = unsafe { (*self).GetVersionString(dummy, &mut length) };

        let mut buffer: Vec<u16> = Vec::with_capacity(length as usize);
        let version = PWSTR(buffer.as_mut_ptr());

        let hr = unsafe { (*self).GetVersionString(version.as_ptr(), &mut length) };

        if hr.is_err() {
            return Err(format!("Failed while running `GetVersionString`: {:?}", hr));
        }

        Ok(RuntimeVersion::from(unsafe {
            version.to_string().unwrap_or_default()
        }))
    }

    pub fn can_be_loaded(&self) -> Result<bool, String> {
        let mut loadable = BOOL(0);

        let hr = unsafe { (*self).IsLoadable(&mut loadable) };

        if hr.is_err() {
            return Err(format!("Failed while running `IsLoadable`: {:?}", hr));
        }

        Ok(loadable.0 > 0)
    }

    pub fn has_started(&self) -> Result<bool, String> {
        let mut startup_flags = 0;
        let mut started = BOOL(0);

        let hr = unsafe { (*self).IsStarted(&mut started, &mut startup_flags) };

        if hr.is_err() {
            return Err(format!("Failed while running `IsStarted`: {:?}", hr));
        }

        Ok(started.0 > 0)
    }

    #[inline]
    pub unsafe fn GetVersionString(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT {
        ((*self.vtable).GetVersionString)(self as *const _ as *mut _, pwzBuffer, pcchBuffer)
    }

    #[inline]
    pub unsafe fn GetRuntimeDirectory(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT {
        ((*self.vtable).GetRuntimeDirectory)(self as *const _ as *mut _, pwzBuffer, pcchBuffer)
    }

    #[inline]
    pub unsafe fn IsLoaded(&self, hndProcess: HANDLE, pbLoaded: *mut BOOL) -> HRESULT {
        ((*self.vtable).IsLoaded)(self as *const _ as *mut _, hndProcess, pbLoaded)
    }

    #[inline]
    pub unsafe fn LoadErrorString(
        &self,
        iResourceID: u32,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
        iLocaleID: u32,
    ) -> HRESULT {
        ((*self.vtable).LoadErrorString)(
            self as *const _ as *mut _,
            iResourceID,
            pwzBuffer,
            pcchBuffer,
            iLocaleID,
        )
    }

    #[inline]
    pub unsafe fn LoadLibrary(&self, pwzDllName: *const u16, ppProc: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).LoadLibrary)(self as *const _ as *mut _, pwzDllName, ppProc)
    }

    #[inline]
    pub unsafe fn GetProcAddress(
        &self,
        pszProcName: *const i8,
        ppProc: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).GetProcAddress)(self as *const _ as *mut _, pszProcName, ppProc)
    }

    #[inline]
    pub unsafe fn GetInterface(
        &self,
        rclsid: *const GUID,
        riid: *const GUID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).GetInterface)(self as *const _ as *mut _, rclsid, riid, ppUnk)
    }

    #[inline]
    pub unsafe fn IsLoadable(&self, pbLoadable: *mut BOOL) -> HRESULT {
        ((*self.vtable).IsLoadable)(self as *const _ as *mut _, pbLoadable)
    }

    #[inline]
    pub unsafe fn SetDefaultStartupFlags(
        &self,
        dwStartupFlags: u32,
        pwzHostConfigFile: *const u16,
    ) -> HRESULT {
        ((*self.vtable).SetDefaultStartupFlags)(
            self as *const _ as *mut _,
            dwStartupFlags,
            pwzHostConfigFile,
        )
    }

    #[inline]
    pub unsafe fn GetDefaultStartupFlags(
        &self,
        pdwStartupFlags: *mut u32,
        pwzHostConfigFile: *mut u16,
        pcchHostConfigFile: *mut u32,
    ) -> HRESULT {
        ((*self.vtable).GetDefaultStartupFlags)(
            self as *const _ as *mut _,
            pdwStartupFlags,
            pwzHostConfigFile,
            pcchHostConfigFile,
        )
    }

    #[inline]
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> HRESULT {
        ((*self.vtable).BindAsLegacyV2Runtime)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn IsStarted(&self, pbStarted: *mut BOOL, pdwStartupFlags: *mut u32) -> HRESULT {
        ((*self.vtable).IsStarted)(self as *const _ as *mut _, pbStarted, pdwStartupFlags)
    }
}

impl Interface for ICLRRuntimeInfo {
    const IID: GUID = GUID::from_values(
        0xBD39D1D2,
        0xBA2F,
        0x486a,
        [0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for ICLRRuntimeInfo {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ICLRRuntimeInfo as *const IUnknown) }
    }
}
