#[cfg(feature = "UnityEngine+Color32")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Color32 {
    padding: quest_hook::libil2cpp::ValueTypePadding<4usize>,
}
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Color32 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "Color32";
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
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Color32 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Color32 {
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
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Color32 {
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
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Color32 {
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
#[cfg(feature = "UnityEngine+Color32")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Color32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Color32")]
impl crate::UnityEngine::Color32 {
    pub fn Lerp(
        a: crate::UnityEngine::Color32,
        b: crate::UnityEngine::Color32,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (r, g, b, a),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Color0(
        c: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Color32_1(
        c: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Color32")]
impl AsRef<crate::System::IFormattable> for crate::UnityEngine::Color32 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Color32")]
impl AsMut<crate::System::IFormattable> for crate::UnityEngine::Color32 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
