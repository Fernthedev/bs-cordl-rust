#[cfg(feature = "ComponentExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ComponentExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ComponentExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ComponentExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ComponentExtensions";
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
#[cfg(feature = "ComponentExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ComponentExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ComponentExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ComponentExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ComponentExtensions")]
impl crate::GlobalNamespace::ComponentExtensions {
    pub fn GetComponentInParentOnly<T>(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetComponentInParentOnly", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ComponentExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ComponentExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
