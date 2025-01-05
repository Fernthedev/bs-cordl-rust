#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxEventBoxGroupConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::FloatFxEventBoxGroupConverter =>
    "BeatmapDataLoaderVersion4"."FloatFxEventBoxGroupConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion4::FloatFxEventBoxGroupConverter {
    type Target = quest_hook::libil2cpp::Gc<
        crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::FloatFxEventBoxGroupConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
impl crate::BeatmapDataLoaderVersion4::FloatFxEventBoxGroupConverter {
    pub fn ConvertEvents(
        &mut self,
        eventBox: crate::BeatmapSaveDataVersion4::EventBox,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBox>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBox,
        > = __cordl_object.invoke("ConvertEvents", (eventBox, indexFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightshowSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion4::LightshowSaveData,
        >,
        lightGroups: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentLightGroups,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxEventBoxGroupConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::FloatFxEventBoxGroupConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
