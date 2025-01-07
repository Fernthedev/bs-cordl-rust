#[cfg(feature = "TMPro+FaceInfo_Legacy")]
#[repr(C)]
#[derive(Debug)]
pub struct FaceInfo_Legacy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PointSize: f32,
    pub Scale: f32,
    pub CharacterCount: i32,
    pub LineHeight: f32,
    pub Baseline: f32,
    pub Ascender: f32,
    pub CapHeight: f32,
    pub Descender: f32,
    pub CenterLine: f32,
    pub SuperscriptOffset: f32,
    pub SubscriptOffset: f32,
    pub SubSize: f32,
    pub Underline: f32,
    pub UnderlineThickness: f32,
    pub strikethrough: f32,
    pub strikethroughThickness: f32,
    pub TabWidth: f32,
    pub Padding: f32,
    pub AtlasWidth: f32,
    pub AtlasHeight: f32,
}
#[cfg(feature = "TMPro+FaceInfo_Legacy")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::FaceInfo_Legacy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "FaceInfo_Legacy";
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
#[cfg(feature = "TMPro+FaceInfo_Legacy")]
impl std::ops::Deref for crate::TMPro::FaceInfo_Legacy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+FaceInfo_Legacy")]
impl std::ops::DerefMut for crate::TMPro::FaceInfo_Legacy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+FaceInfo_Legacy")]
impl crate::TMPro::FaceInfo_Legacy {
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
}
#[cfg(feature = "TMPro+FaceInfo_Legacy")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::FaceInfo_Legacy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
