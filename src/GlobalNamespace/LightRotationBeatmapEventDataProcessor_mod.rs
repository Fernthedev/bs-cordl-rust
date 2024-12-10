#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBeatmapEventDataProcessor {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataProcessor_1<
        *mut crate::GlobalNamespace::LightRotationBeatmapEventData,
    >,
}
#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightRotationBeatmapEventDataProcessor => ""
    ."LightRotationBeatmapEventDataProcessor"
);
#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::LightRotationBeatmapEventDataProcessor {
    type Target = crate::GlobalNamespace::BeatmapEventDataProcessor_1<
        *mut crate::GlobalNamespace::LightRotationBeatmapEventData,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightRotationBeatmapEventDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
impl crate::GlobalNamespace::LightRotationBeatmapEventDataProcessor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBeforeDeleteEventDataInternal(
        &mut self,
        nodeToDelete: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBeforeDeleteEventDataInternal", (nodeToDelete))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInsertedEventDataInternal(
        &mut self,
        insertedNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInsertedEventDataInternal", (insertedNode))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "LightRotationBeatmapEventDataProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightRotationBeatmapEventDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
