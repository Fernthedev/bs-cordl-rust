#[cfg(feature = "ICertificateValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ICertificateValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ICertificateValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ICertificateValidator => ""
    ."ICertificateValidator"
);
#[cfg(feature = "ICertificateValidator")]
impl std::ops::Deref for crate::GlobalNamespace::ICertificateValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ICertificateValidator")]
impl std::ops::DerefMut for crate::GlobalNamespace::ICertificateValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ICertificateValidator")]
impl crate::GlobalNamespace::ICertificateValidator {
    pub fn ValidateCertificateChain(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DnsEndPoint>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateCertificateChain",
                (endPoint, certificate, certificateChain),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ICertificateValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ICertificateValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
