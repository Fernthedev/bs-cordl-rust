#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueryKeyboardLayoutCommand {
    padding: [u8; 264usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand =>
    "UnityEngine.InputSystem.LowLevel"."QueryKeyboardLayoutCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand {
    pub const kMaxNameLength: i32 = 256i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand+_nameBuffer_e__FixedBuffer"
    )]
    pub type _nameBuffer_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand__nameBuffer_e__FixedBuffer;
    pub fn ReadLayoutName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadLayoutName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WriteLayoutName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteLayoutName",
            (name),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand+_nameBuffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueryKeyboardLayoutCommand__nameBuffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand+_nameBuffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand__nameBuffer_e__FixedBuffer
    => "UnityEngine.InputSystem.LowLevel"
    ."QueryKeyboardLayoutCommand/<nameBuffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand+_nameBuffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand__nameBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyboardLayoutCommand+_nameBuffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::QueryKeyboardLayoutCommand__nameBuffer_e__FixedBuffer {}
