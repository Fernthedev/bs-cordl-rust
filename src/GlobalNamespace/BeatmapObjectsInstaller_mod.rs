#[cfg(feature = "BeatmapObjectsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _normalBasicNotePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameNoteController,
    >,
    pub _proModeNotePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameNoteController,
    >,
    pub _burstSliderHeadNotePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameNoteController,
    >,
    pub _burstSliderNotePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BurstSliderGameNoteController,
    >,
    pub _bombNotePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BombNoteController,
    >,
    pub _obstaclePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ObstacleController,
    >,
    pub _sliderShortPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderController,
    >,
    pub _sliderMediumPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderController,
    >,
    pub _sliderLongPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderController,
    >,
    pub _beatLinePrefab: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatLine>,
    pub _sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
}
#[cfg(feature = "BeatmapObjectsInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectsInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectsInstaller";
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
#[cfg(feature = "BeatmapObjectsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl crate::GlobalNamespace::BeatmapObjectsInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
