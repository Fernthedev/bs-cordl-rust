#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspRequest {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub tbsRequest: *mut crate::Org::BouncyCastle::Asn1::Ocsp::TbsRequest,
    pub optionalSignature: *mut crate::Org::BouncyCastle::Asn1::Ocsp::Signature,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Ocsp::OcspRequest =>
    "Org.BouncyCastle.Asn1.Ocsp"."OcspRequest"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Ocsp::OcspRequest {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Ocsp::OcspRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
impl crate::Org::BouncyCastle::Asn1::Ocsp::OcspRequest {
    pub fn New_Asn1Sequence1(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TbsRequest_Signature0(
        tbsRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::TbsRequest,
        >,
        optionalSignature: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::Signature,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tbsRequest, optionalSignature))?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TbsRequest_Signature0(
        &mut self,
        tbsRequest: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::TbsRequest,
        >,
        optionalSignature: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::Signature,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tbsRequest, optionalSignature))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OptionalSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Ocsp::Signature>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::Signature,
        > = __cordl_object.invoke("get_OptionalSignature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TbsRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Ocsp::TbsRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Ocsp::TbsRequest,
        > = __cordl_object.invoke("get_TbsRequest", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Ocsp::OcspRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
