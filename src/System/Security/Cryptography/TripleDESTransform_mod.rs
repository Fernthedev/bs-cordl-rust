#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct TripleDESTransform {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Cryptography::SymmetricTransform,
    >,
    pub E1: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D2: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub E3: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D1: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub E2: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D3: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::TripleDESTransform =>
    "System.Security.Cryptography"."TripleDESTransform"
);
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::TripleDESTransform {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Cryptography::SymmetricTransform,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::TripleDESTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl crate::System::Security::Cryptography::TripleDESTransform {
    pub fn ECB(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ECB", (input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStrongKey() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetStrongKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::TripleDES,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::TripleDES,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::TripleDESTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
