#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DropdownUtility =>
    "UnityEngine.UIElements"."DropdownUtility"
);
#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DropdownUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DropdownUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
impl crate::UnityEngine::UIElements::DropdownUtility {
    pub fn CreateDropdown() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGenericMenu>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGenericMenu,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateDropdown", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DropdownUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DropdownUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
