#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TerrainUtils::TerrainUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TerrainUtils";
    const CLASS_NAME: &'static str = "TerrainUtility";
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
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl crate::UnityEngine::TerrainUtils::TerrainUtility {
    pub fn AutoConnect() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AutoConnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearConnectivity() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearConnectivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CollectTerrains(
        onlyAutoConnectedTerrains: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollectTerrains", (onlyAutoConnectedTerrains))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidTerrainsExist() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidTerrainsExist", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TerrainUtils::TerrainUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
