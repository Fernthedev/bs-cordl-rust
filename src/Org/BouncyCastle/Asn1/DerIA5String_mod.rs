#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
#[repr(C)]
#[derive(Debug)]
pub struct DerIA5String {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerStringBase,
    >,
    pub str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::DerIA5String =>
    "Org.BouncyCastle.Asn1"."DerIA5String"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::DerIA5String {
    type Target = quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerStringBase,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::DerIA5String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
impl crate::Org::BouncyCastle::Asn1::DerIA5String {
    pub fn Asn1Equals(
        &mut self,
        asn1Object: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Asn1Equals", (asn1Object))?;
        Ok(__cordl_ret.into())
    }
    pub fn Asn1GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Asn1GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        derOut: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerOutputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (derOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Gc0(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerIA5String>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerIA5String,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetInstance", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance__cordl_bool1(
        obj: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1TaggedObject>,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerIA5String>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerIA5String,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (obj, isExplicit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOctets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetOctets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIA5String(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIA5String", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (str))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (str))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool2(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        validate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (str, validate))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc0(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool2(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        validate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (str, validate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+DerIA5String")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::DerIA5String {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
