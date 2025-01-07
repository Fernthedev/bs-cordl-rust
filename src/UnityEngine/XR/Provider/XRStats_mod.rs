#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
#[repr(C)]
#[derive(Debug)]
pub struct XRStats {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::Provider::XRStats {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.Provider";
    const CLASS_NAME: &'static str = "XRStats";
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
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl std::ops::Deref for crate::UnityEngine::XR::Provider::XRStats {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl std::ops::DerefMut for crate::UnityEngine::XR::Provider::XRStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl crate::UnityEngine::XR::Provider::XRStats {
    pub fn TryGetStat(
        xrSubsystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetStat", (xrSubsystem, tag, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetStat_Internal(
        ptr: crate::System::IntPtr,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetStat_Internal", (ptr, tag, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::Provider::XRStats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
