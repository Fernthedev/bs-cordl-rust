#[cfg(feature = "LightGroupSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _groupDescription: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _groupId: i32,
    pub _startLightId: i32,
    pub _numberOfElements: i32,
    pub _sameIdElements: i32,
    pub _ignoreLightGroupEffectManager: bool,
}
#[cfg(feature = "LightGroupSO")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::LightGroupSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightGroupSO";
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
#[cfg(feature = "LightGroupSO")]
impl std::ops::Deref for crate::GlobalNamespace::LightGroupSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightGroupSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupSO")]
impl crate::GlobalNamespace::LightGroupSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_groupId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_groupId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_groupName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreLightGroupEffectManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ignoreLightGroupEffectManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfElements(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sameIdElements(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sameIdElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startLightId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_startLightId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightGroupSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightGroupSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightGroupSO")]
impl AsRef<crate::GlobalNamespace::ILightGroup>
for crate::GlobalNamespace::LightGroupSO {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILightGroup {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LightGroupSO")]
impl AsMut<crate::GlobalNamespace::ILightGroup>
for crate::GlobalNamespace::LightGroupSO {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILightGroup {
        unsafe { std::mem::transmute(self) }
    }
}
