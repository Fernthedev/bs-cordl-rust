#[cfg(feature = "TMPro+TMP_UpdateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_UpdateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LayoutQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_LayoutRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_GraphicQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_GraphicRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_InternalUpdateLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_InternalUpdateQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
    pub m_CullingUpdateLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_CullingUpdateQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
        >,
    >,
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_UpdateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_UpdateManager";
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
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::Deref for crate::TMPro::TMP_UpdateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::DerefMut for crate::TMPro::TMP_UpdateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl crate::TMPro::TMP_UpdateManager {
    pub fn DoRebuilds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoRebuilds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForCullingUpdate(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForCullingUpdate", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterTextObjectForUpdate(
        &mut self,
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextObjectForUpdate", (textObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterTextObjectForUpdate(
        &mut self,
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextObjectForUpdate", (textObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCameraPreCull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCameraPreCull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForCullingUpdate(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterTextElementForCullingUpdate", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterTextElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterTextElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterTextObjectForUpdate(
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterTextObjectForUpdate", (textObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterTextElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnRegisterTextElementForRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterTextObjectForUpdate(
        textObject: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Text>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnRegisterTextObjectForUpdate", (textObject))?;
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateManager> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_UpdateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
