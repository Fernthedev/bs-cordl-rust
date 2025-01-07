#[cfg(feature = "UnityEngine+TextGenerationSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextGenerationSettings {
    pub font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    pub color: crate::UnityEngine::Color,
    pub fontSize: i32,
    pub lineSpacing: f32,
    pub richText: bool,
    pub scaleFactor: f32,
    pub fontStyle: crate::UnityEngine::FontStyle,
    pub textAnchor: crate::UnityEngine::TextAnchor,
    pub alignByGeometry: bool,
    pub resizeTextForBestFit: bool,
    pub resizeTextMinSize: i32,
    pub resizeTextMaxSize: i32,
    pub updateBounds: bool,
    pub verticalOverflow: crate::UnityEngine::VerticalWrapMode,
    pub horizontalOverflow: crate::UnityEngine::HorizontalWrapMode,
    pub generationExtents: crate::UnityEngine::Vector2,
    pub pivot: crate::UnityEngine::Vector2,
    pub generateOutOfBounds: bool,
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextGenerationSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "TextGenerationSettings";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextGenerationSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextGenerationSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextGenerationSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextGenerationSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextGenerationSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextGenerationSettings")]
impl crate::UnityEngine::TextGenerationSettings {
    pub fn CompareColors(
        &mut self,
        left: crate::UnityEngine::Color,
        right: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareColors",
            (left, right),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareVector2(
        &mut self,
        left: crate::UnityEngine::Vector2,
        right: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareVector2",
            (left, right),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
}
