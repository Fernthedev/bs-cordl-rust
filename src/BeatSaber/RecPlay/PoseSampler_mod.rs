#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseSampler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::RecPlay::PoseSampler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.RecPlay";
    const CLASS_NAME: &'static str = "PoseSampler";
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
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PoseSampler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PoseSampler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl crate::BeatSaber::RecPlay::PoseSampler {
    pub fn FindPoseSample(
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
        _cordl_time: f32,
        nearest: i32,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::RecPlay::FrameSample> {
        let __cordl_ret: crate::BeatSaber::RecPlay::FrameSample = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindPoseSample", (frames, _cordl_time, nearest))?;
        Ok(__cordl_ret.into())
    }
    pub fn InterpolatePoseSample(
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
        sample: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::RecPlay::FrameSample>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InterpolatePoseSample", (frames, sample))?;
        Ok(__cordl_ret.into())
    }
    pub fn SamplePose(
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
        _cordl_time: f32,
        nearest: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SamplePose", (frames, _cordl_time, nearest))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseSampler")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::PoseSampler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
