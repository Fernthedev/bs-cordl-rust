#[cfg(feature = "StandardLevelReturnToMenuController")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelReturnToMenuController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _standardLevelSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    >,
    pub _prepareLevelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrepareLevelCompletionResults,
    >,
}
#[cfg(feature = "StandardLevelReturnToMenuController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StandardLevelReturnToMenuController";
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
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl crate::GlobalNamespace::StandardLevelReturnToMenuController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnToMenu", ())?;
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
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl AsRef<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardLevelReturnToMenuController")]
impl AsMut<crate::GlobalNamespace::IReturnToMenuController>
for crate::GlobalNamespace::StandardLevelReturnToMenuController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReturnToMenuController {
        unsafe { std::mem::transmute(self) }
    }
}
