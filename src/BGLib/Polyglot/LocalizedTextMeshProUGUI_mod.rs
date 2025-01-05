#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedTextMeshProUGUI {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    >,
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizedTextMeshProUGUI =>
    "BGLib.Polyglot"."LocalizedTextMeshProUGUI"
);
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizedTextMeshProUGUI {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizedTextMeshProUGUI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
impl crate::BGLib::Polyglot::LocalizedTextMeshProUGUI {
    pub fn IsAlignmentLeft(
        alignment: crate::TMPro::TextAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAlignmentLeft", (alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAlignmentRight(
        alignment: crate::TMPro::TextAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAlignmentRight", (alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOppositeDirection(
        alignment: crate::TMPro::TextAlignmentOptions,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOppositeDirection", (alignment, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (text, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAlignment(
        &mut self,
        text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAlignment", (text, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizedTextMeshProUGUI")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::Polyglot::LocalizedTextMeshProUGUI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
