#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
#[repr(C)]
#[derive(Debug)]
pub struct ITypeDescriptorFilterService {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel.Design";
    const CLASS_NAME: &'static str = "ITypeDescriptorFilterService";
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
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl std::ops::Deref
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl std::ops::DerefMut
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    pub fn FilterAttributes(
        &mut self,
        component: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IComponent>,
        attributes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterAttributes", (component, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterEvents(
        &mut self,
        component: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IComponent>,
        events: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterEvents", (component, events))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterProperties(
        &mut self,
        component: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IComponent>,
        properties: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FilterProperties", (component, properties))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+ITypeDescriptorFilterService")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::ITypeDescriptorFilterService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
