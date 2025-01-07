#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector2Mask {
    pub m_Mask: u8,
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::Vector2Mask {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "Vector2Mask";
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
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ProBuilder::Vector2Mask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ProBuilder::Vector2Mask {
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
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ProBuilder::Vector2Mask {
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
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ProBuilder::Vector2Mask {
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
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Vector2Mask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
impl crate::UnityEngine::ProBuilder::Vector2Mask {
    pub const X: u8 = 1u8;
    pub const Y: u8 = 2u8;
    pub fn _ctor_Vector3_f32_0(
        &mut self,
        v: crate::UnityEngine::Vector3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, epsilon),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_1(
        &mut self,
        mask: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_x(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_x",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_y(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_y",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd(
        left: crate::UnityEngine::ProBuilder::Vector2Mask,
        right: crate::UnityEngine::ProBuilder::Vector2Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Vector2Mask> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Vector2Mask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr(
        left: crate::UnityEngine::ProBuilder::Vector2Mask,
        right: crate::UnityEngine::ProBuilder::Vector2Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Vector2Mask> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Vector2Mask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr(
        left: crate::UnityEngine::ProBuilder::Vector2Mask,
        right: crate::UnityEngine::ProBuilder::Vector2Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Vector2Mask> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Vector2Mask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector2Mask0(
        mask: crate::UnityEngine::ProBuilder::Vector2Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector2_1(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Vector2Mask> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Vector2Mask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        mask: crate::UnityEngine::ProBuilder::Vector2Mask,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (mask, value))?;
        Ok(__cordl_ret.into())
    }
}
