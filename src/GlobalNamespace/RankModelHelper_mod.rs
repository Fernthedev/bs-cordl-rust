#[cfg(feature = "RankModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct RankModelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RankModelHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::RankModelHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RankModelHelper";
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
#[cfg(feature = "RankModelHelper")]
impl std::ops::Deref for crate::GlobalNamespace::RankModelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::RankModelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RankModelHelper")]
impl crate::GlobalNamespace::RankModelHelper {
    pub fn MaxRankForGameplayModifiers(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        gameplayModifiersModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiersModelSO,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::RankModel_Rank> {
        let __cordl_ret: crate::GlobalNamespace::RankModel_Rank = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MaxRankForGameplayModifiers",
                (gameplayModifiers, gameplayModifiersModel, energy),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RankModelHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RankModelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
