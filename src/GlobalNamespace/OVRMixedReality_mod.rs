#[cfg(feature = "OVRMixedReality")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRMixedReality {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRMixedReality")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRMixedReality {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRMixedReality";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OVRMixedReality")]
impl std::ops::Deref for crate::GlobalNamespace::OVRMixedReality {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedReality")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRMixedReality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRMixedReality")]
impl crate::GlobalNamespace::OVRMixedReality {
    pub fn Cleanup() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecenterPose() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecenterPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Update",
                (parentObject, mainCamera, configuration, trackingOrigin),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRMixedReality")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRMixedReality {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
