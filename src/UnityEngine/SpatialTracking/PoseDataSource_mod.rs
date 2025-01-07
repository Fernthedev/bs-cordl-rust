#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SpatialTracking::PoseDataSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SpatialTracking";
    const CLASS_NAME: &'static str = "PoseDataSource";
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
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::Deref for crate::UnityEngine::SpatialTracking::PoseDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::DerefMut for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl crate::UnityEngine::SpatialTracking::PoseDataSource {
    pub fn GetDataFromSource(
        poseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDataFromSource", (poseSource, resultPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePoseData(
        node: crate::UnityEngine::XR::XRNode,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePoseData", (node, resultPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDataFromSource(
        poseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDataFromSource", (poseSource, resultPose))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
