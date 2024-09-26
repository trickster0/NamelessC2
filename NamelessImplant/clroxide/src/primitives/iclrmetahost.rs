use crate::primitives::{
    Class, ICLRRuntimeInfo, IEnumUnknown, IUnknown, IUnknownVtbl, Interface, RuntimeVersion, GUID,
    HRESULT,
};
use std::{collections::HashMap, ffi::c_void, ops::Deref, ptr};

#[repr(C)]
pub struct ICLRMetaHostVtbl {
    pub parent: IUnknownVtbl,
    pub GetRuntime: unsafe extern "system" fn(
        this: *mut c_void,
        pwzVersion: *mut u16,
        riid: *const GUID,
        ppRuntime: *mut *mut c_void,
    ) -> HRESULT,
    pub GetVersionFromFile: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub EnumerateInstalledRuntimes: unsafe extern "system" fn(
        this: *mut c_void,
        ppEnumerator: *mut *mut IEnumUnknown,
    ) -> HRESULT,
    pub EnumerateLoadedRuntimes: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub RequestRuntimeLoadedNotification: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub QueryLegacyV2RuntimeBinding: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
    pub ExitProcess: unsafe extern "system" fn(this: *mut c_void) -> HRESULT,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ICLRMetaHost {
    pub vtable: *const ICLRMetaHostVtbl,
}

pub type CLRCreateInstance =
    fn(class_id: *const GUID, interface_id: *const GUID, interface: *mut *mut c_void) -> HRESULT;

impl ICLRMetaHost {
    pub fn new(clr_create_instance: CLRCreateInstance) -> Result<*mut ICLRMetaHost, String> {
        let mut ppv: *mut ICLRMetaHost = ptr::null_mut();

        let hr = clr_create_instance(
            &ICLRMetaHost::CLSID,
            &ICLRMetaHost::IID,
            &mut ppv as *mut *mut _ as *mut *mut c_void,
        );

        if hr.is_err() {
            return Err(format!("{:?}", hr));
        }

        if ppv.is_null() {
            return Err("Could not retrieve ICLRMetaHost".into());
        }

        return Ok(ppv);
    }

    pub fn get_first_available_runtime(
        &self,
        prefer_version: Option<RuntimeVersion>,
    ) -> Result<*mut ICLRRuntimeInfo, String> {
        let runtimes = self.get_installed_runtimes()?;

        if let Some(prefer_version) = prefer_version {
            if let Some(runtime) = runtimes.get(&prefer_version) {
                return Ok(*runtime);
            }
        }

        for runtime in runtimes.values() {
            return Ok(*runtime);
        }

        Err("Could not find any runtimes".into())
    }

    pub fn get_runtime(&self, version: RuntimeVersion) -> Result<*mut ICLRRuntimeInfo, String> {
        let version_ptr = version.clone().to_bstr();
        let mut ppv: *mut ICLRRuntimeInfo = ptr::null_mut();

        let hr = unsafe {
            (*self).GetRuntime(
                version_ptr.into_raw() as *mut _,
                &ICLRRuntimeInfo::IID,
                &mut ppv as *mut *mut _ as *mut *mut c_void,
            )
        };

        return match hr.is_ok() {
            true => Ok(ppv),
            false => Err(format!(
                "Could not find a runtime for version `{}`: {:?}",
                version, hr
            )),
        };
    }

    pub fn get_installed_runtimes(
        &self,
    ) -> Result<HashMap<RuntimeVersion, *mut ICLRRuntimeInfo>, String> {
        let mut ieu_ptr: *mut IEnumUnknown = ptr::null_mut();

        let hr = unsafe { (*self).EnumerateInstalledRuntimes(&mut ieu_ptr) };

        if hr.is_err() {
            return Err(format!("Could not enumerate installed runtimes: {:?}", hr));
        }

        if ieu_ptr.is_null() {
            return Err("Could not enumerate installed runtimes.".into());
        }

        let mut hmri: HashMap<RuntimeVersion, *mut ICLRRuntimeInfo> = HashMap::new();

        loop {
            let mut iu_ptr: *mut IUnknown = ptr::null_mut();
            let mut cfetched: u32 = 0;

            let next_hr = unsafe { (*ieu_ptr).Next(1, &mut iu_ptr, &mut cfetched) };

            if next_hr.is_err() || iu_ptr.is_null() {
                break;
            }

            let mut ri_ptr: *mut ICLRRuntimeInfo = ptr::null_mut();

            let inner_hr = unsafe {
                (*iu_ptr).QueryInterface(
                    &ICLRRuntimeInfo::IID,
                    &mut ri_ptr as *mut _ as *mut *mut c_void,
                )
            };

            if inner_hr.is_err() || ri_ptr.is_null() {
                break;
            }

            let version = unsafe { (*ri_ptr).get_version()? };

            hmri.insert(version, ri_ptr);
        }

        Ok(hmri)
    }

    #[inline]
    pub unsafe fn GetRuntime(
        &self,
        pwzVersion: *mut u16,
        riid: *const GUID,
        ppRuntime: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).GetRuntime)(self as *const _ as *mut _, pwzVersion, riid, ppRuntime)
    }

    #[inline]
    pub unsafe fn EnumerateInstalledRuntimes(
        &self,
        ppEnumerator: *mut *mut IEnumUnknown,
    ) -> HRESULT {
        ((*self.vtable).EnumerateInstalledRuntimes)(self as *const _ as *mut _, ppEnumerator)
    }
}

impl Interface for ICLRMetaHost {
    const IID: GUID = GUID {
        data1: 0xD332DB9E,
        data2: 0xB9B3,
        data3: 0x4125,
        data4: [0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16],
    };

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Class for ICLRMetaHost {
    const CLSID: GUID = GUID {
        data1: 0x9280188d,
        data2: 0xe8e,
        data3: 0x4867,
        data4: [0xb3, 0xc, 0x7f, 0xa8, 0x38, 0x84, 0xe8, 0xde],
    };
}

impl Deref for ICLRMetaHost {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ICLRMetaHost as *const IUnknown) }
    }
}
