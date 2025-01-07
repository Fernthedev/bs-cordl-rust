#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollViewItemsVisibilityController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _viewport: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _contentRectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub _items: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::HMUI::ScrollViewItemForVisibilityController>,
        >,
    >,
    pub _lastContentAnchoredPositionY: f32,
    pub _viewportWorldCorners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub _upperItemsCornes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Tuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::HMUI::ScrollViewItemForVisibilityController,
                    >,
                    f32,
                >,
            >,
        >,
    >,
    pub _lowerItemsCornes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Tuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::HMUI::ScrollViewItemForVisibilityController,
                    >,
                    f32,
                >,
            >,
        >,
    >,
    pub _lowerLastVisibleIndex: i32,
    pub _upperLastVisibleIndex: i32,
    pub _contentMaxY: f32,
    pub _contentMinY: f32,
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HMUI::ScrollViewItemsVisibilityController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ScrollViewItemsVisibilityController";
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
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl std::ops::Deref for crate::HMUI::ScrollViewItemsVisibilityController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl std::ops::DerefMut for crate::HMUI::ScrollViewItemsVisibilityController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl crate::HMUI::ScrollViewItemsVisibilityController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibilityDownDirection(
        &mut self,
        newContentAnchoredPositionY: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibilityDownDirection", (newContentAnchoredPositionY))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibilityUpDirection(
        &mut self,
        newContentAnchoredPositionY: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibilityUpDirection", (newContentAnchoredPositionY))?;
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
#[cfg(feature = "HMUI+ScrollViewItemsVisibilityController")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ScrollViewItemsVisibilityController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
