#[cfg(feature = "System+Threading+TimeoutHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeoutHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::TimeoutHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "TimeoutHelper";
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
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl std::ops::Deref for crate::System::Threading::TimeoutHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl std::ops::DerefMut for crate::System::Threading::TimeoutHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl crate::System::Threading::TimeoutHelper {
    pub fn GetTime() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTimeOut(
        startTime: u32,
        originalWaitMillisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateTimeOut", (startTime, originalWaitMillisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::TimeoutHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
