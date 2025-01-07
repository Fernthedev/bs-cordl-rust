#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBaseDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "FloatFxBaseDataConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    pub fn Convert(
        beat: f32,
        floatFxEvent: crate::BeatmapSaveDataVersion4::FloatFxEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBaseData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (beat, floatFxEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
