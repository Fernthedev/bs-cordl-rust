#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct AesWrapEngine {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Engines::AesWrapEngine =>
    "Org.BouncyCastle.Crypto.Engines"."AesWrapEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::AesWrapEngine {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::AesWrapEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::AesWrapEngine {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+AesWrapEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::AesWrapEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
