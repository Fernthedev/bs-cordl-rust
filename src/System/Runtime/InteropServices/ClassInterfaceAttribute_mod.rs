#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ClassInterfaceAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: crate::System::Runtime::InteropServices::ClassInterfaceType,
}
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::ClassInterfaceAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "ClassInterfaceAttribute";
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
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::ClassInterfaceAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::ClassInterfaceAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
impl crate::System::Runtime::InteropServices::ClassInterfaceAttribute {
    pub fn New(
        classInterfaceType: crate::System::Runtime::InteropServices::ClassInterfaceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classInterfaceType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        classInterfaceType: crate::System::Runtime::InteropServices::ClassInterfaceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (classInterfaceType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ClassInterfaceAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
