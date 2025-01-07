#[cfg(feature = "EventsTestGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _moveTime: bool,
    pub _spawnTestBox: bool,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _floatValue: f32,
    pub _beatmapEventDataBoxGroupLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapEventDataBoxGroupList,
            >,
        >,
    >,
    pub groupState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    pub _beatmapEventTypeBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::KeyCode,
            crate::GlobalNamespace::BasicBeatmapEventType,
        >,
    >,
    pub _intBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::KeyCode,
            i32,
        >,
    >,
    pub _beatmapValuesBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::KeyCode,
            i32,
        >,
    >,
    pub _floatValuesBindings: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::KeyCode,
            f32,
        >,
    >,
    pub _rotatingLasers: bool,
}
#[cfg(feature = "EventsTestGameplayManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EventsTestGameplayManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EventsTestGameplayManager";
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
#[cfg(feature = "EventsTestGameplayManager")]
impl std::ops::Deref for crate::GlobalNamespace::EventsTestGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::EventsTestGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager")]
impl crate::GlobalNamespace::EventsTestGameplayManager {
    pub const kNumberOfLightGroups: i32 = 20i32;
    #[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
    pub type MockBeatToTimeConverter = crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter;
    pub fn AddEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEventsForLightGroup", (lightGroupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddInstantToggleEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstantToggleEventsForLightGroup", (lightGroupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTestBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTestBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToggleEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
        color: crate::GlobalNamespace::EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToggleEventsForLightGroup", (lightGroupId, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "EventsTestGameplayManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EventsTestGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestGameplayManager_MockBeatToTimeConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bpm: f32,
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockBeatToTimeConverter";
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
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl std::ops::Deref
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    pub fn ConvertBeatToTime(
        &mut self,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ConvertBeatToTime", (beat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpm))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpm))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl AsRef<crate::GlobalNamespace::IBeatToTimeConverter>
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatToTimeConverter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl AsMut<crate::GlobalNamespace::IBeatToTimeConverter>
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatToTimeConverter {
        unsafe { std::mem::transmute(self) }
    }
}
