#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackingServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Services::TrackingServices {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Services";
    const CLASS_NAME: &'static str = "TrackingServices";
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
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Services::TrackingServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Services::TrackingServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl crate::System::Runtime::Remoting::Services::TrackingServices {
    pub fn NotifyDisconnectedObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyDisconnectedObject", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyMarshaledObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_or: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyMarshaledObject", (obj, _cordl_or))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyUnmarshaledObject(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_or: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ObjRef>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyUnmarshaledObject", (obj, _cordl_or))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Services+TrackingServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Services::TrackingServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
