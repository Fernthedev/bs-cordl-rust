#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::MemoryHelpers =>
    "UnityEngine.InputSystem.Utilities"."MemoryHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
    pub type BitRegion = crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion;
    pub fn AlignNatural(
        offset: u32,
        sizeInBytes: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignNatural", (offset, sizeInBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        ptr1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ptr2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        region: crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (ptr1, ptr2, region))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeFollowingByteOffset(
        byteOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeFollowingByteOffset", (byteOffset, sizeInBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCmpBitRegion(
        ptr1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ptr2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemCmpBitRegion", (ptr1, ptr2, bitOffset, bitCount, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCpyBitRegion(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemCpyBitRegion", (destination, source, bitOffset, bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemCpyMasked(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numBytes: i32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemCpyMasked", (destination, source, numBytes, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemSet(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numBytes: i32,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemSet", (destination, numBytes, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadExcessKMultipleBitsAsInt(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadExcessKMultipleBitsAsInt", (ptr, bitOffset, bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMultipleBitsAsNormalizedUInt(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadMultipleBitsAsNormalizedUInt", (ptr, bitOffset, bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadMultipleBitsAsUInt(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadMultipleBitsAsUInt", (ptr, bitOffset, bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadSingleBit(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadSingleBit", (ptr, bitOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadTwosComplementMultipleBitsAsInt(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadTwosComplementMultipleBitsAsInt", (ptr, bitOffset, bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBitsInBuffer(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byteOffset: i32,
        bitOffset: i32,
        sizeInBits: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetBitsInBuffer",
                (buffer, byteOffset, bitOffset, sizeInBits, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap<TValue>(
        a: quest_hook::libil2cpp::ByRefMut<TValue>,
        b: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Swap", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIntAsExcessKMultipleBits(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteIntAsExcessKMultipleBits", (ptr, bitOffset, bitCount, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteIntAsTwosComplementMultipleBits(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteIntAsTwosComplementMultipleBits",
                (ptr, bitOffset, bitCount, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteNormalizedUIntAsMultipleBits(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteNormalizedUIntAsMultipleBits",
                (ptr, bitOffset, bitCount, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSingleBit(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteSingleBit", (ptr, bitOffset, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUIntAsMultipleBits(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitOffset: u32,
        bitCount: u32,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUIntAsMultipleBits", (ptr, bitOffset, bitCount, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MemoryHelpers_BitRegion {
    pub bitOffset: u32,
    pub sizeInBits: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion =>
    "UnityEngine.InputSystem.Utilities"."MemoryHelpers/BitRegion"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MemoryHelpers+BitRegion")]
impl crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion {
    pub fn Overlap(
        &mut self,
        other: crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Overlap",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_1(
        &mut self,
        byteOffset: u32,
        bitOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (byteOffset, bitOffset, sizeInBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_0(
        &mut self,
        bitOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bitOffset, sizeInBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
