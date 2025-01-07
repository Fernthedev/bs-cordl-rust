#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub xmlNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub lineNumber: i32,
    pub linePosition: i32,
}
#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlNodeEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlNodeEventArgs";
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
#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlNodeEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlNodeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
impl crate::System::Xml::Serialization::XmlNodeEventArgs {
    pub fn New(
        xmlNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        lineNumber: i32,
        linePosition: i32,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlNode, lineNumber, linePosition, o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        xmlNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        lineNumber: i32,
        linePosition: i32,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlNode, lineNumber, linePosition, o))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlNodeEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlNodeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
