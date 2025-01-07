#[cfg(feature = "LastUsedBeatmapDataCache")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LastUsedBeatmapDataCache {
    pub cachedReadonlyBeatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _environmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IEnvironmentInfo,
    >,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSpecificSettings,
    >,
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LastUsedBeatmapDataCache";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LastUsedBeatmapDataCache {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LastUsedBeatmapDataCache")]
impl crate::GlobalNamespace::LastUsedBeatmapDataCache {
    pub fn AreRequiredGameplayModifiersSame(
        first: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
        second: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreRequiredGameplayModifiersSame", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreRequiredPlayerSettingsSame(
        first: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
        second: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreRequiredPlayerSettingsSame", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreSameBeatmapDataCached(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AreSameBeatmapDataCached",
            (beatmapKey, environmentInfo, gameplayModifiers, playerSpecificSettings),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        environmentInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEnvironmentInfo,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                beatmapData,
                beatmapKey,
                environmentInfo,
                gameplayModifiers,
                playerSpecificSettings,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
