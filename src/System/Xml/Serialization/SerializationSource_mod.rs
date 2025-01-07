#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub includedTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub canBeGenerated: bool,
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::SerializationSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "SerializationSource";
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
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl std::ops::Deref for crate::System::Xml::Serialization::SerializationSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::SerializationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl crate::System::Xml::Serialization::SerializationSource {
    pub fn BaseEquals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::SerializationSource,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BaseEquals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includedTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namspace, includedTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includedTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namspace, includedTypes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::SerializationSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
