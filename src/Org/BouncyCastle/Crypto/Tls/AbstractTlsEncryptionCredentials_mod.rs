#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsEncryptionCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCredentials,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "AbstractTlsEncryptionCredentials";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsCredentials;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    pub fn DecryptPreMasterSecret(
        &mut self,
        encryptedPreMasterSecret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("DecryptPreMasterSecret", (encryptedPreMasterSecret))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsEncryptionCredentials")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials>
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials {
        unsafe { std::mem::transmute(self) }
    }
}
