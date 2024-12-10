#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BerApplicationSpecificParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tag: i32,
    pub parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::BerApplicationSpecificParser => "Org.BouncyCastle.Asn1"
    ."BerApplicationSpecificParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    pub fn New(
        tag: i32,
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tag, parser))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
        > = __cordl_object.invoke("ReadObject", ())?;
        Ok(__cordl_ret.into())
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
    pub fn _ctor(
        &mut self,
        tag: i32,
        parser: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tag, parser))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser>
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser>
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1ApplicationSpecificParser {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl AsRef<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecificParser")]
impl AsMut<crate::Org::BouncyCastle::Asn1::IAsn1Convertible>
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecificParser {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible {
        unsafe { std::mem::transmute(self) }
    }
}
