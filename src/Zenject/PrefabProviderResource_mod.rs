#[cfg(feature = "Zenject+PrefabProviderResource")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabProviderResource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::PrefabProviderResource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "PrefabProviderResource";
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
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl std::ops::Deref for crate::Zenject::PrefabProviderResource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl std::ops::DerefMut for crate::Zenject::PrefabProviderResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl crate::Zenject::PrefabProviderResource {
    pub fn GetPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("GetPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourcePath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabProviderResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl AsRef<crate::Zenject::IPrefabProvider> for crate::Zenject::PrefabProviderResource {
    fn as_ref(&self) -> &crate::Zenject::IPrefabProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabProviderResource")]
impl AsMut<crate::Zenject::IPrefabProvider> for crate::Zenject::PrefabProviderResource {
    fn as_mut(&mut self) -> &mut crate::Zenject::IPrefabProvider {
        unsafe { std::mem::transmute(self) }
    }
}
