#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyDefaultAliasAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _DefaultAlias_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::AssemblyDefaultAliasAttribute => "System.Reflection"
    ."AssemblyDefaultAliasAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyDefaultAliasAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyDefaultAliasAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
impl crate::System::Reflection::AssemblyDefaultAliasAttribute {
    pub fn New(
        defaultAlias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (defaultAlias))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        defaultAlias: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (defaultAlias))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+AssemblyDefaultAliasAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyDefaultAliasAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
