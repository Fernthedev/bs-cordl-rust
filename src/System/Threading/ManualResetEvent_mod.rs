#[cfg(feature = "System+Threading+ManualResetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ManualResetEvent {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Threading::EventWaitHandle>,
}
#[cfg(feature = "System+Threading+ManualResetEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ManualResetEvent =>
    "System.Threading"."ManualResetEvent"
);
#[cfg(feature = "System+Threading+ManualResetEvent")]
impl std::ops::Deref for crate::System::Threading::ManualResetEvent {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Threading::EventWaitHandle>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ManualResetEvent")]
impl std::ops::DerefMut for crate::System::Threading::ManualResetEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ManualResetEvent")]
impl crate::System::Threading::ManualResetEvent {
    pub fn New(
        initialState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialState))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initialState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialState))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+ManualResetEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ManualResetEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
