#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackClipTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub inspectedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub allowAutoCreate: bool,
}
#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TrackClipTypeAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TrackClipTypeAttribute";
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
#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackClipTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackClipTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
impl crate::UnityEngine::Timeline::TrackClipTypeAttribute {
    pub fn New_Type0(
        clipClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clipClass))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        clipClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allowAutoCreate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clipClass, allowAutoCreate))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Type0(
        &mut self,
        clipClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clipClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        clipClass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allowAutoCreate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clipClass, allowAutoCreate))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackClipTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackClipTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
