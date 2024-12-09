#[cfg(feature = "Mono+Security+PKCS7")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS7 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Security+PKCS7")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::PKCS7 => "Mono.Security"."PKCS7"
);
#[cfg(feature = "Mono+Security+PKCS7")]
impl std::ops::Deref for crate::Mono::Security::PKCS7 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7")]
impl std::ops::DerefMut for crate::Mono::Security::PKCS7 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7")]
impl crate::Mono::Security::PKCS7 {
    #[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
    pub type ContentInfo = crate::Mono::Security::PKCS7_ContentInfo;
    #[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
    pub type EncryptedData = crate::Mono::Security::PKCS7_EncryptedData;
    #[cfg(feature = "Mono+Security+PKCS7+SignedData")]
    pub type SignedData = crate::Mono::Security::PKCS7_SignedData;
    #[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
    pub type SignerInfo = crate::Mono::Security::PKCS7_SignerInfo;
}
#[cfg(feature = "Mono+Security+PKCS7")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::PKCS7 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS7_ContentInfo {
    __cordl_parent: crate::System::Object,
    pub contentType: *mut crate::System::String,
    pub content: *mut crate::Mono::Security::ASN1,
}
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::PKCS7_ContentInfo =>
    "Mono.Security"."PKCS7/ContentInfo"
);
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
impl std::ops::Deref for crate::Mono::Security::PKCS7_ContentInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
impl std::ops::DerefMut for crate::Mono::Security::PKCS7_ContentInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
impl crate::Mono::Security::PKCS7_ContentInfo {
    pub fn GetASN1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("GetASN1", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ASN1_3(
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asn1))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray2(
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ASN1_3(
        &mut self,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asn1))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn get_ASN1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("get_ASN1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("get_Content", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ContentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Content(
        &mut self,
        value: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Content", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ContentType(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContentType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+ContentInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::PKCS7_ContentInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS7_EncryptedData {
    __cordl_parent: crate::System::Object,
    pub _version: u8,
    pub _content: *mut crate::Mono::Security::PKCS7_ContentInfo,
    pub _encryptionAlgorithm: *mut crate::Mono::Security::PKCS7_ContentInfo,
    pub _encrypted: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::PKCS7_EncryptedData =>
    "Mono.Security"."PKCS7/EncryptedData"
);
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
impl std::ops::Deref for crate::Mono::Security::PKCS7_EncryptedData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
impl std::ops::DerefMut for crate::Mono::Security::PKCS7_EncryptedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
impl crate::Mono::Security::PKCS7_EncryptedData {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ASN1_1(
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asn1))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ASN1_1(
        &mut self,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asn1))?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptedContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_EncryptedContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::PKCS7_ContentInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::PKCS7_ContentInfo = __cordl_object
            .invoke("get_EncryptionAlgorithm", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+EncryptedData")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::PKCS7_EncryptedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS7_SignedData {
    __cordl_parent: crate::System::Object,
    pub version: u8,
    pub hashAlgorithm: *mut crate::System::String,
    pub contentInfo: *mut crate::Mono::Security::PKCS7_ContentInfo,
    pub certs: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub crls: *mut crate::System::Collections::ArrayList,
    pub signerInfo: *mut crate::Mono::Security::PKCS7_SignerInfo,
    pub mda: bool,
}
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::PKCS7_SignedData =>
    "Mono.Security"."PKCS7/SignedData"
);
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
impl std::ops::Deref for crate::Mono::Security::PKCS7_SignedData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
impl std::ops::DerefMut for crate::Mono::Security::PKCS7_SignedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
impl crate::Mono::Security::PKCS7_SignedData {
    pub fn New(
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asn1))?;
        Ok(__cordl_object)
    }
    pub fn OidToName(
        &mut self,
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("OidToName", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asn1))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509CertificateCollection = __cordl_object
            .invoke("get_Certificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::PKCS7_ContentInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::PKCS7_ContentInfo = __cordl_object
            .invoke("get_ContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::PKCS7_SignerInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::PKCS7_SignerInfo = __cordl_object
            .invoke("get_SignerInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_HashName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HashName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignedData")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::PKCS7_SignedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PKCS7_SignerInfo {
    __cordl_parent: crate::System::Object,
    pub version: u8,
    pub hashAlgorithm: *mut crate::System::String,
    pub authenticatedAttributes: *mut crate::System::Collections::ArrayList,
    pub unauthenticatedAttributes: *mut crate::System::Collections::ArrayList,
    pub signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub issuer: *mut crate::System::String,
    pub serial: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub ski: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::PKCS7_SignerInfo =>
    "Mono.Security"."PKCS7/SignerInfo"
);
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
impl std::ops::Deref for crate::Mono::Security::PKCS7_SignerInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
impl std::ops::DerefMut for crate::Mono::Security::PKCS7_SignerInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
impl crate::Mono::Security::PKCS7_SignerInfo {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_ASN1_1(
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (asn1))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ASN1_1(
        &mut self,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (asn1))?;
        Ok(__cordl_ret)
    }
    pub fn get_AuthenticatedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_AuthenticatedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HashName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_HashName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_IssuerName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Signature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Signature", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnauthenticatedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_UnauthenticatedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_HashName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HashName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+PKCS7+SignerInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::PKCS7_SignerInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
