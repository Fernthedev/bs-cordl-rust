#[cfg(feature = "BloomFogParamsAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogParamsAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _bloomFog: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogSO>,
}
#[cfg(feature = "BloomFogParamsAnimator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomFogParamsAnimator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomFogParamsAnimator";
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
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFogParamsAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFogParamsAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogParamsAnimator")]
impl crate::GlobalNamespace::BloomFogParamsAnimator {
    pub fn AnimateBloomFogParamsChange(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateBloomFogParamsChange", (envFogParams, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn AnimationCoroutine(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("AnimationCoroutine", (envFogParams, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultBloomFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogEnvironmentParams>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        > = __cordl_object.invoke("GetDefaultBloomFogParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetBloomFogParamsChange(
        &mut self,
        envFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
        transition: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBloomFogParamsChange", (envFogParams, transition))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultBloomFogParams(
        &mut self,
        newDefaultBloomFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultBloomFogParams", (newDefaultBloomFogParams))?;
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
#[cfg(feature = "BloomFogParamsAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomFogParamsAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
