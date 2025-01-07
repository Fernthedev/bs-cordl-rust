#[cfg(feature = "FileStorageExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FileStorageExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "FileStorageExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FileStorageExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FileStorageExtensions";
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
#[cfg(feature = "FileStorageExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::FileStorageExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileStorageExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileStorageExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileStorageExtensions")]
impl crate::GlobalNamespace::FileStorageExtensions {
    pub const kSizeInBytesUntilDeserializeWarning: i32 = 10000i32;
    pub fn DeleteFile(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteFile", (fileStorage, fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileExists(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FileExists", (fileStorage, fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFile(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFile", (fileStorage, fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromJSONFile<T>(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromJSONFile", (fileStorage, fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromJSONFileAsync<T>(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromJSONFileAsync", (fileStorage, fileName, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveFile(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveFile", (fileStorage, fileName, value, storageLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveToJSONFile(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
        overrideSerializerSettings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SaveToJSONFile",
                (fileStorage, obj, fileName, storageLocation, overrideSerializerSettings),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveToJSONFileAsync(
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        storageLocation: crate::GlobalNamespace::StoragePreference,
        overrideSerializerSettings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SaveToJSONFileAsync",
                (fileStorage, obj, fileName, storageLocation, overrideSerializerSettings),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileStorageExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileStorageExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
