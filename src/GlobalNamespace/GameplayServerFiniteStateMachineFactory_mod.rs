#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayServerFiniteStateMachineFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayServerFiniteStateMachineFactory";
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
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    pub fn Create(
        initParams: crate::GlobalNamespace::GameplayServerFiniteStateMachine_InitParams,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayServerFiniteStateMachine,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayServerFiniteStateMachine,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (initParams))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
