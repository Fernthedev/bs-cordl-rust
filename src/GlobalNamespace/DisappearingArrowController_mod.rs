#[cfg(feature = "DisappearingArrowController")]
#[repr(C)]
#[derive(Debug)]
pub struct DisappearingArrowController {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameNoteController>,
    >,
    pub _gameNoteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameNoteController,
    >,
}
#[cfg(feature = "DisappearingArrowController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DisappearingArrowController =>
    ""."DisappearingArrowController"
);
#[cfg(feature = "DisappearingArrowController")]
impl std::ops::Deref for crate::GlobalNamespace::DisappearingArrowController {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameNoteController>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DisappearingArrowController")]
impl std::ops::DerefMut for crate::GlobalNamespace::DisappearingArrowController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DisappearingArrowController")]
impl crate::GlobalNamespace::DisappearingArrowController {
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
    pub fn get_gameNoteController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameNoteController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameNoteController,
        > = __cordl_object.invoke("get_gameNoteController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DisappearingArrowController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DisappearingArrowController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
