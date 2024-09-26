use crate::primitives::{
    empty_variant_array, get_array_length, itype::_Type, IUnknown, IUnknownVtbl, Interface, GUID,
    HRESULT,
};
use std::{
    ffi::{c_long, c_void},
    ops::Deref,
};
use windows::{
    core::BSTR,
    Win32::System::{
        Com::{SAFEARRAY, VARIANT, VT_UNKNOWN},
        Ole::SafeArrayCreateVector,
    },
};

#[repr(C)]
pub struct _MethodInfo {
    pub vtable: *const _MethodInfoVtbl,
}

#[repr(C)]
pub struct _MethodInfoVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut c_long) -> HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_MemberType: *const c_void,
    pub get_name: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_DeclaringType: *const c_void,
    pub get_ReflectedType: *const c_void,
    pub GetCustomAttributes: *const c_void,
    pub GetCustomAttributes_2: *const c_void,
    pub IsDefined: *const c_void,
    pub GetParameters:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetMethodImplementationFlags: *const c_void,
    pub get_MethodHandle: *const c_void,
    pub get_Attributes: *const c_void,
    pub get_CallingConvention: *const c_void,
    pub Invoke_2: *const c_void,
    pub get_IsPublic: *const c_void,
    pub get_IsPrivate: *const c_void,
    pub get_IsFamily: *const c_void,
    pub get_IsAssembly: *const c_void,
    pub get_IsFamilyAndAssembly: *const c_void,
    pub get_IsFamilyOrAssembly: *const c_void,
    pub get_IsStatic: *const c_void,
    pub get_IsFinal: *const c_void,
    pub get_IsVirtual: *const c_void,
    pub get_IsHideBySig: *const c_void,
    pub get_IsAbstract: *const c_void,
    pub get_IsSpecialName: *const c_void,
    pub get_IsConstructor: *const c_void,
    pub Invoke_3: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        parameters: *mut SAFEARRAY,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    pub get_returnType: *const c_void,
    pub get_ReturnTypeCustomAttributes: *const c_void,
    pub GetBaseDefinition:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _MethodInfo) -> HRESULT,
}

impl _MethodInfo {
    pub fn invoke(
        &self,
        args: *mut SAFEARRAY,
        instance: Option<VARIANT>,
    ) -> Result<VARIANT, String> {
        let args_len = get_array_length(args);
        let parameter_count = (*self).get_parameter_count()?;

        if args_len != parameter_count {
            return Err(format!(
                "Arguments do not match method signature: {} given, {} expected",
                args_len, parameter_count
            ));
        }

        let mut return_value: VARIANT = unsafe { std::mem::zeroed() };

        let object: VARIANT = match instance {
            None => unsafe { std::mem::zeroed() },
            Some(i) => i,
        };

        let hr = unsafe { (*self).Invoke_3(object, args, &mut return_value) };

        if hr.is_err() {
            return Err(format!("Could not invoke method: {:?}", hr));
        }

        Ok(return_value)
    }

    pub fn invoke_without_args(&self, instance: Option<VARIANT>) -> Result<VARIANT, String> {
        let method_args = empty_variant_array();

        (*self).invoke(method_args, instance)
    }

    pub fn get_parameter_count(&self) -> Result<i32, String> {
        let mut safe_array_ptr: *mut SAFEARRAY =
            unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, 255) };

        let hr = unsafe { (*self).GetParameters(&mut safe_array_ptr) };

        if hr.is_err() {
            return Err(format!("Could not get parameter count: {:?}", hr));
        }

        Ok(get_array_length(safe_array_ptr))
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
    pub unsafe fn get_name(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_name)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetParameters(&self, pRetVal: *mut *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).GetParameters)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn Invoke_3(
        &self,
        obj: VARIANT,
        parameters: *mut SAFEARRAY,
        pRetVal: *mut VARIANT,
    ) -> HRESULT {
        ((*self.vtable).Invoke_3)(self as *const _ as *mut _, obj, parameters, pRetVal)
    }

    #[inline]
    pub unsafe fn GetBaseDefinition(&self, pRetVal: *mut *mut _MethodInfo) -> HRESULT {
        ((*self.vtable).GetBaseDefinition)(self as *const _ as *mut _, pRetVal)
    }
}

impl Interface for _MethodInfo {
    const IID: GUID = GUID::from_values(
        0xffcc1b5d,
        0xecb8,
        0x38dd,
        [0x9b, 0x01, 0x3d, 0xc8, 0xab, 0xc2, 0xaa, 0x5f],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for _MethodInfo {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _MethodInfo as *const IUnknown) }
    }
}
