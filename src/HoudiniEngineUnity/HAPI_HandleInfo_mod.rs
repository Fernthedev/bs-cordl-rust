#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HAPI_HandleInfo {
    pub nameSH: i32,
    pub typeNameSH: i32,
    pub bindingsCount: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_HandleInfo =>
    "HoudiniEngineUnity"."HAPI_HandleInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::HAPI_HandleInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_HandleInfo")]
impl crate::HoudiniEngineUnity::HAPI_HandleInfo {}
