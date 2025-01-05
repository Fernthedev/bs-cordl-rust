#[cfg(feature = "System+MonoListItem")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoListItem {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub next: quest_hook::libil2cpp::Gc<crate::System::MonoListItem>,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+MonoListItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoListItem => "System"."MonoListItem"
);
#[cfg(feature = "System+MonoListItem")]
impl std::ops::Deref for crate::System::MonoListItem {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoListItem")]
impl std::ops::DerefMut for crate::System::MonoListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoListItem")]
impl crate::System::MonoListItem {
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
#[cfg(feature = "System+MonoListItem")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoListItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
