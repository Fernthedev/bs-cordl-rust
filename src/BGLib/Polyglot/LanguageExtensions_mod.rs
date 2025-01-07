#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LanguageExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LanguageExtensions";
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
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl std::ops::Deref for crate::BGLib::Polyglot::LanguageExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LanguageExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl crate::BGLib::Polyglot::LanguageExtensions {
    pub fn ToCultureInfoName(
        lang: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCultureInfoName", (lang))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLanguage_Il2CppString0(
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_ret: crate::BGLib::Polyglot::Language = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLanguage", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLanguage_SystemLanguage__cordl_bool1(
        systemLanguage: crate::UnityEngine::SystemLanguage,
        useFallbackLanguage: bool,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_ret: crate::BGLib::Polyglot::Language = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToLanguage", (systemLanguage, useFallbackLanguage))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializedName(
        lang: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSerializedName", (lang))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LanguageExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::Polyglot::LanguageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
