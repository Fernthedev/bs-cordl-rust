#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstantCheck {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::ConstantCheck {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "ConstantCheck";
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
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl std::ops::Deref for crate::System::Linq::Expressions::ConstantCheck {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::ConstantCheck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl crate::System::Linq::Expressions::ConstantCheck {
    pub fn AnalyzeTypeIs_Expression_Type1(
        operand: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        testType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
    > {
        let __cordl_ret: crate::System::Linq::Expressions::AnalyzeTypeIsResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnalyzeTypeIs", (operand, testType))?;
        Ok(__cordl_ret.into())
    }
    pub fn AnalyzeTypeIs_TypeBinaryExpression0(
        typeIs: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::TypeBinaryExpression,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Linq::Expressions::AnalyzeTypeIsResult,
    > {
        let __cordl_ret: crate::System::Linq::Expressions::AnalyzeTypeIsResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnalyzeTypeIs", (typeIs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+ConstantCheck")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::ConstantCheck {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
