#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRuntimeReflectionSystemWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _implementation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    >,
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "ScriptableRuntimeReflectionSystemWrapper";
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
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    pub fn Internal_ScriptableRuntimeReflectionSystemWrapper_TickRealtimeProbes(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_ScriptableRuntimeReflectionSystemWrapper_TickRealtimeProbes",
                (result),
            )?;
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
    pub fn get_implementation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
        > = __cordl_object.invoke("get_implementation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_implementation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_implementation", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
