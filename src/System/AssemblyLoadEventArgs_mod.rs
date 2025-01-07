#[cfg(feature = "System+AssemblyLoadEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyLoadEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _LoadedAssembly_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::Assembly,
    >,
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::AssemblyLoadEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "AssemblyLoadEventArgs";
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
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl std::ops::Deref for crate::System::AssemblyLoadEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl std::ops::DerefMut for crate::System::AssemblyLoadEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl crate::System::AssemblyLoadEventArgs {
    pub fn New(
        loadedAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loadedAssembly))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        loadedAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loadedAssembly))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AssemblyLoadEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
