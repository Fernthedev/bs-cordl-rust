#[cfg(feature = "BloomPrePassEffectContainerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassEffectContainerSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _bloomPrePassEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectSO,
    >,
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassEffectContainerSO
    => ""."BloomPrePassEffectContainerSO"
);
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassEffectContainerSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassEffectContainerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl crate::GlobalNamespace::BloomPrePassEffectContainerSO {
    pub fn Init(
        &mut self,
        bloomPrePassEffect: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassEffectSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (bloomPrePassEffect))?;
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
    pub fn get_bloomPrePassEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassEffectSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassEffectSO,
        > = __cordl_object.invoke("get_bloomPrePassEffect", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassEffectContainerSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassEffectContainerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
