#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryProfiler {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::Memory::MemoryProfiler =>
    "Unity.Profiling.Memory"."MemoryProfiler"
);
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl std::ops::Deref for crate::Unity::Profiling::Memory::MemoryProfiler {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl std::ops::DerefMut for crate::Unity::Profiling::Memory::MemoryProfiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl crate::Unity::Profiling::Memory::MemoryProfiler {
    pub fn FinalizeSnapshot(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FinalizeSnapshot", (path, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareMetadata() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareMetadata", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveScreenshotToDisk(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: bool,
        pixelsPtr: crate::System::IntPtr,
        pixelsCount: i32,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SaveScreenshotToDisk",
                (path, result, pixelsPtr, pixelsCount, format, width, height),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIntToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteIntToByteArray", (array, offset, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringToByteArray(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteStringToByteArray", (array, offset, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+Memory+MemoryProfiler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::Memory::MemoryProfiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
