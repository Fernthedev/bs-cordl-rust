#[cfg(feature = "System+AppContextDefaultValues")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContextDefaultValues {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+AppContextDefaultValues")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppContextDefaultValues => "System"
    ."AppContextDefaultValues"
);
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::Deref for crate::System::AppContextDefaultValues {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::DerefMut for crate::System::AppContextDefaultValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl crate::System::AppContextDefaultValues {
    pub fn PopulateDefaultValues() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateDefaultValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSwitchOverride(
        switchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        overrideValue: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSwitchOverride", (switchName, overrideValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppContextDefaultValues {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
