#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
#[repr(C)]
#[derive(Debug)]
pub struct DisposerReplySink {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
    pub _disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::DisposerReplySink {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "DisposerReplySink";
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
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl std::ops::Deref for crate::System::Runtime::Remoting::DisposerReplySink {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::DisposerReplySink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl crate::System::Runtime::Remoting::DisposerReplySink {
    pub fn AsyncProcessMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        replySink: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageCtrl,
        > = __cordl_object.invoke("AsyncProcessMessage", (msg, replySink))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next, disposable))?;
        Ok(__cordl_object.into())
    }
    pub fn SyncProcessMessage(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = __cordl_object.invoke("SyncProcessMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
        disposable: quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (next, disposable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::DisposerReplySink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl AsRef<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::DisposerReplySink {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+DisposerReplySink")]
impl AsMut<crate::System::Runtime::Remoting::Messaging::IMessageSink>
for crate::System::Runtime::Remoting::DisposerReplySink {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Messaging::IMessageSink {
        unsafe { std::mem::transmute(self) }
    }
}
