#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct ECGost3410NamedCurves {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::CryptoPro::ECGost3410NamedCurves {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.CryptoPro";
    const CLASS_NAME: &'static str = "ECGost3410NamedCurves";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::CryptoPro::ECGost3410NamedCurves {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::CryptoPro::ECGost3410NamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
impl crate::Org::BouncyCastle::Asn1::CryptoPro::ECGost3410NamedCurves {
    pub fn ConfigureBasepoint(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConfigureBasepoint", (curve, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCurve(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConfigureCurve", (curve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByNameX9(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByNameX9", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByOid(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByOid", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByOidX9(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByOidX9", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetName", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOid(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetOid", (name))?;
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
    pub fn get_Names() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Names", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+ECGost3410NamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::CryptoPro::ECGost3410NamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
