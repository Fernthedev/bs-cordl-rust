#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct NoAvatarPoseRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "NoAvatarPoseRestriction";
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
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RestrictPose(
        &mut self,
        headRotation: crate::UnityEngine::Quaternion,
        headPosition: crate::UnityEngine::Vector3,
        leftHandPosition: crate::UnityEngine::Vector3,
        rightHandPosition: crate::UnityEngine::Vector3,
        newHeadPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        newLeftHandPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector3,
        >,
        newRightHandPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RestrictPose",
                (
                    headRotation,
                    headPosition,
                    leftHandPosition,
                    rightHandPosition,
                    newHeadPosition,
                    newLeftHandPosition,
                    newRightHandPosition,
                ),
            )?;
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
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarPoseRestriction>
for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+NoAvatarPoseRestriction")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarPoseRestriction>
for crate::BeatSaber::AvatarCore::NoAvatarPoseRestriction {
    fn as_mut(&mut self) -> &mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
        unsafe { std::mem::transmute(self) }
    }
}
