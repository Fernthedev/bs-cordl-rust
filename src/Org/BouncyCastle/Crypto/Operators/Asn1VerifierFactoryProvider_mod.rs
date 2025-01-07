#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1VerifierFactoryProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub publicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "Asn1VerifierFactoryProvider";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    pub fn CreateVerifierFactory(
        &mut self,
        algorithmDetails: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IVerifierFactory>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IVerifierFactory,
        > = __cordl_object.invoke("CreateVerifierFactory", (algorithmDetails))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider {
        unsafe { std::mem::transmute(self) }
    }
}
