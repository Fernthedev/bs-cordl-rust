#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
#[repr(C)]
#[derive(Debug)]
pub struct Datatype_tokenV1Compat {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::Datatype_normalizedStringV1Compat,
    >,
}
#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Datatype_tokenV1Compat =>
    "System.Xml.Schema"."Datatype_tokenV1Compat"
);
#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
impl std::ops::Deref for crate::System::Xml::Schema::Datatype_tokenV1Compat {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::Datatype_normalizedStringV1Compat,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Datatype_tokenV1Compat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
impl crate::System::Xml::Schema::Datatype_tokenV1Compat {
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
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlTypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+Datatype_tokenV1Compat")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Datatype_tokenV1Compat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
