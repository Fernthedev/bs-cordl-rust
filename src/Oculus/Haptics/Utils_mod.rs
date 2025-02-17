#[cfg(feature = "Oculus+Haptics+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Haptics+Utils")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Utils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "Utils";
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
#[cfg(feature = "Oculus+Haptics+Utils")]
impl std::ops::Deref for crate::Oculus::Haptics::Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl crate::Oculus::Haptics::Utils {
    pub fn ControllerToFfiController(
        controller: crate::Oculus::Haptics::Controller,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Controller> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Haptics::Controller),
                crate::Oculus::Haptics::Ffi_Controller,
                1usize,
            >("ControllerToFfiController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ControllerToFfiController", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Controller = unsafe {
            method.invoke_unchecked((), (controller))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Map(
        input: i32,
        inMin: i32,
        inMax: i32,
        outMin: i32,
        outMax: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32, i32), f32, 5usize>("Map")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Map", 5usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (input, inMin, inMax, outMin, outMax))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Haptics+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
