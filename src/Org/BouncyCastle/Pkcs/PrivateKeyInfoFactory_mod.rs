#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct PrivateKeyInfoFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkcs::PrivateKeyInfoFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkcs";
    const CLASS_NAME: &'static str = "PrivateKeyInfoFactory";
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
#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::PrivateKeyInfoFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::PrivateKeyInfoFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
impl crate::Org::BouncyCastle::Pkcs::PrivateKeyInfoFactory {
    pub fn CreatePrivateKeyInfo_AsymmetricKeyParameter0(
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrivateKeyInfo", (privateKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrivateKeyInfo_AsymmetricKeyParameter_Asn1Set1(
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        attributes: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Set>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrivateKeyInfo", (privateKey, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrivateKeyInfo_Il2CppArray_EncryptedPrivateKeyInfo2(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        encInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrivateKeyInfo", (passPhrase, encInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrivateKeyInfo_Il2CppArray__cordl_bool_EncryptedPrivateKeyInfo3(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        wrongPkcs12Zero: bool,
        encInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrivateKeyInfo", (passPhrase, wrongPkcs12Zero, encInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractBytes(
        encKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
        offSet: i32,
        bI: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractBytes", (encKey, _cordl_size, offSet, bI))?;
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
#[cfg(feature = "Org+BouncyCastle+Pkcs+PrivateKeyInfoFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::PrivateKeyInfoFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
