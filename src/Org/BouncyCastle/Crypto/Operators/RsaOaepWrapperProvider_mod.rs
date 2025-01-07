#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaOaepWrapperProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub digestOid: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "RsaOaepWrapperProvider";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    pub fn New(
        digestOid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digestOid))?;
        Ok(__cordl_object.into())
    }
    pub fn Org_BouncyCastle_Crypto_Operators_WrapperProvider_CreateWrapper(
        &mut self,
        forWrapping: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke(
                "Org.BouncyCastle.Crypto.Operators.WrapperProvider.CreateWrapper",
                (forWrapping, parameters),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        digestOid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digestOid))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Operators::WrapperProvider>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Operators::WrapperProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Operators::WrapperProvider>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Operators::WrapperProvider {
        unsafe { std::mem::transmute(self) }
    }
}
