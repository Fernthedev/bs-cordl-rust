#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlTypeRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlTypeRestriction =>
    "UnityEngine.UIElements"."UxmlTypeRestriction"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl crate::UnityEngine::UIElements::UxmlTypeRestriction {
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlTypeRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
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
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl AsRef<
    crate::System::IEquatable_1<*mut crate::UnityEngine::UIElements::UxmlTypeRestriction>,
> for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        *mut crate::UnityEngine::UIElements::UxmlTypeRestriction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlTypeRestriction")]
impl AsMut<
    crate::System::IEquatable_1<*mut crate::UnityEngine::UIElements::UxmlTypeRestriction>,
> for crate::UnityEngine::UIElements::UxmlTypeRestriction {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        *mut crate::UnityEngine::UIElements::UxmlTypeRestriction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
