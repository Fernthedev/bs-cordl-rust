#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorNoteData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem,
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub c: crate::BeatmapSaveDataCommon::NoteColorType,
    pub d: crate::BeatmapSaveDataCommon::NoteCutDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::ColorNoteData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "ColorNoteData";
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
#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::ColorNoteData {
    type Target = crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::ColorNoteData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
impl crate::BeatmapSaveDataVersion3::ColorNoteData {
    pub fn New(
        beat: f32,
        line: i32,
        layer: i32,
        color: crate::BeatmapSaveDataCommon::NoteColorType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        angleOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beat, line, layer, color, cutDirection, angleOffset),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        line: i32,
        layer: i32,
        color: crate::BeatmapSaveDataCommon::NoteColorType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        angleOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, line, layer, color, cutDirection, angleOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angleOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_angleOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteColorType = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = __cordl_object
            .invoke("get_cutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layer(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_line(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_line", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+ColorNoteData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::ColorNoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
