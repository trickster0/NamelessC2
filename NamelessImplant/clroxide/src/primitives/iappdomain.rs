use crate::primitives::{
    itype::_Type, IUnknown, IUnknownVtbl, Interface, _Assembly, prepare_assembly, GUID, HRESULT,
};
use std::{
    ffi::{c_long, c_void},
    ops::Deref,
    ptr,
};
use windows::{core::BSTR, Win32::System::Com::SAFEARRAY};

#[repr(C)]
pub struct _AppDomain {
    pub vtable: *const _AppDomainVtbl,
}

unsafe impl Sync for _AppDomain {}
unsafe impl Send for _AppDomain {}

#[repr(C)]
pub struct _AppDomainVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut c_long) -> HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub InitializeLifetimeService: *const c_void,
    pub GetLifetimeService: *const c_void,
    pub get_Evidence: *const c_void,
    pub set_Evidence: *const c_void,
    pub get_DomainUnload: *const c_void,
    pub set_DomainUnload: *const c_void,
    pub get_AssemblyLoad: *const c_void,
    pub set_AssemblyLoad: *const c_void,
    pub get_ProcessExit: *const c_void,
    pub set_ProcessExit: *const c_void,
    pub get_TypeResolve: *const c_void,
    pub set_TypeResolve: *const c_void,
    pub get_ResourceResolve: *const c_void,
    pub set_ResourceResolve: *const c_void,
    pub get_AssemblyResolve: *const c_void,
    pub get_UnhandledException: *const c_void,
    pub set_UnhandledException: *const c_void,
    pub DefineDynamicAssembly: *const c_void,
    pub DefineDynamicAssembly_2: *const c_void,
    pub DefineDynamicAssembly_3: *const c_void,
    pub DefineDynamicAssembly_4: *const c_void,
    pub DefineDynamicAssembly_5: *const c_void,
    pub DefineDynamicAssembly_6: *const c_void,
    pub DefineDynamicAssembly_7: *const c_void,
    pub DefineDynamicAssembly_8: *const c_void,
    pub DefineDynamicAssembly_9: *const c_void,
    pub CreateInstance: *const c_void,
    pub CreateInstanceFrom: *const c_void,
    pub CreateInstance_2: *const c_void,
    pub CreateInstanceFrom_2: *const c_void,
    pub CreateInstance_3: *const c_void,
    pub CreateInstanceFrom_3: *const c_void,
    pub Load: *const c_void,
    pub Load_2: unsafe extern "system" fn(
        this: *mut c_void,
        assemblyString: *mut u16,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    pub Load_3: unsafe extern "system" fn(
        this: *mut c_void,
        rawAssembly: *mut SAFEARRAY,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    pub Load_4: *const c_void,
    pub Load_5: *const c_void,
    pub Load_6: *const c_void,
    pub Load_7: *const c_void,
    pub ExecuteAssembly: *const c_void,
    pub ExecuteAssembly_2: *const c_void,
    pub ExecuteAssembly_3: *const c_void,
    pub get_FriendlyName: *const c_void,
    pub get_BaseDirectory: *const c_void,
    pub get_RelativeSearchPath: *const c_void,
    pub get_ShadowCopyFiles: *const c_void,
    pub GetAssemblies: *const c_void,
    pub AppendPrivatePath: *const c_void,
    pub ClearPrivatePath: *const c_void,
    pub ClearShadowCopyPath: *const c_void,
    pub SetData: *const c_void,
    pub GetData: *const c_void,
    pub SetAppDomainPolicy: *const c_void,
    pub SetThreadPrincipal: *const c_void,
    pub SetPrincipalPolicy: *const c_void,
    pub DoCallBack: *const c_void,
    pub get_DynamicDirectory: *const c_void,
}

impl _AppDomain {
    pub fn load_library(&self, library: &str) -> Result<*mut _Assembly, String> {
        let library_buffer = BSTR::from(library);

        let mut library_ptr: *mut _Assembly = ptr::null_mut();

        let hr = unsafe { (*self).Load_2(library_buffer.into_raw() as *mut _, &mut library_ptr) };

        if hr.is_err() {
            return Err(format!("Could not retrieve `{}`: {:?}", library, hr));
        }

        if library_ptr.is_null() {
            return Err(format!("Could not retrieve `{}`", library));
        }

        Ok(library_ptr)
    }

    pub fn load_assembly(&self, bytes: &[u8]) -> Result<*mut _Assembly, String> {
        let assembly_bytes = prepare_assembly(bytes)?;

        let mut assembly_ptr: *mut _Assembly = ptr::null_mut();

        let hr = unsafe { (*self).Load_3(assembly_bytes, &mut assembly_ptr) };

        if hr.is_err() {
            return Err(format!("Could not retrieve assembly: {:?}", hr));
        }

        if assembly_ptr.is_null() {
            return Err("Could not retrieve assembly".into());
        }

        Ok(assembly_ptr)
    }

    pub fn to_string(&self) -> Result<String, String> {
        let mut buffer = BSTR::new();

        let hr = unsafe { (*self).ToString(&mut buffer as *mut _ as *mut *mut u16) };

        if hr.is_err() {
            return Err(format!("Failed while running `ToString`: {:?}", hr));
        }

        Ok(buffer.to_string())
    }

    #[inline]
    pub unsafe fn ToString(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).ToString)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetHashCode(&self, pRetVal: *mut c_long) -> HRESULT {
        ((*self.vtable).GetHashCode)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetType(&self, pRetVal: *mut *mut _Type) -> HRESULT {
        ((*self.vtable).GetType)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn Load_2(&self, assemblyString: *mut u16, pRetVal: *mut *mut _Assembly) -> HRESULT {
        ((*self.vtable).Load_2)(self as *const _ as *mut _, assemblyString, pRetVal)
    }

    #[inline]
    pub unsafe fn Load_3(
        &self,
        rawAssembly: *mut SAFEARRAY,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT {
        ((*self.vtable).Load_3)(self as *const _ as *mut _, rawAssembly, pRetVal)
    }
}

impl Interface for _AppDomain {
    const IID: GUID = GUID::from_values(
        0x05F696DC,
        0x2B29,
        0x3663,
        [0xAD, 0x8B, 0xC4, 0x38, 0x9C, 0xF2, 0xA7, 0x13],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for _AppDomain {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _AppDomain as *const IUnknown) }
    }
}
