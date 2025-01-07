#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
#[repr(C)]
#[derive(Debug)]
pub struct ShorthandApplicator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::ShorthandApplicator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "ShorthandApplicator";
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::ShorthandApplicator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::ShorthandApplicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
impl crate::UnityEngine::UIElements::StyleSheets::ShorthandApplicator {
    pub fn ApplyBackgroundPosition(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBackgroundPosition", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBorderColor(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBorderColor", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBorderRadius(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBorderRadius", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBorderWidth(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyBorderWidth", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyFlex(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyFlex", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyMargin(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyMargin", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPadding(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyPadding", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyTransition(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyTransition", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUnityBackgroundScaleMode(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUnityBackgroundScaleMode", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUnityTextOutline(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUnityTextOutline", (reader, computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBackgroundPosition(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        backgroundPositionX: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPosition,
        >,
        backgroundPositionY: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CompileBackgroundPosition",
                (reader, backgroundPositionX, backgroundPositionY),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBorderRadius(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        top: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        right: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        bottom: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        left: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileBorderRadius", (reader, top, right, bottom, left))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBoxArea_StylePropertyReader_ByRefMut_ByRefMut_ByRefMut_ByRefMut0(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        top: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        right: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        bottom: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        left: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileBoxArea", (reader, top, right, bottom, left))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBoxArea_StylePropertyReader_ByRefMut_ByRefMut_ByRefMut_ByRefMut1(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        top: quest_hook::libil2cpp::ByRefMut<f32>,
        right: quest_hook::libil2cpp::ByRefMut<f32>,
        bottom: quest_hook::libil2cpp::ByRefMut<f32>,
        left: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileBoxArea", (reader, top, right, bottom, left))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileBoxArea_StylePropertyReader_ByRefMut_ByRefMut_ByRefMut_ByRefMut2(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        top: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        right: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        bottom: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        left: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileBoxArea", (reader, top, right, bottom, left))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileFlexShorthand(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        grow: quest_hook::libil2cpp::ByRefMut<f32>,
        shrink: quest_hook::libil2cpp::ByRefMut<f32>,
        basis: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileFlexShorthand", (reader, grow, shrink, basis))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileTextOutline(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        outlineColor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        outlineWidth: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompileTextOutline", (reader, outlineColor, outlineWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileTransition(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        outDelay: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::TimeValue,
                >,
            >,
        >,
        outDuration: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::TimeValue,
                >,
            >,
        >,
        outProperty: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::StylePropertyName,
                >,
            >,
        >,
        outTimingFunction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::EasingFunction,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CompileTransition",
                (reader, outDelay, outDuration, outProperty, outTimingFunction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompileUnityBackgroundScaleMode(
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        backgroundPositionX: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPosition,
        >,
        backgroundPositionY: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPosition,
        >,
        backgroundRepeat: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundRepeat,
        >,
        backgroundSize: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundSize,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CompileUnityBackgroundScaleMode",
                (
                    reader,
                    backgroundPositionX,
                    backgroundPositionY,
                    backgroundRepeat,
                    backgroundSize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _CompileBackgroundPosition_g__SwapKeyword_16_0(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPositionKeyword,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundPositionKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<CompileBackgroundPosition>g__SwapKeyword|16_0", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+ShorthandApplicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::ShorthandApplicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
