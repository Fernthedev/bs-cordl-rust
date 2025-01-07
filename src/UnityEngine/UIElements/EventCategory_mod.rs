#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventCategory {
    #[default]
    Bind = 9i32,
    ChangePanel = 11i32,
    ChangeValue = 8i32,
    Command = 14i32,
    Default = 0i32,
    EnterLeave = 3i32,
    EnterLeaveWindow = 4i32,
    Focus = 10i32,
    Geometry = 6i32,
    IMGUI = 16i32,
    Keyboard = 5i32,
    Navigation = 13i32,
    Pointer = 1i32,
    PointerMove = 2i32,
    Reserved = 31i32,
    Style = 7i32,
    StyleTransition = 12i32,
    Tooltip = 15i32,
}
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::EventCategory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventCategory";
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
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::EventCategory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::EventCategory {
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
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::EventCategory {
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
#[cfg(feature = "UnityEngine+UIElements+EventCategory")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::EventCategory {
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
