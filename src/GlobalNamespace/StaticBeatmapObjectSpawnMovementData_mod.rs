#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticBeatmapObjectSpawnMovementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StaticBeatmapObjectSpawnMovementData";
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
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl std::ops::Deref for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    pub const kBaseLinesYPos: f32 = 0.25f32;
    pub const kNoteLinesDistance: f32 = 0.6f32;
    pub const kObstacleVerticalOffset: f32 = -0.15f32;
    pub const kTopLinesYPos: f32 = 1.45f32;
    pub const kUpperLinesYPos: f32 = 0.85f32;
    pub fn Get2DNoteOffset(
        noteLineIndex: i32,
        noteLinesCount: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Get2DNoteOffset", (noteLineIndex, noteLinesCount, noteLineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn LineYPosForLineLayer(
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LineYPosForLineLayer", (lineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layerHeight() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_layerHeight", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
