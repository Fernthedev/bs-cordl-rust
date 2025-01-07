#[cfg(feature = "Oculus+Haptics+Ffi")]
#[repr(C)]
#[derive(Debug)]
pub struct Ffi {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "Ffi";
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
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl std::ops::Deref for crate::Oculus::Haptics::Ffi {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Ffi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl crate::Oculus::Haptics::Ffi {
    pub const InvalidId: i32 = -1i32;
    pub const NativeLibName: &'static str = "haptics_sdk";
    #[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
    pub type Controller = crate::Oculus::Haptics::Ffi_Controller;
    #[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
    pub type LogCallback = crate::Oculus::Haptics::Ffi_LogCallback;
    #[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
    pub type LogLevel = crate::Oculus::Haptics::Ffi_LogLevel;
    #[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
    pub type NullBackendStatistics = crate::Oculus::Haptics::Ffi_NullBackendStatistics;
    #[cfg(feature = "Oculus+Haptics+Ffi+Result")]
    pub type Result = crate::Oculus::Haptics::Ffi_Result;
    #[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
    pub type SdkVersion = crate::Oculus::Haptics::Ffi_SdkVersion;
    pub fn Failed(
        result: crate::Oculus::Haptics::Ffi_Result,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Failed", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Succeeded(
        result: crate::Oculus::Haptics::Ffi_Result,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Succeeded", (result))?;
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
    pub fn clip_duration(
        clipId: i32,
        clip_duration: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clip_duration", (clipId, clip_duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn create_player(
        player_id: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("create_player", (player_id))?;
        Ok(__cordl_ret.into())
    }
    pub fn error_message() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("error_message", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_null_backend_statistics() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Haptics::Ffi_NullBackendStatistics,
    > {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_NullBackendStatistics = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_null_backend_statistics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn haptics_sdk_error_message() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("haptics_sdk_error_message", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn haptics_sdk_error_message_length() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("haptics_sdk_error_message_length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn initialize_with_null_backend(
        logCallback: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::Ffi_LogCallback>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("initialize_with_null_backend", (logCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn initialize_with_ovr_plugin(
        game_engine_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        game_engine_version: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        game_engine_haptics_sdk_version: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        logCallback: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::Ffi_LogCallback>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "initialize_with_ovr_plugin",
                (
                    game_engine_name,
                    game_engine_version,
                    game_engine_haptics_sdk_version,
                    logCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn initialize_with_ovr_plugin_bytes(
        game_engine_name: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        game_engine_version: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        game_engine_haptics_sdk_version: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        logCallback: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::Ffi_LogCallback>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "initialize_with_ovr_plugin_bytes",
                (
                    game_engine_name,
                    game_engine_version,
                    game_engine_haptics_sdk_version,
                    logCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn initialized(
        initialized: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("initialized", (initialized))?;
        Ok(__cordl_ret.into())
    }
    pub fn load_clip(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clip_id_out: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("load_clip", (data, clip_id_out))?;
        Ok(__cordl_ret.into())
    }
    pub fn load_clip_bytes(
        data: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        data_length: u32,
        clip_id_out: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("load_clip_bytes", (data, data_length, clip_id_out))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_amplitude(
        playerId: i32,
        amplitude: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_amplitude", (playerId, amplitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_frequency_shift(
        playerId: i32,
        frequency_shift: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_frequency_shift", (playerId, frequency_shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_looping_enabled(
        playerId: i32,
        looping_enabled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_looping_enabled", (playerId, looping_enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_play(
        playerId: i32,
        controller: crate::Oculus::Haptics::Ffi_Controller,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_play", (playerId, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_priority(
        playerId: i32,
        priority: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_priority", (playerId, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_set_amplitude(
        playerId: i32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_set_amplitude", (playerId, amplitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_set_clip(
        playerId: i32,
        clipId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_set_clip", (playerId, clipId))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_set_frequency_shift(
        playerId: i32,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_set_frequency_shift", (playerId, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_set_looping_enabled(
        playerId: i32,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_set_looping_enabled", (playerId, enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_set_priority(
        playerId: i32,
        priority: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_set_priority", (playerId, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn player_stop(
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("player_stop", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn release_clip(
        clipId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("release_clip", (clipId))?;
        Ok(__cordl_ret.into())
    }
    pub fn release_player(
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("release_player", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_suspended(
        suspended: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Haptics::Ffi_Result> {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_suspended", (suspended))?;
        Ok(__cordl_ret.into())
    }
    pub fn uninitialize() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Haptics::Ffi_Result,
    > {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uninitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn version() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Haptics::Ffi_SdkVersion,
    > {
        let __cordl_ret: crate::Oculus::Haptics::Ffi_SdkVersion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Ffi {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ffi_Controller {
    #[default]
    Both = 2i32,
    Left = 0i32,
    Right = 1i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi_Controller {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "Controller";
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
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Oculus::Haptics::Ffi_Controller {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Oculus::Haptics::Ffi_Controller {
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
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Oculus::Haptics::Ffi_Controller {
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
#[cfg(feature = "Oculus+Haptics+Ffi+Controller")]
unsafe impl quest_hook::libil2cpp::Return for crate::Oculus::Haptics::Ffi_Controller {
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
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct Ffi_LogCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi_LogCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "LogCallback";
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
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl std::ops::Deref for crate::Oculus::Haptics::Ffi_LogCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Ffi_LogCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl crate::Oculus::Haptics::Ffi_LogCallback {
    pub fn BeginInvoke(
        &mut self,
        level: crate::Oculus::Haptics::Ffi_LogLevel,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (level, message, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        level: crate::Oculus::Haptics::Ffi_LogLevel,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (level, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Ffi_LogCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ffi_LogLevel {
    #[default]
    Debug = 1i32,
    Error = 4i32,
    Info = 2i32,
    Trace = 0i32,
    Warn = 3i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi_LogLevel {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "LogLevel";
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
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Oculus::Haptics::Ffi_LogLevel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Oculus::Haptics::Ffi_LogLevel {
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
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Oculus::Haptics::Ffi_LogLevel {
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
#[cfg(feature = "Oculus+Haptics+Ffi+LogLevel")]
unsafe impl quest_hook::libil2cpp::Return for crate::Oculus::Haptics::Ffi_LogLevel {
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
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ffi_NullBackendStatistics {
    pub play_call_count: i64,
    pub stop_call_count: i64,
    pub samples_played: i64,
}
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "NullBackendStatistics";
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
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
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
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
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
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
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
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Haptics::Ffi_NullBackendStatistics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+NullBackendStatistics")]
impl crate::Oculus::Haptics::Ffi_NullBackendStatistics {}
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ffi_Result {
    #[default]
    ClipIdInvalid = -9i32,
    CreatePlayerFailed = -8i32,
    Error = -1i32,
    InstanceAlreadyInitialized = -3i32,
    InstanceAlreadyUninitialized = -4i32,
    InstanceInitializationFailed = -2i32,
    InstanceNotInitialized = -5i32,
    InvalidUtf8 = -6i32,
    LoadClipFailed = -7i32,
    NoClipLoaded = -14i32,
    PlayerIdInvalid = -10i32,
    PlayerInvalidAmplitude = -11i32,
    PlayerInvalidFrequencyShift = -12i32,
    PlayerInvalidPriority = -13i32,
    Success = 0i32,
}
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi_Result {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "Result";
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
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Oculus::Haptics::Ffi_Result {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Oculus::Haptics::Ffi_Result {
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
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Oculus::Haptics::Ffi_Result {
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
#[cfg(feature = "Oculus+Haptics+Ffi+Result")]
unsafe impl quest_hook::libil2cpp::Return for crate::Oculus::Haptics::Ffi_Result {
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
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ffi_SdkVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Ffi_SdkVersion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "SdkVersion";
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
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Oculus::Haptics::Ffi_SdkVersion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Oculus::Haptics::Ffi_SdkVersion {
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
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Oculus::Haptics::Ffi_SdkVersion {
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
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::Return for crate::Oculus::Haptics::Ffi_SdkVersion {
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
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Oculus::Haptics::Ffi_SdkVersion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Oculus+Haptics+Ffi+SdkVersion")]
impl crate::Oculus::Haptics::Ffi_SdkVersion {}
