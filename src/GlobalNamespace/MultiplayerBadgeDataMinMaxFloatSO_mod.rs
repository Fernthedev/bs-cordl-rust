#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerBadgeDataMinMaxFloatSO {
    __cordl_parent: crate::GlobalNamespace::MultiplayerBadgeDataSO,
    pub _minMax: crate::GlobalNamespace::MultiplayerBadgeMinMax,
    pub _weightMultiplier: f32,
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerBadgeDataMinMaxFloatSO";
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
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO {
    type Target = crate::GlobalNamespace::MultiplayerBadgeDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO {
    pub fn CalculateBadgeData(
        &mut self,
        resultsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerBadgeAwardData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgeAwardData,
        > = __cordl_object
            .invoke(
                "CalculateBadgeData",
                (resultsData, playerDataModel, beatmapKey, randomMultiplier),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMax(
        &mut self,
        resultsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerBadgeAwardData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgeAwardData,
        > = __cordl_object.invoke("CalculateMax", (resultsData, randomMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMin(
        &mut self,
        resultsData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
        randomMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerBadgeAwardData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgeAwardData,
        > = __cordl_object.invoke("CalculateMin", (resultsData, randomMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
        result: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetValue", (result))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "MultiplayerBadgeDataMinMaxFloatSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerBadgeDataMinMaxFloatSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
