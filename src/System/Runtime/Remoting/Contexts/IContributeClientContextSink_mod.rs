#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
#[repr(C)]
#[derive(Debug)]
pub struct IContributeClientContextSink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::IContributeClientContextSink =>
    "System.Runtime.Remoting.Contexts"."IContributeClientContextSink"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::IContributeClientContextSink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::IContributeClientContextSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
impl crate::System::Runtime::Remoting::Contexts::IContributeClientContextSink {
    pub fn GetClientContextSink(
        &mut self,
        nextSink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object.invoke("GetClientContextSink", (nextSink))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+IContributeClientContextSink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::IContributeClientContextSink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
