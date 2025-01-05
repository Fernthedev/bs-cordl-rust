#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnHeaderColumnSortIndicator {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_IndexLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Label>,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator =>
    "UnityEngine.UIElements.Internal"."MultiColumnHeaderColumnSortIndicator"
);
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator {
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
    pub fn set_sortOrderLabel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortOrderLabel", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnHeaderColumnSortIndicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnSortIndicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
