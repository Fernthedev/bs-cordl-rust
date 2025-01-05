#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSyncSessionsChangedNotification {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub ConnectionId: i64,
    pub Sessions: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::NetSyncSessionList,
    >,
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Models::NetSyncSessionsChangedNotification =>
    "Oculus.Platform.Models"."NetSyncSessionsChangedNotification"
);
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl std::ops::Deref
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl std::ops::DerefMut
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
