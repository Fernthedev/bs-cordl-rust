#[cfg(feature = "OVRSimpleJSON+JSON")]
#[repr(C)]
#[derive(Debug)]
pub struct JSON {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVRSimpleJSON::JSON {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVRSimpleJSON";
    const CLASS_NAME: &'static str = "JSON";
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
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl std::ops::Deref for crate::OVRSimpleJSON::JSON {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl std::ops::DerefMut for crate::OVRSimpleJSON::JSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl crate::OVRSimpleJSON::JSON {
    pub fn Parse(
        aJSON: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVRSimpleJSON::JSONNode> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (aJSON))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl quest_hook::libil2cpp::ObjectType for crate::OVRSimpleJSON::JSON {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
