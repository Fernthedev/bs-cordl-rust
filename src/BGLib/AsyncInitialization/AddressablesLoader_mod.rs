#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AsyncInitialization::AddressablesLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AsyncInitialization";
    const CLASS_NAME: &'static str = "AddressablesLoader";
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
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::Deref for crate::BGLib::AsyncInitialization::AddressablesLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::DerefMut for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl crate::BGLib::AsyncInitialization::AddressablesLoader {
    pub fn InstantiateFromAddressableToContainer<TInstantiate, TReturn>(
        prefab: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TReturn>>,
    >
    where
        TInstantiate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TReturn>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateFromAddressableToContainer", (prefab, container))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
