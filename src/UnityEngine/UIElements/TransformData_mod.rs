#[cfg(feature = "UnityEngine+UIElements+TransformData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransformData {
    pub rotate: crate::UnityEngine::UIElements::Rotate,
    pub scale: crate::UnityEngine::UIElements::Scale,
    pub transformOrigin: crate::UnityEngine::UIElements::TransformOrigin,
    pub translate: crate::UnityEngine::UIElements::Translate,
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TransformData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TransformData";
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
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::TransformData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::TransformData {
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
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::TransformData {
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
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::TransformData {
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
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TransformData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
impl crate::UnityEngine::UIElements::TransformData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransformData> {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransformData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TransformData0(
        &mut self,
        other: crate::UnityEngine::UIElements::TransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::TransformData,
        rhs: crate::UnityEngine::UIElements::TransformData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformData>>
for crate::UnityEngine::UIElements::TransformData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformData> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformData>>
for crate::UnityEngine::UIElements::TransformData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::TransformData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransformData,
    >,
> for crate::UnityEngine::UIElements::TransformData {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransformData,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformData")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransformData,
    >,
> for crate::UnityEngine::UIElements::TransformData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleDataGroup_1<
        crate::UnityEngine::UIElements::TransformData,
    > {
        todo!()
    }
}
