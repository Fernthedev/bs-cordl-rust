#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct CamelCaseNamingStrategy {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::NamingStrategy,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::CamelCaseNamingStrategy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "CamelCaseNamingStrategy";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::CamelCaseNamingStrategy {
    type Target = crate::Newtonsoft::Json::Serialization::NamingStrategy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::CamelCaseNamingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
impl crate::Newtonsoft::Json::Serialization::CamelCaseNamingStrategy {
    pub fn New_2() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool0(
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processDictionaryKeys, overrideSpecifiedNames))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool__cordl_bool1(
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
        processExtensionDataNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    processDictionaryKeys,
                    overrideSpecifiedNames,
                    processExtensionDataNames,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ResolvePropertyName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolvePropertyName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool0(
        &mut self,
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processDictionaryKeys, overrideSpecifiedNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        processDictionaryKeys: bool,
        overrideSpecifiedNames: bool,
        processExtensionDataNames: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    processDictionaryKeys,
                    overrideSpecifiedNames,
                    processExtensionDataNames,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+CamelCaseNamingStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::CamelCaseNamingStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
