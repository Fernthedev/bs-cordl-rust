#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InterfaceTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: crate::System::Runtime::InteropServices::ComInterfaceType,
}
#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::InteropServices::InterfaceTypeAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.InteropServices";
    const CLASS_NAME: &'static str = "InterfaceTypeAttribute";
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
#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::InterfaceTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::InterfaceTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
impl crate::System::Runtime::InteropServices::InterfaceTypeAttribute {
    pub fn New(
        interfaceType: crate::System::Runtime::InteropServices::ComInterfaceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (interfaceType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        interfaceType: crate::System::Runtime::InteropServices::ComInterfaceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (interfaceType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+InterfaceTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::InterfaceTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
