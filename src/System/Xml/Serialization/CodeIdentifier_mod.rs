#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct CodeIdentifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::CodeIdentifier =>
    "System.Xml.Serialization"."CodeIdentifier"
);
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl std::ops::Deref for crate::System::Xml::Serialization::CodeIdentifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::CodeIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl crate::System::Xml::Serialization::CodeIdentifier {
    pub fn IsValid(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidStart(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidStart", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakePascal(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakePascal", (identifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeValid(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeValid", (identifier))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+CodeIdentifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::CodeIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
