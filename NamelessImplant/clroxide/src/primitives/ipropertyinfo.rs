use crate::primitives::{
    BindingFlags, IUnknown, IUnknownVtbl, Interface, MemberTypes, _MethodInfo, _Type, empty_array,
    GUID, HRESULT,
};
use std::{
    ffi::{c_long, c_void},
    ops::Deref,
};
use windows::{
    core::BSTR,
    Win32::System::Com::{SAFEARRAY, VARIANT},
};

#[repr(C)]
pub struct _PropertyInfo {
    pub vtable: *const _PropertyInfoVtbl,
}

#[repr(C)]
pub struct _PropertyInfoVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,

    pub Equals:
        unsafe extern "system" fn(this: *mut c_void, other: VARIANT, pRetVal: *mut i16) -> HRESULT,
    pub GetHashCode: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut c_long) -> HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_MemberType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut MemberTypes) -> HRESULT,
    pub get_name: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_DeclaringType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_ReflectedType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub GetCustomAttributes: unsafe extern "system" fn(
        this: *mut c_void,
        attributeType: *mut _Type,
        inherit: i16,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub GetCustomAttributes_2: unsafe extern "system" fn(
        this: *mut c_void,
        inherit: i16,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub IsDefined: unsafe extern "system" fn(
        this: *mut c_void,
        attributeType: *mut _Type,
        inherit: i16,
        pRetVal: *mut i16,
    ) -> HRESULT,
    pub get_PropertyType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        index: *mut SAFEARRAY,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    pub GetValue_2: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        invokeAttr: BindingFlags,
        Binder: *mut c_void,
        index: *mut SAFEARRAY,
        culture: *mut c_void,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        val: VARIANT,
        index: *mut SAFEARRAY,
    ) -> HRESULT,
    pub SetValue_2: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        val: VARIANT,
        invokeAttr: BindingFlags,
        Binder: *mut c_void,
        index: *mut SAFEARRAY,
        culture: *mut c_void,
    ) -> HRESULT,
    pub GetAccessors: unsafe extern "system" fn(
        this: *mut c_void,
        nonPublic: i16,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub GetGetMethod: unsafe extern "system" fn(
        this: *mut c_void,
        nonPublic: i16,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetSetMethod: unsafe extern "system" fn(
        this: *mut c_void,
        nonPublic: i16,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetIndexParameters:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub get_Attributes: unsafe extern "system" fn(
        this: *mut c_void,
        pRetVal: *mut *mut PropertyAttributes,
    ) -> HRESULT,
    pub get_CanRead:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut i16) -> HRESULT,
    pub get_CanWrite:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut i16) -> HRESULT,
    pub GetAccessors_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetGetMethod_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _MethodInfo) -> HRESULT,
    pub GetSetMethod_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _MethodInfo) -> HRESULT,
    pub get_IsSpecialName:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut i16) -> HRESULT,
}

impl _PropertyInfo {
    pub fn to_string(&self) -> Result<String, String> {
        let mut buffer = BSTR::new();

        let hr = unsafe { (*self).ToString(&mut buffer as *mut _ as *mut *mut u16) };

        if hr.is_err() {
            return Err(format!("Failed while running `ToString`: {:?}", hr));
        }

        Ok(buffer.to_string())
    }

    pub fn get_value(&self, instance: Option<VARIANT>) -> Result<VARIANT, String> {
        let mut return_value: VARIANT = unsafe { std::mem::zeroed() };

        let object: VARIANT = match instance {
            None => unsafe { std::mem::zeroed() },
            Some(i) => i,
        };

        let index = empty_array();

        let hr = unsafe { (*self).GetValue(object, index, &mut return_value) };

        if hr.is_err() {
            return Err(format!("Could not invoke method: {:?}", hr));
        }

        Ok(return_value)
    }

    pub fn set_value(&self, value: VARIANT, instance: Option<VARIANT>) -> Result<(), String> {
        let object: VARIANT = match instance {
            None => unsafe { std::mem::zeroed() },
            Some(i) => i,
        };

        let index = empty_array();

        let hr = unsafe { (*self).SetValue(object, value, index) };

        if hr.is_err() {
            return Err(format!("Could not invoke method: {:?}", hr));
        }

        Ok(())
    }

    #[inline]
    pub unsafe fn ToString(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).ToString)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetValue(
        &self,
        obj: VARIANT,
        index: *mut SAFEARRAY,
        pRetVal: *mut VARIANT,
    ) -> HRESULT {
        ((*self.vtable).GetValue)(self as *const _ as *mut _, obj, index, pRetVal)
    }

    #[inline]
    pub unsafe fn SetValue(&self, obj: VARIANT, val: VARIANT, index: *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).SetValue)(self as *const _ as *mut _, obj, val, index)
    }
}

impl Interface for _PropertyInfo {
    const IID: GUID = GUID::from_values(
        0xf59ed4e4,
        0xe68f,
        0x3218,
        [0xbd, 0x77, 0x06, 0x1a, 0xa8, 0x28, 0x24, 0xbf],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for _PropertyInfo {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _PropertyInfo as *const IUnknown) }
    }
}

pub type PropertyAttributes = u32;
pub const PROPERTY_ATTRIBUTES_NONE: PropertyAttributes = 0;
pub const PROPERTY_ATTRIBUTES_SPECIAL_NAME: PropertyAttributes = 512;
pub const PROPERTY_ATTRIBUTES_RESERVED_MASK: PropertyAttributes = 62464;
pub const PROPERTY_ATTRIBUTES_RTSPECIAL_NAME: PropertyAttributes = 1024;
pub const PROPERTY_ATTRIBUTES_HAS_DEFAULT: PropertyAttributes = 4096;
pub const PROPERTY_ATTRIBUTES_RESERVED2: PropertyAttributes = 8192;
pub const PROPERTY_ATTRIBUTES_RESERVED3: PropertyAttributes = 16384;
pub const PROPERTY_ATTRIBUTES_RESERVED4: PropertyAttributes = 32768;
