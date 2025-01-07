#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct GameBuildInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Init::Generated::GameBuildInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Init.Generated";
    const CLASS_NAME: &'static str = "GameBuildInfo";
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
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl std::ops::Deref for crate::BeatSaber::Init::Generated::GameBuildInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl std::ops::DerefMut for crate::BeatSaber::Init::Generated::GameBuildInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl crate::BeatSaber::Init::Generated::GameBuildInfo {
    pub const kDefaultPlatformVersion: &'static str = "1390";
    pub const kPreReleaseLabel: &'static str = "";
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Init::Generated::GameBuildInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
