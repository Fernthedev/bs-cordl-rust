#[cfg(feature = "MainCameraCullingMask")]
#[repr(C)]
#[derive(Debug)]
pub struct MainCameraCullingMask {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainCameraCullingMask_InitData,
    >,
}
#[cfg(feature = "MainCameraCullingMask")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MainCameraCullingMask {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MainCameraCullingMask";
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
#[cfg(feature = "MainCameraCullingMask")]
impl std::ops::Deref for crate::GlobalNamespace::MainCameraCullingMask {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainCameraCullingMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask")]
impl crate::GlobalNamespace::MainCameraCullingMask {
    #[cfg(feature = "MainCameraCullingMask+InitData")]
    pub type InitData = crate::GlobalNamespace::MainCameraCullingMask_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "MainCameraCullingMask")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainCameraCullingMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MainCameraCullingMask_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub showDebris: bool,
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MainCameraCullingMask/InitData";
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
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl crate::GlobalNamespace::MainCameraCullingMask_InitData {
    pub fn New(
        showDebris: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (showDebris))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        showDebris: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (showDebris))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MainCameraCullingMask+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainCameraCullingMask_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
