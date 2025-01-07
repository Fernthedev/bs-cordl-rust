#[cfg(feature = "Oculus+Platform+PlatformInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::PlatformInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "PlatformInternal";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl crate::Oculus::Platform::PlatformInternal {
    #[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
    pub type HTTP = crate::Oculus::Platform::PlatformInternal_HTTP;
    #[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
    pub type MessageTypeInternal = crate::Oculus::Platform::PlatformInternal_MessageTypeInternal;
    #[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
    pub type Users = crate::Oculus::Platform::PlatformInternal_Users;
    pub fn CrashApplication() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CrashApplication", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeStandaloneAsync(
        appID: u64,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::PlatformInitialize,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::PlatformInitialize,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeStandaloneAsync", (appID, accessToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMessageHandle(
        messageHandle: crate::System::IntPtr,
        messageType: crate::Oculus::Platform::Message_MessageType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMessageHandle", (messageHandle, messageType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::PlatformInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal_HTTP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_HTTP {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "HTTP";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal_HTTP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal_HTTP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl crate::Oculus::Platform::PlatformInternal_HTTP {
    pub fn SetHttpTransferUpdateCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::HttpTransferUpdate,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetHttpTransferUpdateCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+HTTP")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::PlatformInternal_HTTP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformInternal_MessageTypeInternal {
    #[default]
    AbuseReport_LaunchAdvancedReportFlow = 2973396720u32,
    Application_ExecuteCoordinatedLaunch = 2109011184u32,
    Application_GetInstalledApplications = 259280112u32,
    Avatar_UpdateMetaData = 3487141616u32,
    Cal_FinalizeApplication = 3586894301u32,
    Cal_GetSuggestedApplications = 1886393840u32,
    Cal_ProposeApplication = 2213731824u32,
    Colocation_GetCurrentMapUuid = 1434366704u32,
    Colocation_RequestMap = 359034352u32,
    Colocation_ShareMap = 3720637912u32,
    DeviceApplicationIntegrity_GetAttestationToken = 3735236560u32,
    GraphAPI_Get = 4278218480u32,
    GraphAPI_Post = 2779235568u32,
    HTTP_Get = 3056739312u32,
    HTTP_GetToFile = 2178701808u32,
    HTTP_MultiPartPost = 1121063152u32,
    HTTP_Post = 916803568u32,
    Livestreaming_IsAllowedForApplication = 1988980171u32,
    Livestreaming_StartPartyStream = 794615024u32,
    Livestreaming_StartStream = 449298160u32,
    Livestreaming_StopPartyStream = 1729059056u32,
    Livestreaming_StopStream = 3826109168u32,
    Livestreaming_UpdateMicStatus = 2273138652u32,
    NetSync_Connect = 1837457392u32,
    NetSync_Disconnect = 3053349333u32,
    NetSync_GetSessions = 3590993392u32,
    NetSync_GetVoipAttenuation = 399125201u32,
    NetSync_GetVoipAttenuationDefault = 2074648816u32,
    NetSync_SetVoipAttenuation = 2547513072u32,
    NetSync_SetVoipAttenuationModel = 2494402288u32,
    NetSync_SetVoipChannelCfg = 2510615536u32,
    NetSync_SetVoipGroup = 312250096u32,
    NetSync_SetVoipListentoChannels = 3505009392u32,
    NetSync_SetVoipMicSource = 49770736u32,
    NetSync_SetVoipSessionMuted = 2248084208u32,
    NetSync_SetVoipSpeaktoChannels = 2949502448u32,
    NetSync_SetVoipStreamMode = 3785177072u32,
    Party_Create = 1327223770u32,
    Party_GatherInApplication = 2277606384u32,
    Party_Get = 2303966704u32,
    Party_GetCurrentForUser = 3422497520u32,
    Party_Invite = 3049579504u32,
    Party_Join = 41710576u32,
    Party_Leave = 2449920496u32,
    RichPresence_SetDestination = 853609968u32,
    RichPresence_SetIsJoinable = 2602525168u32,
    RichPresence_SetLobbySession = 17373168u32,
    RichPresence_SetMatchSession = 3757870832u32,
    Room_CreateOrUpdateAndJoinNamed = 2383057392u32,
    Room_GetNamedRooms = 2356051399u32,
    Room_GetSocialRooms = 2283632368u32,
    User_CancelRecordingForReportFlow = 1238491331u32,
    User_GetLinkedAccounts = 2482263792u32,
    User_GetUserCapabilities = 2083593426u32,
    User_LaunchReportFlow = 1654657520u32,
    User_LaunchReportFlow2 = 2203608048u32,
    User_NewEntitledTestUser = 52393169u32,
    User_NewTestUser = 3897527536u32,
    User_NewTestUserFriends = 3341211614u32,
    User_StartRecordingForReportFlow = 1848894448u32,
    User_StopRecordingAndLaunchReportFlow = 2022476784u32,
    User_StopRecordingAndLaunchReportFlow2 = 733201113u32,
    User_TestUserCreateDeviceManifest = 1890762224u32,
    Voip_ReportAppVoipSessions = 3461436120u32,
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "MessageTypeInternal";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+MessageTypeInternal")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::PlatformInternal_MessageTypeInternal {
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInternal_Users {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::PlatformInternal_Users {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Users";
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
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformInternal_Users {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformInternal_Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl crate::Oculus::Platform::PlatformInternal_Users {
    pub fn GetLinkedAccounts(
        providers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::Oculus::Platform::ServiceProvider>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LinkedAccountList,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LinkedAccountList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLinkedAccounts", (providers))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+PlatformInternal+Users")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::PlatformInternal_Users {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
