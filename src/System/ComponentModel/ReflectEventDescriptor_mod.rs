#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectEventDescriptor {
    __cordl_parent: crate::System::ComponentModel::EventDescriptor,
    pub _type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _addMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub _removeMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub _realEvent: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
    pub _filledMethods: bool,
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::ReflectEventDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel";
    const CLASS_NAME: &'static str = "ReflectEventDescriptor";
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
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl std::ops::Deref for crate::System::ComponentModel::ReflectEventDescriptor {
    type Target = crate::System::ComponentModel::EventDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl std::ops::DerefMut for crate::System::ComponentModel::ReflectEventDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl crate::System::ComponentModel::ReflectEventDescriptor {
    pub fn FillAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillEventInfoAttribute(
        &mut self,
        realEventInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillEventInfoAttribute", (realEventInfo, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillMethods(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillMethods", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillSingleMethodAttribute(
        &mut self,
        realMethodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillSingleMethodAttribute", (realMethodInfo, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        eventInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentClass, eventInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        componentClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        eventInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::EventInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentClass, eventInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ReflectEventDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ReflectEventDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
