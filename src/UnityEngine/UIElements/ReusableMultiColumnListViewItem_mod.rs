#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableMultiColumnListViewItem {
    __cordl_parent: crate::UnityEngine::UIElements::ReusableListViewItem,
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ReusableMultiColumnListViewItem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ReusableMultiColumnListViewItem";
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
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ReusableMultiColumnListViewItem {
    type Target = crate::UnityEngine::UIElements::ReusableListViewItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ReusableMultiColumnListViewItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
impl crate::UnityEngine::UIElements::ReusableMultiColumnListViewItem {
    pub fn Init_Columns__cordl_bool1(
        &mut self,
        container: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
        usesAnimatedDrag: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (container, columns, usesAnimatedDrag))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_VisualElement0(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (item))?;
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
    pub fn get_rootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_rootElement", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableMultiColumnListViewItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ReusableMultiColumnListViewItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
