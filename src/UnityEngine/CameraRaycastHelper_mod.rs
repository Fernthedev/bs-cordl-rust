#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CameraRaycastHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::CameraRaycastHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "CameraRaycastHelper";
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
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl std::ops::Deref for crate::UnityEngine::CameraRaycastHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl std::ops::DerefMut for crate::UnityEngine::CameraRaycastHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl crate::UnityEngine::CameraRaycastHelper {
    pub fn RaycastTry(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        ray: crate::UnityEngine::Ray,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastTry", (cam, ray, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastTry2D(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        ray: crate::UnityEngine::Ray,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastTry2D", (cam, ray, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastTry2D_Injected(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastTry2D_Injected", (cam, ray, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaycastTry_Injected(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        ray: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Ray>,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastTry_Injected", (cam, ray, distance, layerMask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::CameraRaycastHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
