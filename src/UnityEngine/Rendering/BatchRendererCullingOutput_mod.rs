#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchRendererCullingOutput {
    pub cullingJobsFence: crate::Unity::Jobs::JobHandle,
    pub localToWorldMatrix: crate::UnityEngine::Matrix4x4,
    pub cullingPlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cullingPlaneCount: i32,
    pub receiverPlaneOffset: i32,
    pub receiverPlaneCount: i32,
    pub cullingSplits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cullingSplitCount: i32,
    pub viewType: crate::UnityEngine::Rendering::BatchCullingViewType,
    pub projectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
    pub cullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
    pub viewID: u64,
    pub cullingLayerMask: u32,
    pub sceneCullingMask: u64,
    pub drawCommands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BatchRendererCullingOutput";
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
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
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
impl crate::UnityEngine::Rendering::BatchRendererCullingOutput {}
