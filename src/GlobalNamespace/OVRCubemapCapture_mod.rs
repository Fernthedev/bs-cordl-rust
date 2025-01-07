#[cfg(feature = "OVRCubemapCapture")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRCubemapCapture {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub autoTriggerAfterLaunch: bool,
    pub autoTriggerDelay: f32,
    pub autoTriggerElapse: f32,
    pub triggeredByKey: crate::UnityEngine::KeyCode,
    pub pathName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cubemapSize: i32,
}
#[cfg(feature = "OVRCubemapCapture")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRCubemapCapture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRCubemapCapture";
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
#[cfg(feature = "OVRCubemapCapture")]
impl std::ops::Deref for crate::GlobalNamespace::OVRCubemapCapture {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCubemapCapture")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRCubemapCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRCubemapCapture")]
impl crate::GlobalNamespace::OVRCubemapCapture {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RenderIntoCubemap(
        ownerCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        outCubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RenderIntoCubemap", (ownerCamera, outCubemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveCubemapCapture(
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
        pathName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveCubemapCapture", (cubemap, pathName))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerCubemapCapture(
        capturePos: crate::UnityEngine::Vector3,
        cubemapSize: i32,
        pathName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriggerCubemapCapture", (capturePos, cubemapSize, pathName))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "OVRCubemapCapture")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRCubemapCapture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
