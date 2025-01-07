#[cfg(feature = "BeatSaber+Init+BSAppInit")]
#[repr(C)]
#[derive(Debug)]
pub struct BSAppInit {
    __cordl_parent: crate::GlobalNamespace::AppInit,
    pub _commandLineArguments_k__BackingField: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
}
#[cfg(feature = "BeatSaber+Init+BSAppInit")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Init::BSAppInit {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "BSAppInit";
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
#[cfg(feature = "BeatSaber+Init+BSAppInit")]
impl std::ops::Deref for crate::BeatSaber::Init::BSAppInit {
    type Target = crate::GlobalNamespace::AppInit;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSAppInit")]
impl std::ops::DerefMut for crate::BeatSaber::Init::BSAppInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSAppInit")]
impl crate::BeatSaber::Init::BSAppInit {
    pub const kAutoPlayOptionIdentifier: &'static str = "--auto_play";
    pub const kAutoRecOptionIdentifier: &'static str = "--auto_rec";
    pub const kCustomSettingsOptionIdentifier: &'static str = "--customSettings";
    pub const kInBuildGameVersion: &'static str = "InBuildGameVersion";
    pub const kRecordingToolOptionIdentifier: &'static str = "--enable_recording_tool";
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PreloadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("PreloadAsync", ())?;
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
    pub fn get_commandLineArguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult = __cordl_object
            .invoke("get_commandLineArguments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_commandLineArguments(
        &mut self,
        value: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_commandLineArguments", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+BSAppInit")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::BSAppInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
