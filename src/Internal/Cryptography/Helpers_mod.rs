#[cfg(feature = "Internal+Cryptography+Helpers")]
#[repr(C)]
#[derive(Debug)]
pub struct Helpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Cryptography::Helpers =>
    "Internal.Cryptography"."Helpers"
);
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl std::ops::Deref for crate::Internal::Cryptography::Helpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl std::ops::DerefMut for crate::Internal::Cryptography::Helpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl crate::Internal::Cryptography::Helpers {
    pub fn CloneByteArray(
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloneByteArray", (src))?;
        Ok(__cordl_ret.into())
    }
    pub fn NibbleToHex(b: u8) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NibbleToHex", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHexArrayUpper(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHexArrayUpper", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHexStringUpper(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHexStringUpper", (bytes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl quest_hook::libil2cpp::ObjectType for crate::Internal::Cryptography::Helpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
