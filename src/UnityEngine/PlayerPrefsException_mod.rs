#[cfg(feature = "UnityEngine+PlayerPrefsException")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPrefsException {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Exception>,
}
#[cfg(feature = "UnityEngine+PlayerPrefsException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayerPrefsException =>
    "UnityEngine"."PlayerPrefsException"
);
#[cfg(feature = "UnityEngine+PlayerPrefsException")]
impl std::ops::Deref for crate::UnityEngine::PlayerPrefsException {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Exception>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PlayerPrefsException")]
impl std::ops::DerefMut for crate::UnityEngine::PlayerPrefsException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PlayerPrefsException")]
impl crate::UnityEngine::PlayerPrefsException {
    pub fn New(
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (error))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (error))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PlayerPrefsException")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PlayerPrefsException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
