use crate::primitives::{
    itype::_Type, IUnknown, IUnknownVtbl, Interface, _MethodInfo, wrap_method_arguments,
    wrap_strings_in_array, GUID, HRESULT,
};
use std::{
    ffi::{c_long, c_void},
    ops::Deref,
    ptr,
};
use windows::{
    core::BSTR,
    Win32::System::{
        Com::{SAFEARRAY, VARIANT, VT_UNKNOWN},
        Ole::{SafeArrayCreateVector, SafeArrayGetElement, SafeArrayGetUBound},
    },
};

#[repr(C)]
pub struct _Assembly {
    pub vtable: *const _AssemblyVtbl,
}

#[repr(C)]
pub struct _AssemblyVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut c_long) -> HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_CodeBase:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_EscapedCodeBase:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut c_void) -> HRESULT,
    pub GetName_2: *const c_void,
    pub get_FullName:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_EntryPoint:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _MethodInfo) -> HRESULT,
    pub GetType_2: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    pub GetType_3: *const c_void,
    pub GetExportedTypes: *const c_void,
    pub GetTypes:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetManifestResourceStream: *const c_void,
    pub GetManifestResourceStream_2: *const c_void,
    pub GetFile: *const c_void,
    pub GetFiles: *const c_void,
    pub GetFiles_2: *const c_void,
    pub GetManifestResourceNames: *const c_void,
    pub GetManifestResourceInfo: *const c_void,
    pub get_Location:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_Evidence: *const c_void,
    pub GetCustomAttributes: *const c_void,
    pub GetCustomAttributes_2: *const c_void,
    pub IsDefined: *const c_void,
    pub GetObjectData: *const c_void,
    pub add_ModuleResolve: *const c_void,
    pub remove_ModuleResolve: *const c_void,
    pub GetType_4: *const c_void,
    pub GetSatelliteAssembly: *const c_void,
    pub GetSatelliteAssembly_2: *const c_void,
    pub LoadModule: *const c_void,
    pub LoadModule_2: *const c_void,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut c_void,
        typeName: *mut u16,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    pub CreateInstance_2: *const c_void,
    pub CreateInstance_3: *const c_void,
    pub GetLoadedModules: *const c_void,
    pub GetLoadedModules_2: *const c_void,
    pub GetModules: *const c_void,
    pub GetModules_2: *const c_void,
    pub GetModule: *const c_void,
    pub GetReferencedAssemblies: *const c_void,
    pub get_GlobalAssemblyCache: *const c_void,
}

impl _Assembly {
    pub fn run_entrypoint(&self, args: &[String]) -> Result<VARIANT, String> {
        let entrypoint = (*self).get_entrypoint()?;
        let signature = unsafe { (*entrypoint).to_string()? };

        if signature.ends_with("Main()") {
            return unsafe { (*entrypoint).invoke_without_args(None) };
        }

        if signature.ends_with("Main(System.String[])") {
            let args_variant = wrap_strings_in_array(args)?;
            let method_args = wrap_method_arguments(vec![args_variant])?;

            return unsafe { (*entrypoint).invoke(method_args, None) };
        }

        Err(format!(
            "Cannot handle an entrypoint with this method signature: {}",
            signature
        ))
    }

    pub fn get_entrypoint(&self) -> Result<*mut _MethodInfo, String> {
        let mut method_info_ptr: *mut _MethodInfo = ptr::null_mut();

        let hr = unsafe { (*self).get_EntryPoint(&mut method_info_ptr) };

        if hr.is_err() {
            return Err(format!("Could not retrieve entrypoint: {:?}", hr));
        }

        if method_info_ptr.is_null() {
            return Err("Could not retrieve entrypoint".into());
        }

        Ok(method_info_ptr)
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
    pub unsafe fn get_CodeBase(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_CodeBase)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_EscapedCodeBase(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_EscapedCodeBase)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetName(&self, pRetVal: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).GetName)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_FullName(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_FullName)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_EntryPoint(&self, pRetVal: *mut *mut _MethodInfo) -> HRESULT {
        ((*self.vtable).get_EntryPoint)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetType_2(&self, name: *mut u16, pRetVal: *mut *mut _Type) -> HRESULT {
        ((*self.vtable).GetType_2)(self as *const _ as *mut _, name, pRetVal)
    }

    #[inline]
    pub unsafe fn GetTypes(&self, pRetVal: *mut *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).GetTypes)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_Location(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_Location)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn CreateInstance(&self, typeName: *mut u16, pRetVal: *mut VARIANT) -> HRESULT {
        ((*self.vtable).CreateInstance)(self as *const _ as *mut _, typeName, pRetVal)
    }

    pub fn create_instance(&self, name: &str) -> Result<VARIANT, String> {
        let dw = BSTR::from(name);

        let mut instance: VARIANT = VARIANT::default();
        let hr = unsafe { (*self).CreateInstance(dw.into_raw() as *mut _, &mut instance) };

        if hr.is_err() {
            return Err(format!(
                "Error while creating instance of `{}`: 0x{:x}",
                name, hr.0
            ));
        }

        Ok(instance)
    }

    pub fn get_type(&self, name: &str) -> Result<*mut _Type, String> {
        let dw = BSTR::from(name);

        let mut type_ptr: *mut _Type = ptr::null_mut();
        let hr = unsafe { (*self).GetType_2(dw.into_raw() as *mut _, &mut type_ptr) };

        if hr.is_err() {
            return Err(format!(
                "Error while retrieving type `{}`: 0x{:x}",
                name, hr.0
            ));
        }

        if type_ptr.is_null() {
            return Err(format!("Could not retrieve type `{}`", name));
        }

        Ok(type_ptr)
    }

    pub fn get_types(&self) -> Result<Vec<*mut _Type>, String> {
        let mut results: Vec<*mut _Type> = vec![];

        let mut safe_array_ptr: *mut SAFEARRAY = unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, 0) };

        let hr = unsafe { (*self).GetTypes(&mut safe_array_ptr) };

        if hr.is_err() {
            return Err(format!("Error while retrieving types: 0x{:x}", hr.0));
        }

        let ubound = unsafe { SafeArrayGetUBound(safe_array_ptr, 1) }.unwrap_or(0);

        for i in 0..ubound {
            let indices: [i32; 1] = [i as _];
            let mut variant: *mut _Type = ptr::null_mut();
            let pv = &mut variant as *mut _ as *mut c_void;

            match unsafe { SafeArrayGetElement(safe_array_ptr, indices.as_ptr(), pv) } {
                Ok(_) => {},
                Err(e) => return Err(format!("Could not access safe array: {:?}", e.code())),
            }

            if !pv.is_null() {
                results.push(variant)
            }
        }

        Ok(results)
    }
}

impl Interface for _Assembly {
    const IID: GUID = GUID::from_values(
        0x17156360,
        0x2f1a,
        0x384a,
        [0xbc, 0x52, 0xfd, 0xe9, 0x3c, 0x21, 0x5c, 0x5b],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for _Assembly {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _Assembly as *const IUnknown) }
    }
}
