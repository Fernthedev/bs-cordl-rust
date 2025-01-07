#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
#[repr(C)]
#[derive(Debug)]
pub struct Level2Map {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Source: u8,
    pub Replace: u8,
}
#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::Level2Map {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "Level2Map";
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
#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::Level2Map {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::Level2Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
impl crate::Mono::Globalization::Unicode::Level2Map {
    pub fn New(
        source: u8,
        replace: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, replace))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        source: u8,
        replace: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, replace))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+Level2Map")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::Level2Map {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
