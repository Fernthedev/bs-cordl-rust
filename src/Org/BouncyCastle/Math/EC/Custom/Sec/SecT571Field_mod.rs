#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
#[repr(C)]
#[derive(Debug)]
pub struct SecT571Field {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Custom::Sec::SecT571Field =>
    "Org.BouncyCastle.Math.EC.Custom.Sec"."SecT571Field"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecT571Field {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecT571Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecT571Field {
    pub const M59: u64 = 576460752303423487u64;
    pub const RM: u64 = 17256631552825064414u64;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecT571Field")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecT571Field {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
