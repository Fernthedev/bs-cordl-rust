#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
#[repr(C)]
#[derive(Debug)]
pub struct IDtdParserAdapterV1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::IDtdParserAdapterV1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "IDtdParserAdapterV1";
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
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl std::ops::Deref for crate::System::Xml::IDtdParserAdapterV1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl std::ops::DerefMut for crate::System::Xml::IDtdParserAdapterV1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl crate::System::Xml::IDtdParserAdapterV1 {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Namespaces(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Namespaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Normalization(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Normalization", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_V1CompatibilityMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_V1CompatibilityMode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsRef<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsMut<crate::System::Xml::IDtdParserAdapter>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParserAdapter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsRef<crate::System::Xml::IDtdParserAdapterWithValidation>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_ref(&self) -> &crate::System::Xml::IDtdParserAdapterWithValidation {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+IDtdParserAdapterV1")]
impl AsMut<crate::System::Xml::IDtdParserAdapterWithValidation>
for crate::System::Xml::IDtdParserAdapterV1 {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IDtdParserAdapterWithValidation {
        unsafe { std::mem::transmute(self) }
    }
}
