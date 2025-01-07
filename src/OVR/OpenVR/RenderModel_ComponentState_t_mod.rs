#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderModel_ComponentState_t {
    pub mTrackingToComponentRenderModel: crate::OVR::OpenVR::HmdMatrix34_t,
    pub mTrackingToComponentLocal: crate::OVR::OpenVR::HmdMatrix34_t,
    pub uProperties: u32,
}
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "RenderModel_ComponentState_t";
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
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
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
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
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
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
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
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
impl crate::OVR::OpenVR::RenderModel_ComponentState_t {}
