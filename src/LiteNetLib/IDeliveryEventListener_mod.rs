#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
#[repr(C)]
#[derive(Debug)]
pub struct IDeliveryEventListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::IDeliveryEventListener =>
    "LiteNetLib"."IDeliveryEventListener"
);
#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
impl std::ops::Deref for crate::LiteNetLib::IDeliveryEventListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
impl std::ops::DerefMut for crate::LiteNetLib::IDeliveryEventListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
impl crate::LiteNetLib::IDeliveryEventListener {
    pub fn OnMessageDelivered(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMessageDelivered", (peer, userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "LiteNetLib+IDeliveryEventListener")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::IDeliveryEventListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
