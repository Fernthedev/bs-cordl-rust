#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_HAPI_AssetInfo_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::Test_HAPI_AssetInfo_Extensions => "HoudiniEngineUnity"
    ."Test_HAPI_AssetInfo_Extensions"
);
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_HAPI_AssetInfo_Extensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_HAPI_AssetInfo_Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
impl crate::HoudiniEngineUnity::Test_HAPI_AssetInfo_Extensions {
    pub fn ToTestObject(
        _cordl_self: crate::HoudiniEngineUnity::HAPI_AssetInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_HAPI_AssetInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::Test_HAPI_AssetInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTestObject", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_HAPI_AssetInfo_Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::Test_HAPI_AssetInfo_Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
