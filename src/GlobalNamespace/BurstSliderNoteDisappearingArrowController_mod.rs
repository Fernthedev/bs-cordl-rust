#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstSliderNoteDisappearingArrowController {
    __cordl_parent: crate::GlobalNamespace::DisappearingArrowControllerBase_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >,
    pub _burstSliderNoteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BurstSliderGameNoteController,
    >,
}
#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BurstSliderNoteDisappearingArrowController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BurstSliderNoteDisappearingArrowController";
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
#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
impl std::ops::Deref
for crate::GlobalNamespace::BurstSliderNoteDisappearingArrowController {
    type Target = crate::GlobalNamespace::DisappearingArrowControllerBase_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BurstSliderNoteDisappearingArrowController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
impl crate::GlobalNamespace::BurstSliderNoteDisappearingArrowController {
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
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BurstSliderGameNoteController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BurstSliderGameNoteController,
        > = __cordl_object.invoke("get_gameNoteController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BurstSliderNoteDisappearingArrowController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BurstSliderNoteDisappearingArrowController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
