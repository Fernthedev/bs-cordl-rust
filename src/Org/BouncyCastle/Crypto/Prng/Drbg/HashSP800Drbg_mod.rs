#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
#[repr(C)]
#[derive(Debug)]
pub struct HashSP800Drbg {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mEntropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    pub mSecurityStrength: i32,
    pub mSeedLength: i32,
    pub mV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mC: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mReseedCounter: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::Drbg::HashSP800Drbg =>
    "Org.BouncyCastle.Crypto.Prng.Drbg"."HashSP800Drbg"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HashSP800Drbg {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HashSP800Drbg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
impl crate::Org::BouncyCastle::Crypto::Prng::Drbg::HashSP800Drbg {
    pub fn AddTo(
        &mut self,
        longer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        shorter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTo", (longer, shorter))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoHash(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoHash", (input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Generate(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        additionalInput: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Generate", (output, additionalInput, predictionResistant))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEntropy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Hash(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("Hash", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        securityStrength: i32,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (digest, securityStrength, entropySource, personalizationString, nonce),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Reseed(
        &mut self,
        additionalInput: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reseed", (additionalInput))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        securityStrength: i32,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (digest, securityStrength, entropySource, personalizationString, nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn hashgen(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        lengthInBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("hashgen", (input, lengthInBits))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HashSP800Drbg")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HashSP800Drbg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
