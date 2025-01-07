#[cfg(feature = "TimelinePauseReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelinePauseReceiver {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub timelinePauseEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "TimelinePauseReceiver")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TimelinePauseReceiver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TimelinePauseReceiver";
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
#[cfg(feature = "TimelinePauseReceiver")]
impl std::ops::Deref for crate::GlobalNamespace::TimelinePauseReceiver {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelinePauseReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl crate::GlobalNamespace::TimelinePauseReceiver {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnNotify(
        &mut self,
        origin: crate::UnityEngine::Playables::Playable,
        notification: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::INotification,
        >,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNotify", (origin, notification, context))?;
        Ok(__cordl_ret.into())
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
    pub fn add_timelinePauseEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_timelinePauseEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_timelinePauseEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_timelinePauseEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TimelinePauseReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl AsRef<crate::UnityEngine::Playables::INotificationReceiver>
for crate::GlobalNamespace::TimelinePauseReceiver {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::INotificationReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TimelinePauseReceiver")]
impl AsMut<crate::UnityEngine::Playables::INotificationReceiver>
for crate::GlobalNamespace::TimelinePauseReceiver {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::INotificationReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
