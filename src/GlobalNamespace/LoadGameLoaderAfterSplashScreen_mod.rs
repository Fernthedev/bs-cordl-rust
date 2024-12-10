#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
#[repr(C)]
#[derive(Debug)]
pub struct LoadGameLoaderAfterSplashScreen {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _camera: *mut crate::UnityEngine::Camera,
}
#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LoadGameLoaderAfterSplashScreen
    => ""."LoadGameLoaderAfterSplashScreen"
);
#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
impl std::ops::Deref for crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
impl std::ops::DerefMut for crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
impl crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen {
    pub const kGameLoaderSceneName: &'static str = "GameLoader";
    #[cfg(feature = "LoadGameLoaderAfterSplashScreen+_Start_d__2")]
    pub type _Start_d__2 = crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen__Start_d__2;
    #[cfg(feature = "LoadGameLoaderAfterSplashScreen+__c")]
    pub type __c = crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen___c;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Start", ())?;
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
#[cfg(feature = "LoadGameLoaderAfterSplashScreen")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LoadGameLoaderAfterSplashScreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
