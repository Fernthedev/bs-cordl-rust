#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
#[repr(C)]
#[derive(Debug)]
pub struct IDualMotorRumble {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Haptics::IDualMotorRumble =>
    "UnityEngine.InputSystem.Haptics"."IDualMotorRumble"
);
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    pub fn SetMotorSpeeds(
        &mut self,
        lowFrequency: f32,
        highFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMotorSpeeds", (lowFrequency, highFrequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Haptics::IHaptics>>
for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Haptics::IHaptics> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Haptics+IDualMotorRumble")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Haptics::IHaptics>>
for crate::UnityEngine::InputSystem::Haptics::IDualMotorRumble {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Haptics::IHaptics,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
