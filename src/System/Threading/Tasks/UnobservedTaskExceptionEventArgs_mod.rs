#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnobservedTaskExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub m_exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    pub m_observed: bool,
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "UnobservedTaskExceptionEventArgs";
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
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::Deref
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    pub fn New(
        exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (exception))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
