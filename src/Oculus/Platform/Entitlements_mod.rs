#[cfg(feature = "Oculus+Platform+Entitlements")]
#[repr(C)]
#[derive(Debug)]
pub struct Entitlements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Entitlements {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Entitlements";
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
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::Deref for crate::Oculus::Platform::Entitlements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::DerefMut for crate::Oculus::Platform::Entitlements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl crate::Oculus::Platform::Entitlements {
    pub fn IsUserEntitledToApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUserEntitledToApplication", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Entitlements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
