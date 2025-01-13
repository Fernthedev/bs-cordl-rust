#[cfg(feature = "PS5SharedPackageSKUsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5SharedPackageSKUsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _availableSKUs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS5PublisherSKUSettingsSO>,
        >,
    >,
    pub _conceptId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _buildType: crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType,
    pub _buildVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
    >,
    pub _latestBuildVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
    >,
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS5SharedPackageSKUsSO";
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
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl std::ops::Deref for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    #[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
    pub type BuildType = crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType;
    #[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
    pub type PS5BuildVersion = crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion;
    pub fn GetPrimarySKU(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PS5PublisherSKUSettingsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
        > = __cordl_object.invoke("GetPrimarySKU", ())?;
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
    pub fn get_availableSKUs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PS5PublisherSKUSettingsSO,
                >,
            >,
        > = __cordl_object.invoke("get_availableSKUs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buildType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType = __cordl_object
            .invoke("get_buildType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
        > = __cordl_object.invoke("get_buildVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_conceptId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_conceptId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_latestBuildVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
        > = __cordl_object.invoke("get_latestBuildVersion", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5SharedPackageSKUsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PS5SharedPackageSKUsSO_BuildType {
    #[default]
    Application = 0i32,
    RemasterPatch = 1i32,
}
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BuildType";
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
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType {
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
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType {
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
#[cfg(feature = "PS5SharedPackageSKUsSO+BuildType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_BuildType {
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
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5SharedPackageSKUsSO_PS5BuildVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _masterVersion: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyVersion>,
    pub _contentVersion: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyContentVersion,
    >,
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PS5SharedPackageSKUsSO/PS5BuildVersion";
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
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl std::ops::Deref for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    pub fn CopyValueFrom(
        &mut self,
        newVersion: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValueFrom", (newVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseContentVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseContentVersion", ())?;
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
    pub fn get_contentVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyContentVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyContentVersion,
        > = __cordl_object.invoke("get_contentVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_masterVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SonyVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyVersion,
        > = __cordl_object.invoke("get_masterVersion", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS5SharedPackageSKUsSO+PS5BuildVersion")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5SharedPackageSKUsSO_PS5BuildVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
