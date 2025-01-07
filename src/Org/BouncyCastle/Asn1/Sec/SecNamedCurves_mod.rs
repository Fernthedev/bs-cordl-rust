#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "SecNamedCurves";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves {
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
    pub type Secp112r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
    pub type Secp112r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
    pub type Secp128r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
    pub type Secp128r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
    pub type Secp160k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
    pub type Secp160r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
    pub type Secp160r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
    pub type Secp192k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
    pub type Secp192r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
    pub type Secp224k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
    pub type Secp224r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
    pub type Secp256k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
    pub type Secp256r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
    pub type Secp384r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
    pub type Secp521r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
    pub type Sect113r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
    pub type Sect113r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
    pub type Sect131r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
    pub type Sect131r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
    pub type Sect163k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
    pub type Sect163r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
    pub type Sect163r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
    pub type Sect193r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
    pub type Sect193r2Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
    pub type Sect233k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
    pub type Sect233r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
    pub type Sect239k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
    pub type Sect283k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
    pub type Sect283r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
    pub type Sect409k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
    pub type Sect409r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
    pub type Sect571k1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder;
    #[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
    pub type Sect571r1Holder = crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder;
    pub fn ConfigureBasepoint(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConfigureBasepoint", (curve, encoding))?;
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
    pub fn ConfigureCurveGlv(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeBParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConfigureCurveGlv", (c, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefineCurve(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        holder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefineCurve", (name, oid, holder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHex(
        hex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromHex", (hex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetByOid(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetByOid", (oid))?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp112r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp112r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp112r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp112r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp112r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp112r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp128r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp128r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp128r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp128r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp128r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp128r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp160k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp160k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp160r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp160r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp160r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp160r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp160r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp160r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp192k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp192k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp192r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp192r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp192r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp192r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp224k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp224k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp224r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp224r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp224r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp224r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp256k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp256k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp256r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp256r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp256r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp256r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp384r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp384r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp384r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp384r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Secp521r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Secp521r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Secp521r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Secp521r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect113r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect113r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder {
    pub const k: i32 = 9i32;
    pub const m: i32 = 113i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect113r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect113r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder {
    pub const k: i32 = 9i32;
    pub const m: i32 = 113i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect113r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect113r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect131r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect131r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder {
    pub const k1: i32 = 2i32;
    pub const k2: i32 = 3i32;
    pub const k3: i32 = 8i32;
    pub const m: i32 = 131i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect131r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect131r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder {
    pub const k1: i32 = 2i32;
    pub const k2: i32 = 3i32;
    pub const k3: i32 = 8i32;
    pub const m: i32 = 131i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect131r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect131r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect163k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect163k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder {
    pub const k1: i32 = 3i32;
    pub const k2: i32 = 6i32;
    pub const k3: i32 = 7i32;
    pub const m: i32 = 163i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect163r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect163r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder {
    pub const k1: i32 = 3i32;
    pub const k2: i32 = 6i32;
    pub const k3: i32 = 7i32;
    pub const m: i32 = 163i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect163r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect163r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder {
    pub const k1: i32 = 3i32;
    pub const k2: i32 = 6i32;
    pub const k3: i32 = 7i32;
    pub const m: i32 = 163i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect163r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect163r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect193r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect193r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder {
    pub const k: i32 = 15i32;
    pub const m: i32 = 193i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect193r2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect193r2Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder {
    pub const k: i32 = 15i32;
    pub const m: i32 = 193i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect193r2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect193r2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect233k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect233k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder {
    pub const k: i32 = 74i32;
    pub const m: i32 = 233i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect233r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect233r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder {
    pub const k: i32 = 74i32;
    pub const m: i32 = 233i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect233r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect233r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect239k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect239k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder {
    pub const k: i32 = 158i32;
    pub const m: i32 = 239i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect239k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect239k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect283k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect283k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder {
    pub const k1: i32 = 5i32;
    pub const k2: i32 = 7i32;
    pub const k3: i32 = 12i32;
    pub const m: i32 = 283i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect283r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect283r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder {
    pub const k1: i32 = 5i32;
    pub const k2: i32 = 7i32;
    pub const k3: i32 = 12i32;
    pub const m: i32 = 283i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect283r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect283r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect409k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect409k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder {
    pub const k: i32 = 87i32;
    pub const m: i32 = 409i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect409r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect409r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder {
    pub const k: i32 = 87i32;
    pub const m: i32 = 409i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect409r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect409r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect571k1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect571k1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder {
    pub const k1: i32 = 2i32;
    pub const k2: i32 = 5i32;
    pub const k3: i32 = 10i32;
    pub const m: i32 = 571i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571k1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571k1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct SecNamedCurves_Sect571r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Sec";
    const CLASS_NAME: &'static str = "Sect571r1Holder";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
impl crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder {
    pub const k1: i32 = 2i32;
    pub const k2: i32 = 5i32;
    pub const k3: i32 = 10i32;
    pub const m: i32 = 571i32;
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = __cordl_object.invoke("CreateParameters", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Sec+SecNamedCurves+Sect571r1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Sec::SecNamedCurves_Sect571r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
