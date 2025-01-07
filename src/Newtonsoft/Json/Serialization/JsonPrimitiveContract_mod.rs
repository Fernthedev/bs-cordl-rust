#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonPrimitiveContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContract,
    pub _TypeCode_k__BackingField: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonPrimitiveContract";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
impl crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract {
    pub fn New(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TypeCode(
        &mut self,
        value: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeCode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonPrimitiveContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
