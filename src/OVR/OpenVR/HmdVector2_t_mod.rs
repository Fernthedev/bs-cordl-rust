#[cfg(feature = "OVR+OpenVR+HmdVector2_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HmdVector2_t {
    pub v0: f32,
    pub v1: f32,
}
#[cfg(feature = "OVR+OpenVR+HmdVector2_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::HmdVector2_t => "OVR.OpenVR"
    ."HmdVector2_t"
);
#[cfg(feature = "OVR+OpenVR+HmdVector2_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::HmdVector2_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+HmdVector2_t")]
impl crate::OVR::OpenVR::HmdVector2_t {}
