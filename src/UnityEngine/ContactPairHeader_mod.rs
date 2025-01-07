#[cfg(feature = "UnityEngine+ContactPairHeader")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContactPairHeader {
    pub m_BodyID: i32,
    pub m_OtherBodyID: i32,
    pub m_StartPtr: crate::System::IntPtr,
    pub m_NbPairs: u32,
    pub m_Flags: crate::UnityEngine::CollisionPairHeaderFlags,
    pub m_RelativeVelocity: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ContactPairHeader")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ContactPairHeader {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ContactPairHeader";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::ContactPairHeader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::ContactPairHeader {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::ContactPairHeader {
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
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::ContactPairHeader {
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
#[cfg(feature = "UnityEngine+ContactPairHeader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ContactPairHeader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPairHeader")]
impl crate::UnityEngine::ContactPairHeader {
    pub fn GetContactPair(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactPair>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ContactPair,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPair",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContactPair_Internal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetContactPair_Internal",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Body",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_BodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasRemovedBody(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HasRemovedBody",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Component> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherBody",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OtherBodyInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OtherBodyInstanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PairCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PairCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
