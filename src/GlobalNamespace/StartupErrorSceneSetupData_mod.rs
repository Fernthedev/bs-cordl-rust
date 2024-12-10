#[cfg(feature = "StartupErrorSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct StartupErrorSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub title: *mut quest_hook::libil2cpp::Il2CppString,
    pub subtitle: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "StartupErrorSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StartupErrorSceneSetupData =>
    ""."StartupErrorSceneSetupData"
);
#[cfg(feature = "StartupErrorSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::StartupErrorSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StartupErrorSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StartupErrorSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StartupErrorSceneSetupData")]
impl crate::GlobalNamespace::StartupErrorSceneSetupData {
    pub fn New(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (title, subtitle))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subtitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (title, subtitle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StartupErrorSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StartupErrorSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
