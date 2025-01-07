#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct UnsupportedAdvancedHapticsPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnsupportedAdvancedHapticsPlayer";
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
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    pub fn CanPlayHapticPreset(
        &mut self,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanPlayHapticPreset", (hapticPreset, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PlayHapticFeedback(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        hapticPreset: quest_hook::libil2cpp::Gc<
            crate::Libraries::HM::HMLib::VR::HapticPresetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticFeedback", (node, hapticPreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl AsRef<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnsupportedAdvancedHapticsPlayer")]
impl AsMut<crate::GlobalNamespace::IHapticFeedbackPlayer>
for crate::GlobalNamespace::UnsupportedAdvancedHapticsPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IHapticFeedbackPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
