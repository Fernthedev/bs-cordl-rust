#[cfg(feature = "Microsoft+Win32+Win32Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Win32Native {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
unsafe impl quest_hook::libil2cpp::Type for crate::Microsoft::Win32::Win32Native {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Microsoft.Win32";
    const CLASS_NAME: &'static str = "Win32Native";
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
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl std::ops::Deref for crate::Microsoft::Win32::Win32Native {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl std::ops::DerefMut for crate::Microsoft::Win32::Win32Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl crate::Microsoft::Win32::Win32Native {
    pub fn GetMessage(
        hr: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetMessage", (hr))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeHRFromErrorCode(errorCode: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeHRFromErrorCode", (errorCode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+Win32+Win32Native")]
impl quest_hook::libil2cpp::ObjectType for crate::Microsoft::Win32::Win32Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
