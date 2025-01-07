#[cfg(feature = "LocalizedHoverHint")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizedHoverHint {
    __cordl_parent: crate::BGLib::Polyglot::LocalizedTextComponent_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    >,
}
#[cfg(feature = "LocalizedHoverHint")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LocalizedHoverHint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalizedHoverHint";
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
#[cfg(feature = "LocalizedHoverHint")]
impl std::ops::Deref for crate::GlobalNamespace::LocalizedHoverHint {
    type Target = crate::BGLib::Polyglot::LocalizedTextComponent_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedHoverHint")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalizedHoverHint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalizedHoverHint")]
impl crate::GlobalNamespace::LocalizedHoverHint {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetText(
        &mut self,
        hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (hoverHint, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAlignment(
        &mut self,
        hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
        direction: crate::BGLib::Polyglot::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAlignment", (hoverHint, direction))?;
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
#[cfg(feature = "LocalizedHoverHint")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LocalizedHoverHint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
