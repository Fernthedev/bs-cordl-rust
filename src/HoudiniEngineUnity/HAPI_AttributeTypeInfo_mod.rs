#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_AttributeTypeInfo {
    #[default]
    HAPI_ATTRIBUTE_TYPE_BOX = 12i32,
    HAPI_ATTRIBUTE_TYPE_BOX2 = 11i32,
    HAPI_ATTRIBUTE_TYPE_COLOR = 5i32,
    HAPI_ATTRIBUTE_TYPE_HIDDEN = 10i32,
    HAPI_ATTRIBUTE_TYPE_HPOINT = 2i32,
    HAPI_ATTRIBUTE_TYPE_INVALID = -1i32,
    HAPI_ATTRIBUTE_TYPE_MATRIX = 8i32,
    HAPI_ATTRIBUTE_TYPE_MATRIX3 = 7i32,
    HAPI_ATTRIBUTE_TYPE_MAX = 14i32,
    HAPI_ATTRIBUTE_TYPE_NONE = 0i32,
    HAPI_ATTRIBUTE_TYPE_NORMAL = 4i32,
    HAPI_ATTRIBUTE_TYPE_POINT = 1i32,
    HAPI_ATTRIBUTE_TYPE_QUATERNION = 6i32,
    HAPI_ATTRIBUTE_TYPE_ST = 9i32,
    HAPI_ATTRIBUTE_TYPE_TEXTURE = 13i32,
    HAPI_ATTRIBUTE_TYPE_VECTOR = 3i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_AttributeTypeInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_AttributeTypeInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HAPI_AttributeTypeInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
