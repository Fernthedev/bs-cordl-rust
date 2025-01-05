#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SetSamplingFrequencyCommand {
    padding: [u8; 12usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand =>
    "UnityEngine.InputSystem.LowLevel"."SetSamplingFrequencyCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand {
    pub const kSize: i32 = 12i32;
    pub fn Create(
        frequency: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (frequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_typeStatic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+SetSamplingFrequencyCommand")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::SetSamplingFrequencyCommand {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    > {
        todo!()
    }
}
