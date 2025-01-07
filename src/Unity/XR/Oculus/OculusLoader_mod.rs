#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusLoader {
    __cordl_parent: crate::UnityEngine::XR::Management::XRLoaderHelper,
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::OculusLoader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "OculusLoader";
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
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl std::ops::Deref for crate::Unity::XR::Oculus::OculusLoader {
    type Target = crate::UnityEngine::XR::Management::XRLoaderHelper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::OculusLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl crate::Unity::XR::Oculus::OculusLoader {
    #[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
    pub type DeviceSupportedResult = crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult;
    pub fn CheckUnityVersionCompatibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckUnityVersionCompatibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deinitialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Deinitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::XR::Oculus::OculusSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::XR::Oculus::OculusSettings,
        > = __cordl_object.invoke("GetSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDeviceSupported() -> quest_hook::libil2cpp::Result<
        crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult,
    > {
        let __cordl_ret: crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDeviceSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RuntimeLoadOVRPlugin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RuntimeLoadOVRPlugin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Stop", ())?;
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
    pub fn get_displaySubsystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: Blacklisted = __cordl_object
            .invoke("get_displaySubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inputSubsystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRInputSubsystem,
        > = __cordl_object.invoke("get_inputSubsystem", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::OculusLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OculusLoader_DeviceSupportedResult {
    #[default]
    ExitApplication = 2i32,
    NotSupported = 1i32,
    Supported = 0i32,
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "DeviceSupportedResult";
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
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult {
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
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult {
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
#[cfg(feature = "Unity+XR+Oculus+OculusLoader+DeviceSupportedResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::XR::Oculus::OculusLoader_DeviceSupportedResult {
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
