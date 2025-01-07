#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeFeature {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::RuntimeFeature {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "RuntimeFeature";
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
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::RuntimeFeature {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::RuntimeFeature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
impl crate::System::Runtime::CompilerServices::RuntimeFeature {
    pub fn get_IsDynamicCodeSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsDynamicCodeSupported", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeFeature")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::RuntimeFeature {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
