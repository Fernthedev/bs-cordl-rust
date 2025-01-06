#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Test_Terrain_Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Test_Terrain_Extensions =>
    "HoudiniEngineUnity"."Test_Terrain_Extensions"
);
#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
impl std::ops::Deref for crate::HoudiniEngineUnity::Test_Terrain_Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::Test_Terrain_Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
impl crate::HoudiniEngineUnity::Test_Terrain_Extensions {
    pub fn ToTestObject_Il2CppArray1(
        _cordl_self: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_Terrain>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_Terrain>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTestObject", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTestObject_List_1_2(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_Terrain>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_Terrain>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTestObject", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTestObject_Terrain0(
        _cordl_self: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::Test_Terrain>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::Test_Terrain,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTestObject", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+Test_Terrain_Extensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::Test_Terrain_Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
