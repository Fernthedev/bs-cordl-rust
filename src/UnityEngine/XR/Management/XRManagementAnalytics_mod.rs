#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
#[repr(C)]
#[derive(Debug)]
pub struct XRManagementAnalytics {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::Management::XRManagementAnalytics => "UnityEngine.XR.Management"
    ."XRManagementAnalytics"
);
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl std::ops::Deref for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl std::ops::DerefMut for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl crate::UnityEngine::XR::Management::XRManagementAnalytics {
    pub const kEventBuild: &'static str = "xrmanagment_build";
    pub const kMaxEventsPerHour: i32 = 1000i32;
    pub const kMaxNumberOfElements: i32 = 1000i32;
    pub const kVendorKey: &'static str = "unity.xrmanagement";
    #[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
    pub type BuildEvent = crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent;
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::Management::XRManagementAnalytics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XRManagementAnalytics_BuildEvent {
    pub buildGuid: *mut crate::System::String,
    pub buildTarget: *mut crate::System::String,
    pub buildTargetGroup: *mut crate::System::String,
    pub assigned_loaders: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent =>
    "UnityEngine.XR.Management"."XRManagementAnalytics/BuildEvent"
);
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+Management+XRManagementAnalytics+BuildEvent")]
impl crate::UnityEngine::XR::Management::XRManagementAnalytics_BuildEvent {}
