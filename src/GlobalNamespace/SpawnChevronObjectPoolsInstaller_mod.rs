#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnChevronObjectPoolsInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _spawnRotationChevronPrefab: *mut crate::GlobalNamespace::SpawnRotationChevron,
}
#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpawnChevronObjectPoolsInstaller => ""
    ."SpawnChevronObjectPoolsInstaller"
);
#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::SpawnChevronObjectPoolsInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::SpawnChevronObjectPoolsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
impl crate::GlobalNamespace::SpawnChevronObjectPoolsInstaller {
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
#[cfg(feature = "SpawnChevronObjectPoolsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SpawnChevronObjectPoolsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
