#[cfg(feature = "OculusRichPresencePlatformHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRichPresencePlatformHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusRichPresencePlatformHandler";
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
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetPresence(
        &mut self,
        richPresenceData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IRichPresenceData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPresence", (richPresenceData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl AsRef<crate::GlobalNamespace::IRichPresencePlatformHandler>
for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRichPresencePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusRichPresencePlatformHandler")]
impl AsMut<crate::GlobalNamespace::IRichPresencePlatformHandler>
for crate::GlobalNamespace::OculusRichPresencePlatformHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRichPresencePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
