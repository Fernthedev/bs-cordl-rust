#[cfg(feature = "UnityEngine+SendMouseEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct SendMouseEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SendMouseEvents {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SendMouseEvents";
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
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl std::ops::Deref for crate::UnityEngine::SendMouseEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl std::ops::DerefMut for crate::UnityEngine::SendMouseEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl crate::UnityEngine::SendMouseEvents {
    #[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
    pub type HitInfo = crate::UnityEngine::SendMouseEvents_HitInfo;
    pub fn DoSendMouseEvents(
        skipRTCameras: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoSendMouseEvents", (skipRTCameras))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvents(
        i: i32,
        hit: crate::UnityEngine::SendMouseEvents_HitInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendEvents", (i, hit))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMouseMoved() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMouseMoved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMouse() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateMouse", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SendMouseEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendMouseEvents_HitInfo {
    pub target: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SendMouseEvents_HitInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "HitInfo";
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
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::SendMouseEvents_HitInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::SendMouseEvents_HitInfo {
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
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::SendMouseEvents_HitInfo {
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
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::SendMouseEvents_HitInfo {
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
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SendMouseEvents_HitInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SendMouseEvents+HitInfo")]
impl crate::UnityEngine::SendMouseEvents_HitInfo {
    pub fn Compare(
        lhs: crate::UnityEngine::SendMouseEvents_HitInfo,
        rhs: crate::UnityEngine::SendMouseEvents_HitInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendMessage(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendMessage",
            (name),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        exists: crate::UnityEngine::SendMouseEvents_HitInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (exists))?;
        Ok(__cordl_ret.into())
    }
}
