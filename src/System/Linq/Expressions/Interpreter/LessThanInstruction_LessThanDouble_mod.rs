#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
#[repr(C)]
#[derive(Debug)]
pub struct LessThanInstruction_LessThanDouble {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LessThanInstruction,
    >,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LessThanInstruction_LessThanDouble =>
    "System.Linq.Expressions.Interpreter"."LessThanInstruction/LessThanDouble"
);
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
impl std::ops::Deref for crate::GlobalNamespace::LessThanInstruction_LessThanDouble {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::LessThanInstruction,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
impl std::ops::DerefMut for crate::GlobalNamespace::LessThanInstruction_LessThanDouble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
impl crate::GlobalNamespace::LessThanInstruction_LessThanDouble {
    pub fn New(
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nullValue))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        frame: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::InterpretedFrame,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Run", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nullValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nullValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+LessThanInstruction+LessThanDouble"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LessThanInstruction_LessThanDouble {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
