#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NumericConvertInstruction_ToUnderlying {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::NumericConvertInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NumericConvertInstruction_ToUnderlying {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "ToUnderlying";
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
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
impl std::ops::Deref for crate::GlobalNamespace::NumericConvertInstruction_ToUnderlying {
    type Target = crate::System::Linq::Expressions::Interpreter::NumericConvertInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::NumericConvertInstruction_ToUnderlying {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
impl crate::GlobalNamespace::NumericConvertInstruction_ToUnderlying {
    pub fn Convert(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Convert", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (to, isLiftedToNull))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        to: crate::System::TypeCode,
        isLiftedToNull: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (to, isLiftedToNull))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstructionName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_InstructionName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+NumericConvertInstruction+ToUnderlying"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NumericConvertInstruction_ToUnderlying {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
