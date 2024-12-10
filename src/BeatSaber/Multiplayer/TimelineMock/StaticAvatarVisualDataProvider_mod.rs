#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticAvatarVisualDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Multiplayer::TimelineMock::StaticAvatarVisualDataProvider =>
    "BeatSaber.Multiplayer.TimelineMock"."StaticAvatarVisualDataProvider"
);
#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::Multiplayer::TimelineMock::StaticAvatarVisualDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::Multiplayer::TimelineMock::StaticAvatarVisualDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
impl crate::BeatSaber::Multiplayer::TimelineMock::StaticAvatarVisualDataProvider {
    pub fn New(
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (avatarsData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (avatarsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = __cordl_object
            .invoke("get_avatarsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Multiplayer+TimelineMock+StaticAvatarVisualDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Multiplayer::TimelineMock::StaticAvatarVisualDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
