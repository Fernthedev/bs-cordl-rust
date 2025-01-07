#[cfg(feature = "CoroutineStarter")]
#[repr(C)]
#[derive(Debug)]
pub struct CoroutineStarter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "CoroutineStarter")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CoroutineStarter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CoroutineStarter";
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
#[cfg(feature = "CoroutineStarter")]
impl std::ops::Deref for crate::GlobalNamespace::CoroutineStarter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineStarter")]
impl std::ops::DerefMut for crate::GlobalNamespace::CoroutineStarter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CoroutineStarter")]
impl crate::GlobalNamespace::CoroutineStarter {
    pub fn ICoroutineStarter_StartCoroutine(
        &mut self,
        routine: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine> = __cordl_object
            .invoke("ICoroutineStarter.StartCoroutine", (routine))?;
        Ok(__cordl_ret.into())
    }
    pub fn ICoroutineStarter_StopCoroutine(
        &mut self,
        routine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ICoroutineStarter.StopCoroutine", (routine))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "CoroutineStarter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CoroutineStarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CoroutineStarter")]
impl AsRef<crate::GlobalNamespace::ICoroutineStarter>
for crate::GlobalNamespace::CoroutineStarter {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICoroutineStarter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "CoroutineStarter")]
impl AsMut<crate::GlobalNamespace::ICoroutineStarter>
for crate::GlobalNamespace::CoroutineStarter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ICoroutineStarter {
        unsafe { std::mem::transmute(self) }
    }
}
