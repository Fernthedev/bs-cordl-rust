#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectInstaller {
    __cordl_parent: crate::Zenject::ScriptableObjectInstallerBase,
}
#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScriptableObjectInstaller => "Zenject"
    ."ScriptableObjectInstaller"
);
#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
impl std::ops::Deref for crate::Zenject::ScriptableObjectInstaller {
    type Target = crate::Zenject::ScriptableObjectInstallerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
impl std::ops::DerefMut for crate::Zenject::ScriptableObjectInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
impl crate::Zenject::ScriptableObjectInstaller {
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
#[cfg(feature = "Zenject+ScriptableObjectInstaller")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ScriptableObjectInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
