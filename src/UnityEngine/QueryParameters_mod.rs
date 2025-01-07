#[cfg(feature = "UnityEngine+QueryParameters")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryParameters {
    pub layerMask: i32,
    pub hitMultipleFaces: bool,
    pub hitTriggers: crate::UnityEngine::QueryTriggerInteraction,
    pub hitBackfaces: bool,
}
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::QueryParameters {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "QueryParameters";
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
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::QueryParameters {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::QueryParameters {
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
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::QueryParameters {
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
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::QueryParameters {
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
#[cfg(feature = "UnityEngine+QueryParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::QueryParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+QueryParameters")]
impl crate::UnityEngine::QueryParameters {
    pub fn _ctor(
        &mut self,
        layerMask: i32,
        hitMultipleFaces: bool,
        hitTriggers: crate::UnityEngine::QueryTriggerInteraction,
        hitBackfaces: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (layerMask, hitMultipleFaces, hitTriggers, hitBackfaces),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::QueryParameters,
    > {
        let __cordl_ret: crate::UnityEngine::QueryParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Default", ())?;
        Ok(__cordl_ret.into())
    }
}
