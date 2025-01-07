#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseFieldTraits_2<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<TValueType>,
    pub m_Value: TValueUxmlAttributeType,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TValueUxmlAttributeType: std::marker::PhantomData<
        TValueUxmlAttributeType,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseFieldTraits`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements",
                        "BaseFieldTraits`2",
                    )
                    .unwrap()
                    .make_generic::<(TValueType, TValueUxmlAttributeType)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<TValueType>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValueUxmlAttributeType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValueUxmlAttributeType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValueUxmlAttributeType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldTraits_2")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TValueUxmlAttributeType: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseFieldTraits_2<
    TValueType,
    TValueUxmlAttributeType,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
