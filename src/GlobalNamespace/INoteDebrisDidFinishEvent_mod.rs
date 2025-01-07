#[cfg(feature = "INoteDebrisDidFinishEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteDebrisDidFinishEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "INoteDebrisDidFinishEvent";
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
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl std::ops::Deref for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    pub fn HandleNoteDebrisDidFinish(
        &mut self,
        noteDebris: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteDebris>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDebrisDidFinish", (noteDebris))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INoteDebrisDidFinishEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteDebrisDidFinishEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
