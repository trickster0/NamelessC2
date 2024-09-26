use crate::primitives::{
    IUnknown, IUnknownVtbl, Interface, _Assembly, _ConstructorInfo, _MethodInfo, _PropertyInfo,
    empty_variant_array, wrap_method_arguments, GUID, HRESULT,
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
pub struct _Type {
    pub vtable: *const _TypeVtbl,
}

#[repr(C)]
pub struct _TypeVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount:
        unsafe extern "system" fn(this: *mut c_void, pctinfo: *mut u32) -> HRESULT,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut c_long) -> HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_MemberType: *const c_void,
    pub get_Name: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_DeclaringType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub get_ReflectedType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub GetCustomAttributes: *const c_void,
    pub GetCustomAttributes_2: *const c_void,
    pub IsDefined: *const c_void,
    pub get_Guid: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut GUID) -> HRESULT,
    pub get_Module: *const c_void,
    pub get_Assembly:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Assembly) -> HRESULT,
    pub get_TypeHandle: *const c_void,
    pub get_FullName:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_Namespace:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub get_AssemblyQualifiedName:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub GetArrayRank: *const c_void,
    pub get_BaseType:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut _Type) -> HRESULT,
    pub GetConstructors: unsafe extern "system" fn(
        this: *mut c_void,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub GetInterface: *const c_void,
    pub GetInterfaces: *const c_void,
    pub FindInterfaces: *const c_void,
    pub GetEvent: *const c_void,
    pub GetEvents: *const c_void,
    pub GetEvents_2: *const c_void,
    pub GetNestedTypes: *const c_void,
    pub GetNestedType: *const c_void,
    pub GetMember: *const c_void,
    pub GetDefaultMembers: *const c_void,
    pub FindMembers: *const c_void,
    pub GetElementType: *const c_void,
    pub IsSubclassOf: *const c_void,
    pub IsInstanceOfType: *const c_void,
    pub IsAssignableFrom: *const c_void,
    pub GetInterfaceMap: *const c_void,
    pub GetMethod: *const c_void,
    pub GetMethod_2: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetMethods: unsafe extern "system" fn(
        this: *mut c_void,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub GetField: *const c_void,
    pub GetFields: *const c_void,
    pub GetProperty: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    pub GetProperty_2: *const c_void,
    pub GetProperties: unsafe extern "system" fn(
        this: *mut c_void,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    pub GetMember_2: *const c_void,
    pub GetMembers: *const c_void,
    pub InvokeMember: *const c_void,
    pub get_UnderlyingSystemType: *const c_void,
    pub InvokeMember_2: *const c_void,
    pub InvokeMember_3: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        invokeAttr: BindingFlags,
        Binder: *mut c_void,
        Target: VARIANT,
        args: *mut SAFEARRAY,
        namedParameters: *mut SAFEARRAY,
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
    pub GetConstructor: *const c_void,
    pub GetConstructor_2: *const c_void,
    pub GetConstructor_3: unsafe extern "system" fn(
        this: *mut c_void,
        types: *mut SAFEARRAY,
        pRetVal: *mut *mut _ConstructorInfo,
    ) -> HRESULT,
    pub GetConstructors_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub get_TypeInitializer: *const c_void,
    pub GetMethod_3: *const c_void,
    pub GetMethod_4: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        types: *mut SAFEARRAY,
        modifiers: *mut SAFEARRAY,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetMethod_5: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        types: *mut SAFEARRAY,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetMethod_6: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    pub GetMethods_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetField_2: *const c_void,
    pub GetFields_2: *const c_void,
    pub GetInterface_2: *const c_void,
    pub GetEvent_2: *const c_void,
    pub GetProperty_3: *const c_void,
    pub GetProperty_4: *const c_void,
    pub GetProperty_5: *const c_void,
    pub GetProperty_6: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        returnType: *mut _Type,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    pub GetProperty_7: unsafe extern "system" fn(
        this: *mut c_void,
        name: *mut u16,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    pub GetProperties_2:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetNestedTypes_2: *const c_void,
    pub GetNestedType_2: *const c_void,
    pub GetMember_3: *const c_void,
    pub GetMembers_2: *const c_void,
    pub get_Attributes: *const c_void,
    pub get_IsNotPublic: *const c_void,
    pub get_IsPublic: *const c_void,
    pub get_IsNestedPublic: *const c_void,
    pub get_IsNestedPrivate: *const c_void,
    pub get_IsNestedFamily: *const c_void,
    pub get_IsNestedAssembly: *const c_void,
    pub get_IsNestedFamANDAssem: *const c_void,
    pub get_IsNestedFamORAssem: *const c_void,
    pub get_IsAutoLayout: *const c_void,
    pub get_IsLayoutSequential: *const c_void,
    pub get_IsExplicitLayout: *const c_void,
    pub get_IsClass: *const c_void,
    pub get_IsInterface: *const c_void,
    pub get_IsValueType: *const c_void,
    pub get_IsAbstract: *const c_void,
    pub get_IsSealed: *const c_void,
    pub get_IsEnum: *const c_void,
    pub get_IsSpecialName: *const c_void,
    pub get_IsImport: *const c_void,
    pub get_IsSerializable: *const c_void,
    pub get_IsAnsiClass: *const c_void,
    pub get_IsUnicodeClass: *const c_void,
    pub get_IsAutoClass: *const c_void,
    pub get_IsArray: *const c_void,
    pub get_IsByRef: *const c_void,
    pub get_IsPointer: *const c_void,
    pub get_IsPrimitive: *const c_void,
    pub get_IsCOMObject: *const c_void,
    pub get_HasElementType: *const c_void,
    pub get_IsContextful: *const c_void,
    pub get_IsMarshalByRef: *const c_void,
    pub Equals_2: *const c_void,
}

impl _Type {
    pub fn to_string(&self) -> Result<String, String> {
        let mut buffer = BSTR::new();

        let hr = unsafe { (*self).ToString(&mut buffer as *mut _ as *mut *mut u16) };

        if hr.is_err() {
            return Err(format!("Failed while running `ToString`: {:?}", hr));
        }

        Ok(buffer.to_string())
    }

    pub fn get_constructor(
        &self,
        parameter_types: Vec<VARIANT>,
    ) -> Result<*mut _ConstructorInfo, String> {
        let mut constructor_ptr: *mut _ConstructorInfo = ptr::null_mut();
        let type_array = wrap_method_arguments(parameter_types)?;
        let hr = unsafe { (*self).GetConstructor_3(type_array, &mut constructor_ptr) };

        if hr.is_err() {
            return Err(format!("Error while retrieving constructor: 0x{:x}", hr.0));
        }

        if constructor_ptr.is_null() {
            return Err("Could not retrieve constructor".into());
        }

        Ok(constructor_ptr)
    }

    pub fn get_constructor_with_signature(
        &self,
        signature: &str,
    ) -> Result<*mut _ConstructorInfo, String> {
        let constructors = self.get_constructors()?;

        for constructor in constructors {
            let constructor_name = unsafe { (*constructor).to_string()? };

            if &constructor_name == signature {
                return Ok(constructor);
            }
        }

        Err(format!(
            "Could not find a constructor with the given signature: {}",
            signature
        ))
    }

    pub fn get_constructors(&self) -> Result<Vec<*mut _ConstructorInfo>, String> {
        let mut results: Vec<*mut _ConstructorInfo> = vec![];

        let mut safe_array_ptr: *mut SAFEARRAY =
            unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, 255) };

        let hr = unsafe { (*self).GetConstructors_2(&mut safe_array_ptr) };

        if hr.is_err() {
            return Err(format!("Error while retrieving constructors: 0x{:x}", hr.0));
        }

        let ubound = unsafe { SafeArrayGetUBound(safe_array_ptr, 1) }.unwrap_or(0);

        for i in 0..ubound {
            let indices: [i32; 1] = [i as _];
            let mut variant: *mut _ConstructorInfo = ptr::null_mut();
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

    pub fn get_method(&self, name: &str) -> Result<*mut _MethodInfo, String> {
        let dw = BSTR::from(name);

        let mut method_ptr: *mut _MethodInfo = ptr::null_mut();
        let hr = unsafe { (*self).GetMethod_6(dw.into_raw() as *mut _, &mut method_ptr) };

        if hr.is_err() {
            return Err(format!(
                "Error while retrieving method `{}`: 0x{:x}",
                name, hr.0
            ));
        }

        if method_ptr.is_null() {
            return Err(format!("Could not retrieve method `{}`", name));
        }

        Ok(method_ptr)
    }

    pub fn get_method_with_signature(&self, signature: &str) -> Result<*mut _MethodInfo, String> {
        let methods = self.get_methods()?;

        for method in methods {
            let method_name = unsafe { (*method).to_string()? };

            if &method_name == signature {
                return Ok(method);
            }
        }

        Err(format!(
            "Could not find a method with the given signature: {}",
            signature
        ))
    }

    pub fn get_methods(&self) -> Result<Vec<*mut _MethodInfo>, String> {
        let mut results: Vec<*mut _MethodInfo> = vec![];

        let mut safe_array_ptr: *mut SAFEARRAY =
            unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, 255) };

        let hr = unsafe { (*self).GetMethods_2(&mut safe_array_ptr) };

        if hr.is_err() {
            return Err(format!("Error while retrieving methods: 0x{:x}", hr.0));
        }

        let ubound = unsafe { SafeArrayGetUBound(safe_array_ptr, 1) }.unwrap_or(0);

        for i in 0..ubound {
            let indices: [i32; 1] = [i as _];
            let mut variant: *mut _MethodInfo = ptr::null_mut();
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

    pub fn get_property(&self, name: &str) -> Result<*mut _PropertyInfo, String> {
        let dw = BSTR::from(name);

        let mut property_ptr: *mut _PropertyInfo = ptr::null_mut();
        let hr = unsafe { (*self).GetProperty_7(dw.into_raw() as *mut _, &mut property_ptr) };

        if hr.is_err() {
            return Err(format!(
                "Error while retrieving method `{}`: 0x{:x}",
                name, hr.0
            ));
        }

        if property_ptr.is_null() {
            return Err(format!("Could not retrieve method `{}`", name));
        }

        Ok(property_ptr)
    }

    pub fn get_properties(&self) -> Result<Vec<*mut _PropertyInfo>, String> {
        let mut results: Vec<*mut _PropertyInfo> = vec![];

        let mut safe_array_ptr: *mut SAFEARRAY =
            unsafe { SafeArrayCreateVector(VT_UNKNOWN, 0, 255) };

        let hr = unsafe { (*self).GetProperties_2(&mut safe_array_ptr) };

        if hr.is_err() {
            return Err(format!("Error while retrieving methods: 0x{:x}", hr.0));
        }

        let ubound = unsafe { SafeArrayGetUBound(safe_array_ptr, 1) }.unwrap_or(0);

        for i in 0..ubound {
            let indices: [i32; 1] = [i as _];
            let mut variant: *mut _PropertyInfo = ptr::null_mut();
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

    pub fn invoke_static_method(
        &self,
        instance: VARIANT,
        method: String,
        args: *mut SAFEARRAY,
    ) -> Result<VARIANT, String> {
        self.invoke_member(
            instance.clone(),
            method,
            BINDING_FLAGS_PUBLIC | BINDING_FLAGS_STATIC | BINDING_FLAGS_INVOKE_METHOD,
            args,
        )
    }

    pub fn invoke_instance_method(
        &self,
        instance: VARIANT,
        method: String,
        args: *mut SAFEARRAY,
    ) -> Result<VARIANT, String> {
        self.invoke_member(
            instance.clone(),
            method,
            BINDING_FLAGS_PUBLIC | BINDING_FLAGS_INVOKE_METHOD,
            args,
        )
    }

    pub fn invoke_member(
        &self,
        instance: VARIANT,
        method: String,
        flags: BindingFlags,
        args: *mut SAFEARRAY,
    ) -> Result<VARIANT, String> {
        let method_name = BSTR::from(method.clone());
        let binder: *mut c_void = ptr::null_mut();
        let named_params = empty_variant_array();
        let mut return_ptr: *mut VARIANT = ptr::null_mut();

        let hr = unsafe {
            ((*self.vtable).InvokeMember_3)(
                self as *const _ as *mut _,
                method_name.into_raw() as *mut _,
                flags,
                binder,
                instance,
                args,
                named_params,
                &mut return_ptr,
            )
        };

        if hr.is_err() {
            return Err(format!(
                "Error while invoking method `{}`: 0x{:x}",
                method, hr.0
            ));
        }

        if return_ptr.is_null() {
            return Ok(VARIANT::default());
        }

        Ok(unsafe { (*return_ptr).clone() })
    }

    #[inline]
    pub unsafe fn GetTypeInfoCount(&self, pctinfo: *mut u32) -> HRESULT {
        ((*self.vtable).GetTypeInfoCount)(self as *const _ as *mut _, pctinfo)
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
    pub unsafe fn get_Name(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_Name)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_DeclaringType(&self, pRetVal: *mut *mut _Type) -> HRESULT {
        ((*self.vtable).get_DeclaringType)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_ReflectedType(&self, pRetVal: *mut *mut _Type) -> HRESULT {
        ((*self.vtable).get_ReflectedType)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_Guid(&self, pRetVal: *mut *mut GUID) -> HRESULT {
        ((*self.vtable).get_Guid)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_Assembly(&self, pRetVal: *mut *mut _Assembly) -> HRESULT {
        ((*self.vtable).get_Assembly)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_FullName(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_FullName)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_Namespace(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_Namespace)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_AssemblyQualifiedName(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).get_AssemblyQualifiedName)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn get_BaseType(&self, pRetVal: *mut *mut _Type) -> HRESULT {
        ((*self.vtable).get_BaseType)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethod_2(
        &self,
        name: *mut u16,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT {
        ((*self.vtable).GetMethod_2)(self as *const _ as *mut _, name, bindingAttr, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethods(
        &self,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT {
        ((*self.vtable).GetMethods)(self as *const _ as *mut _, bindingAttr, pRetVal)
    }

    #[inline]
    pub unsafe fn GetProperty(
        &self,
        name: *mut u16,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT {
        ((*self.vtable).GetProperty)(self as *const _ as *mut _, name, bindingAttr, pRetVal)
    }

    #[inline]
    pub unsafe fn GetProperties(
        &self,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT {
        ((*self.vtable).GetProperties)(self as *const _ as *mut _, bindingAttr, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethod_4(
        &self,
        name: *mut u16,
        types: *mut SAFEARRAY,
        modifiers: *mut SAFEARRAY,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT {
        ((*self.vtable).GetMethod_4)(self as *const _ as *mut _, name, types, modifiers, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethod_5(
        &self,
        name: *mut u16,
        types: *mut SAFEARRAY,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT {
        ((*self.vtable).GetMethod_5)(self as *const _ as *mut _, name, types, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethod_6(&self, name: *mut u16, pRetVal: *mut *mut _MethodInfo) -> HRESULT {
        ((*self.vtable).GetMethod_6)(self as *const _ as *mut _, name, pRetVal)
    }

    #[inline]
    pub unsafe fn GetMethods_2(&self, pRetVal: *mut *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).GetMethods_2)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetProperty_6(
        &self,
        name: *mut u16,
        returnType: *mut _Type,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT {
        ((*self.vtable).GetProperty_6)(self as *const _ as *mut _, name, returnType, pRetVal)
    }

    #[inline]
    pub unsafe fn GetProperty_7(
        &self,
        name: *mut u16,
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT {
        ((*self.vtable).GetProperty_7)(self as *const _ as *mut _, name, pRetVal)
    }

    #[inline]
    pub unsafe fn GetProperties_2(&self, pRetVal: *mut *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).GetProperties_2)(self as *const _ as *mut _, pRetVal)
    }

    #[inline]
    pub unsafe fn GetConstructor_3(
        &self,
        types: *mut SAFEARRAY,
        pRetVal: *mut *mut _ConstructorInfo,
    ) -> HRESULT {
        ((*self.vtable).GetConstructor_3)(self as *const _ as *mut _, types, pRetVal)
    }

    #[inline]
    pub unsafe fn GetConstructors_2(&self, pRetVal: *mut *mut SAFEARRAY) -> HRESULT {
        ((*self.vtable).GetConstructors_2)(self as *const _ as *mut _, pRetVal)
    }
}

impl Deref for _Type {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _Type as *const IUnknown) }
    }
}
impl Interface for _Type {
    const IID: GUID = GUID::from_values(
        0xbca8b44d,
        0xaad6,
        0x3a86,
        [0x8a, 0xb7, 0x03, 0x34, 0x9f, 0x4f, 0x2d, 0xa2],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

pub type BindingFlags = u32;
pub const BINDING_FLAGS_DEFAULT: BindingFlags = 0;
pub const BINDING_FLAGS_IGNORE_CASE: BindingFlags = 1;
pub const BINDING_FLAGS_DECLARED_ONLY: BindingFlags = 2;
pub const BINDING_FLAGS_INSTANCE: BindingFlags = 4;
pub const BINDING_FLAGS_STATIC: BindingFlags = 8;
pub const BINDING_FLAGS_PUBLIC: BindingFlags = 16;
pub const BINDING_FLAGS_NON_PUBLIC: BindingFlags = 32;
pub const BINDING_FLAGS_FLATTEN_HIERARCHY: BindingFlags = 64;
pub const BINDING_FLAGS_INVOKE_METHOD: BindingFlags = 256;
pub const BINDING_FLAGS_CREATE_INSTANCE: BindingFlags = 512;
pub const BINDING_FLAGS_GET_FIELD: BindingFlags = 1024;
pub const BINDING_FLAGS_SET_FIELD: BindingFlags = 2048;
pub const BINDING_FLAGS_GET_PROPERTY: BindingFlags = 4096;
pub const BINDING_FLAGS_SET_PROPERTY: BindingFlags = 8192;
pub const BINDING_FLAGS_PUT_DISP_PROPERTY: BindingFlags = 16384;
pub const BINDING_FLAGS_PUT_REF_DISP_PROPERTY: BindingFlags = 32768;
pub const BINDING_FLAGS_EXACT_BINDING: BindingFlags = 65536;
pub const BINDING_FLAGS_SUPPRESS_CHANGE_TYPE: BindingFlags = 131072;
pub const BINDING_FLAGS_OPTIONAL_PARAM_BINDING: BindingFlags = 262144;
pub const BINDING_FLAGS_IGNORE_RETURN: BindingFlags = 16777216;

pub type MemberTypes = u32;
pub const MEMBER_TYPES_CONSTRUCTOR: MemberTypes = 1;
pub const MEMBER_TYPES_EVENT: MemberTypes = 2;
pub const MEMBER_TYPES_FIELD: MemberTypes = 4;
pub const MEMBER_TYPES_METHOD: MemberTypes = 8;
pub const MEMBER_TYPES_PROPERTY: MemberTypes = 16;
pub const MEMBER_TYPES_TYPE_INFO: MemberTypes = 32;
pub const MEMBER_TYPES_CUSTOM: MemberTypes = 64;
pub const MEMBER_TYPES_NESTED_TYPE: MemberTypes = 128;
pub const MEMBER_TYPES_ALL: MemberTypes = 191;
