#[cfg(feature = "Vector3Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Vector3Extensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Vector3Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Vector3Extensions";
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
#[cfg(feature = "Vector3Extensions")]
impl std::ops::Deref for crate::GlobalNamespace::Vector3Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Vector3Extensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::Vector3Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Vector3Extensions")]
impl crate::GlobalNamespace::Vector3Extensions {
    pub fn Abs(
        vector: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn InverseLerp(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InverseLerp", (a, b, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MirrorEulerAnglesOnYZPlane(
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MirrorEulerAnglesOnYZPlane", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn MirrorOnYZPlane(
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MirrorOnYZPlane", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotatedAroundPivot(
        vector: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        pivot: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotatedAroundPivot", (vector, rotation, pivot))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Vector3Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Vector3Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
