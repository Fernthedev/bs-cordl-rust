#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
#[repr(C)]
#[derive(Debug)]
pub struct FullConditionalExpressionWithType {
    __cordl_parent: crate::System::Linq::Expressions::FullConditionalExpression,
    pub _Type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::FullConditionalExpressionWithType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "FullConditionalExpressionWithType";
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
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
impl std::ops::Deref
for crate::System::Linq::Expressions::FullConditionalExpressionWithType {
    type Target = crate::System::Linq::Expressions::FullConditionalExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::FullConditionalExpressionWithType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
impl crate::System::Linq::Expressions::FullConditionalExpressionWithType {
    pub fn New(
        test: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifTrue: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifFalse: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (test, ifTrue, ifFalse, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        test: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifTrue: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        ifFalse: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (test, ifTrue, ifFalse, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+FullConditionalExpressionWithType")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::FullConditionalExpressionWithType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
