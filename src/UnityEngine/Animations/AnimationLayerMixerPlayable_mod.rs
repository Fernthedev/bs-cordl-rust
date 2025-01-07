#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationLayerMixerPlayable {
    pub m_Handle: crate::UnityEngine::Playables::PlayableHandle,
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Animations";
    const CLASS_NAME: &'static str = "AnimationLayerMixerPlayable";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
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
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    pub fn Create(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        inputCount: i32,
        singleLayerOptimization: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        let __cordl_ret: crate::UnityEngine::Animations::AnimationLayerMixerPlayable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (graph, inputCount, singleLayerOptimization))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandle(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        inputCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandle", (graph, inputCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal", (graph, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHandleInternal_Injected(
        graph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableGraph,
        >,
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHandleInternal_Injected", (graph, handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableHandle> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHandle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerMaskFromAvatarMask(
        &mut self,
        layerIndex: u32,
        mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLayerMaskFromAvatarMask",
            (layerIndex, mask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerMaskFromAvatarMaskInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        layerIndex: u32,
        mask: quest_hook::libil2cpp::Gc<crate::UnityEngine::AvatarMask>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLayerMaskFromAvatarMaskInternal", (handle, layerIndex, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSingleLayerOptimizationInternal(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Playables::PlayableHandle,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSingleLayerOptimizationInternal", (handle, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handle: crate::UnityEngine::Playables::PlayableHandle,
        singleLayerOptimization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (handle, singleLayerOptimization),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        playable: crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::Playable> {
        let __cordl_ret: crate::UnityEngine::Playables::Playable = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (playable))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    >,
> for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    >,
> for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Animations::AnimationLayerMixerPlayable,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsRef<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Animations+AnimationLayerMixerPlayable")]
impl AsMut<crate::UnityEngine::Playables::IPlayable>
for crate::UnityEngine::Animations::AnimationLayerMixerPlayable {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayable {
        todo!()
    }
}
