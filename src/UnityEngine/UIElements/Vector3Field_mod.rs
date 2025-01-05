#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Field {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FloatField>,
        f32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Vector3Field =>
    "UnityEngine.UIElements"."Vector3Field"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3Field {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Vector3,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FloatField>,
        f32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
impl crate::UnityEngine::UIElements::Vector3Field {
    #[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Vector3Field_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Vector3Field_UxmlTraits;
    pub fn DescribeFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::FloatField,
                    >,
                    f32,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::FloatField,
                    >,
                    f32,
                >,
            >,
        > = __cordl_object.invoke("DescribeFields", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc1(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc1(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Vector3Field {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Field_UxmlFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Vector3Field>,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Vector3Field_UxmlTraits,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Vector3Field_UxmlFactory => "UnityEngine.UIElements"
    ."Vector3Field/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3Field_UxmlFactory {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Vector3Field>,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Vector3Field_UxmlTraits,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3Field_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
impl crate::UnityEngine::UIElements::Vector3Field_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Vector3Field_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Field_UxmlTraits {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    pub m_XValue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
    pub m_YValue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
    pub m_ZValue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Vector3Field_UxmlTraits
    => "UnityEngine.UIElements"."Vector3Field/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3Field_UxmlTraits {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3Field_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
impl crate::UnityEngine::UIElements::Vector3Field_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
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
#[cfg(feature = "UnityEngine+UIElements+Vector3Field+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Vector3Field_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
