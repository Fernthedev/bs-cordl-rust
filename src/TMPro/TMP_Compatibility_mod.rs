#[cfg(feature = "TMPro+TMP_Compatibility")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Compatibility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_Compatibility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_Compatibility";
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
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl std::ops::Deref for crate::TMPro::TMP_Compatibility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl std::ops::DerefMut for crate::TMPro::TMP_Compatibility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl crate::TMPro::TMP_Compatibility {
    #[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
    pub type AnchorPositions = crate::TMPro::TMP_Compatibility_AnchorPositions;
    pub fn ConvertTextAlignmentEnumValues(
        oldValue: crate::TMPro::TextAlignmentOptions,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TextAlignmentOptions> {
        let __cordl_ret: crate::TMPro::TextAlignmentOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTextAlignmentEnumValues", (oldValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Compatibility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TMP_Compatibility_AnchorPositions {
    #[default]
    BaseLine = 9i32,
    Bottom = 7i32,
    BottomLeft = 6i32,
    BottomRight = 8i32,
    Center = 4i32,
    Left = 3i32,
    None = 10i32,
    Right = 5i32,
    Top = 1i32,
    TopLeft = 0i32,
    TopRight = 2i32,
}
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::TMPro::TMP_Compatibility_AnchorPositions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "AnchorPositions";
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
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::TMPro::TMP_Compatibility_AnchorPositions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TMPro::TMP_Compatibility_AnchorPositions {
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
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::TMPro::TMP_Compatibility_AnchorPositions {
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
#[cfg(feature = "TMPro+TMP_Compatibility+AnchorPositions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::TMPro::TMP_Compatibility_AnchorPositions {
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
