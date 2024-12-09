#[cfg(feature = "SongRunningHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SongRunningHelper {
    __cordl_parent: crate::System::Object,
    pub _beatmapCharacteristics: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _menuTransitionsHelper: *mut crate::GlobalNamespace::MenuTransitionsHelper,
    pub _beatmapLevels: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
    pub _playQueue: *mut crate::System::Collections::Generic::Queue_1<
        crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    >,
    pub _current: crate::System::Nullable_1<
        crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    >,
}
#[cfg(feature = "SongRunningHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongRunningHelper => ""
    ."SongRunningHelper"
);
#[cfg(feature = "SongRunningHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SongRunningHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongRunningHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongRunningHelper")]
impl crate::GlobalNamespace::SongRunningHelper {
    #[cfg(feature = "SongRunningHelper+QueuedSongParams")]
    pub type QueuedSongParams = crate::GlobalNamespace::SongRunningHelper_QueuedSongParams;
    pub fn EnqueueLevel(
        &mut self,
        queuedSongParams: crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EnqueueLevel", (queuedSongParams))?;
        Ok(__cordl_ret)
    }
    pub fn FindBeatmapLevelPackBeatmaps(
        &mut self,
        packId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("FindBeatmapLevelPackBeatmaps", (packId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StartLevel(
        &mut self,
        queuedSongParams: crate::GlobalNamespace::SongRunningHelper_QueuedSongParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLevel", (queuedSongParams))?;
        Ok(__cordl_ret)
    }
    pub fn StopAllLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAllLevels", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopCurrentLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopCurrentLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _StartLevel_g__HandleLevelDidFinishCallback_9_0(
        &mut self,
        standardLevelScenesTransition: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        results: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<StartLevel>g__HandleLevelDidFinishCallback|9_0",
                (standardLevelScenesTransition, results),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SongRunningHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongRunningHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SongRunningHelper_QueuedSongParams {
    pub beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub characteristic: *mut crate::System::String,
    pub advancedHud: bool,
    pub songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    pub recordingToolData: crate::GlobalNamespace::RecordingToolManager_SetupData,
    pub songFinishedCallback: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        *mut crate::GlobalNamespace::LevelCompletionResults,
    >,
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SongRunningHelper_QueuedSongParams => ""
    ."SongRunningHelper/QueuedSongParams"
);
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongRunningHelper+QueuedSongParams")]
impl crate::GlobalNamespace::SongRunningHelper_QueuedSongParams {
    pub fn _ctor(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        characteristic: *mut crate::System::String,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        advancedHud: bool,
        songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
        recordingToolData: crate::GlobalNamespace::RecordingToolManager_SetupData,
        songFinishedCallback: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
            *mut crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                beatmapLevel,
                characteristic,
                difficulty,
                advancedHud,
                songSpeed,
                recordingToolData,
                songFinishedCallback,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
