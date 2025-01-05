#[cfg(feature = "AlphaSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphaSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub alphaValue: f32,
}
#[cfg(feature = "AlphaSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AlphaSO => ""."AlphaSO"
);
#[cfg(feature = "AlphaSO")]
impl std::ops::Deref for crate::GlobalNamespace::AlphaSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphaSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AlphaSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphaSO")]
impl crate::GlobalNamespace::AlphaSO {
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
    pub fn op_Implicit(
        obj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AlphaSO>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (obj))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AlphaSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AlphaSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
