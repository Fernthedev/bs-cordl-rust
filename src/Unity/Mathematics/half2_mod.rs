#[cfg(feature = "Unity+Mathematics+half2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct half2 {
    pub x: crate::Unity::Mathematics::half,
    pub y: crate::Unity::Mathematics::half,
}
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::half2 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "half2";
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
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::half2 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::half2 {
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
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::half2 {
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
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::half2 {
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
#[cfg(feature = "Unity+Mathematics+half2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::half2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
impl crate::Unity::Mathematics::half2 {
    #[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::half2_DebuggerProxy;
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_half2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
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
    pub fn _ctor_double2_6(
        &mut self,
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_3(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_5(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float2_4(
        &mut self,
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half2(
        &mut self,
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half2_1(
        &mut self,
        xy: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half_half0(
        &mut self,
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_half2_half1(
        lhs: crate::Unity::Mathematics::half2,
        rhs: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_half2_half2_0(
        lhs: crate::Unity::Mathematics::half2,
        rhs: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_half_half2_2(
        lhs: crate::Unity::Mathematics::half,
        rhs: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2_3(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float2_1(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_half2_half1(
        lhs: crate::Unity::Mathematics::half2,
        rhs: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_half2_half2_0(
        lhs: crate::Unity::Mathematics::half2,
        rhs: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_half_half2_2(
        lhs: crate::Unity::Mathematics::half,
        rhs: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xy(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yx(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::half2>>
for crate::Unity::Mathematics::half2 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::half2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::half2>>
for crate::Unity::Mathematics::half2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::half2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::half2 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half2")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::half2 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct half2_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: crate::Unity::Mathematics::half,
    pub y: crate::Unity::Mathematics::half,
}
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Mathematics::half2_DebuggerProxy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "DebuggerProxy";
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
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::half2_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::half2_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
impl crate::Unity::Mathematics::half2_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+half2+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::half2_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
