#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeAsset {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_ImportedWithErrors: bool,
    pub m_ImportedWithWarnings: bool,
    pub m_Usings: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    >,
    pub inlineSheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub m_VisualElementAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElementAsset,
    >,
    pub m_TemplateAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::TemplateAsset,
    >,
    pub m_UxmlObjectEntries: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
    >,
    pub m_UxmlObjectIds: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_AssetEntries: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry,
    >,
    pub m_Slots: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
    >,
    pub m_ContentContainerId: i32,
    pub m_ContentHash: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualTreeAsset =>
    "UnityEngine.UIElements"."VisualTreeAsset"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeAsset {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualTreeAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl crate::UnityEngine::UIElements::VisualTreeAsset {
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
    pub type AssetEntry = crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
    pub type SlotDefinition = crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
    pub type SlotUsageEntry = crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
    pub type UsingEntry = crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
    pub type UsingEntryComparer = crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
    pub type UxmlObjectEntry = crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+__c__DisplayClass61_0")]
    pub type __c__DisplayClass61_0 = crate::UnityEngine::UIElements::VisualTreeAsset___c__DisplayClass61_0;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+__c__DisplayClass65_0")]
    pub type __c__DisplayClass65_0 = crate::UnityEngine::UIElements::VisualTreeAsset___c__DisplayClass65_0;
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+_get_stylesheets_d__23")]
    pub type _get_stylesheets_d__23 = crate::UnityEngine::UIElements::VisualTreeAsset__get_stylesheets_d__23;
    #[cfg(
        feature = "UnityEngine+UIElements+VisualTreeAsset+_get_templateDependencies_d__19"
    )]
    pub type _get_templateDependencies_d__19 = crate::UnityEngine::UIElements::VisualTreeAsset__get_templateDependencies_d__19;
    pub fn AssetEntryExists(
        &mut self,
        path: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AssetEntryExists", (path, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CloneSetupRecursively(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElementAsset,
        idToChildren: *mut crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::VisualElementAsset,
            >,
        >,
        context: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("CloneSetupRecursively", (root, idToChildren, context))?;
        Ok(__cordl_ret)
    }
    pub fn CloneTree_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::TemplateContainer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::TemplateContainer = __cordl_object
            .invoke("CloneTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloneTree_String1(
        &mut self,
        bindingPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::TemplateContainer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::TemplateContainer = __cordl_object
            .invoke("CloneTree", (bindingPath))?;
        Ok(__cordl_ret)
    }
    pub fn CloneTree_VisualElement2(
        &mut self,
        target: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloneTree", (target))?;
        Ok(__cordl_ret)
    }
    pub fn CloneTree_VisualElement_ByRefMut_ByRefMut3(
        &mut self,
        target: *mut crate::UnityEngine::UIElements::VisualElement,
        firstElementIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        elementAddedCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloneTree", (target, firstElementIndex, elementAddedCount))?;
        Ok(__cordl_ret)
    }
    pub fn CloneTree_VisualElement_Dictionary_2_List_1_4(
        &mut self,
        target: *mut crate::UnityEngine::UIElements::VisualElement,
        slotInsertionPoints: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
        attributeOverrides: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TemplateAsset_AttributeOverride,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloneTree", (target, slotInsertionPoints, attributeOverrides))?;
        Ok(__cordl_ret)
    }
    pub fn GetAsset<T>(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetAsset", (path))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextChildSerialNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNextChildSerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUxmlObjectEntry(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry = __cordl_object
            .invoke("GetUxmlObjectEntry", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetUxmlObjectFactory(
        &mut self,
        uxmlObjectAsset: *mut crate::UnityEngine::UIElements::UxmlObjectAsset,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IBaseUxmlObjectFactory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IBaseUxmlObjectFactory = __cordl_object
            .invoke("GetUxmlObjectFactory", (uxmlObjectAsset))?;
        Ok(__cordl_ret)
    }
    pub fn GetUxmlObjects<T>(
        &mut self,
        asset: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<T> = __cordl_object
            .invoke("GetUxmlObjects", (asset, cc))?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::TemplateContainer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::TemplateContainer = __cordl_object
            .invoke("Instantiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate_String1(
        &mut self,
        bindingPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::TemplateContainer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::TemplateContainer = __cordl_object
            .invoke("Instantiate", (bindingPath))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterAssetEntry(
        &mut self,
        path: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        asset: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAssetEntry", (path, _cordl_type, asset))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterUxmlObject(
        &mut self,
        uxmlObjectAsset: *mut crate::UnityEngine::UIElements::UxmlObjectAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterUxmlObject", (uxmlObjectAsset))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveTemplate(
        &mut self,
        templateName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualTreeAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualTreeAsset = __cordl_object
            .invoke("ResolveTemplate", (templateName))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetSlotInsertionPoint(
        &mut self,
        insertionPointId: i32,
        slotName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSlotInsertionPoint", (insertionPointId, slotName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentContainerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_contentContainerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_contentHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_importedWithErrors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_importedWithErrors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_importedWithWarnings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_importedWithWarnings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_slots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
        > = __cordl_object.invoke("get_slots", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stylesheets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::StyleSheet,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::StyleSheet,
        > = __cordl_object.invoke("get_stylesheets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_templateAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::TemplateAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::TemplateAsset,
        > = __cordl_object.invoke("get_templateAssets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_templateDependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::VisualTreeAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::VisualTreeAsset,
        > = __cordl_object.invoke("get_templateDependencies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_uxmlObjectEntries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry,
        > = __cordl_object.invoke("get_uxmlObjectEntries", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_uxmlObjectIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<i32> = __cordl_object
            .invoke("get_uxmlObjectIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visualElementAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElementAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElementAsset,
        > = __cordl_object.invoke("get_visualElementAssets", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_contentContainerId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentContainerId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_contentHash(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentHash", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_importedWithErrors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_importedWithErrors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_importedWithWarnings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_importedWithWarnings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_slots(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_slots", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_templateAssets(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::TemplateAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_templateAssets", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_visualElementAssets(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElementAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_visualElementAssets", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualTreeAsset_AssetEntry {
    pub path: *mut crate::System::String,
    pub typeFullName: *mut crate::System::String,
    pub asset: *mut crate::UnityEngine::Object,
    pub m_CachedType: *mut crate::System::Type,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_AssetEntry => "UnityEngine.UIElements"
    ."VisualTreeAsset/AssetEntry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+AssetEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_AssetEntry {
    pub fn _ctor(
        &mut self,
        path: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
        asset: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (path, _cordl_type, asset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_ret: *mut crate::System::Type = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_type",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualTreeAsset_SlotDefinition {
    pub name: *mut crate::System::String,
    pub insertionPointId: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition => "UnityEngine.UIElements"
    ."VisualTreeAsset/SlotDefinition"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotDefinition")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_SlotDefinition {}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualTreeAsset_SlotUsageEntry {
    pub slotName: *mut crate::System::String,
    pub assetId: i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry => "UnityEngine.UIElements"
    ."VisualTreeAsset/SlotUsageEntry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+SlotUsageEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_SlotUsageEntry {}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualTreeAsset_UsingEntry {
    pub alias: *mut crate::System::String,
    pub path: *mut crate::System::String,
    pub asset: *mut crate::UnityEngine::UIElements::VisualTreeAsset,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_UsingEntry => "UnityEngine.UIElements"
    ."VisualTreeAsset/UsingEntry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry {
    pub fn _ctor(
        &mut self,
        alias: *mut crate::System::String,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (alias, path),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeAsset_UsingEntryComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer =>
    "UnityEngine.UIElements"."VisualTreeAsset/UsingEntryComparer"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
        y: crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntry,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UsingEntryComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeAsset_UsingEntryComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisualTreeAsset_UxmlObjectEntry {
    pub parentId: i32,
    pub uxmlObjectAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::UxmlObjectAsset,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry =>
    "UnityEngine.UIElements"."VisualTreeAsset/UxmlObjectEntry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeAsset+UxmlObjectEntry")]
impl crate::UnityEngine::UIElements::VisualTreeAsset_UxmlObjectEntry {
    pub fn _ctor(
        &mut self,
        parentId: i32,
        uxmlObjectAssets: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::UxmlObjectAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parentId, uxmlObjectAssets),
        )?;
        Ok(__cordl_ret)
    }
}
