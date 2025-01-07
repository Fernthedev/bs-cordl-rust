#[cfg(feature = "System+ComponentModel+EventHandlerList")]
#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _head: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventHandlerList_ListEntry,
    >,
    pub _parent: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::Component>,
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::EventHandlerList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "EventHandlerList";
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
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl std::ops::Deref for crate::System::ComponentModel::EventHandlerList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl std::ops::DerefMut for crate::System::ComponentModel::EventHandlerList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl crate::System::ComponentModel::EventHandlerList {
    #[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
    pub type ListEntry = crate::System::ComponentModel::EventHandlerList_ListEntry;
    pub fn Find(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventHandlerList_ListEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::EventHandlerList_ListEntry,
        > = __cordl_object.invoke("Find", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = __cordl_object
            .invoke("get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EventHandlerList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct EventHandlerList_ListEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::EventHandlerList_ListEntry,
    >,
    pub _key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _handler: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::EventHandlerList_ListEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ListEntry";
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
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl std::ops::Deref for crate::System::ComponentModel::EventHandlerList_ListEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl std::ops::DerefMut for crate::System::ComponentModel::EventHandlerList_ListEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl crate::System::ComponentModel::EventHandlerList_ListEntry {}
#[cfg(feature = "System+ComponentModel+EventHandlerList+ListEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EventHandlerList_ListEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
