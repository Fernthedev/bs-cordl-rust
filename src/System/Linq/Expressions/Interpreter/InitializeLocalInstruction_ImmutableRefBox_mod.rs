#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitializeLocalInstruction_ImmutableRefBox {
    __cordl_parent: crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction,
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions.Interpreter";
    const CLASS_NAME: &'static str = "InitializeLocalInstruction/ImmutableRefBox";
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
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
impl std::ops::Deref
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox {
    type Target = crate::System::Linq::Expressions::Interpreter::InitializeLocalInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
impl crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox {
    pub fn New(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index))?;
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
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index))?;
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
    feature = "System+Linq+Expressions+Interpreter+InitializeLocalInstruction+ImmutableRefBox"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::InitializeLocalInstruction_ImmutableRefBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
