#[cfg(feature = "AsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AsyncHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::AsyncHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AsyncHelper";
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
#[cfg(feature = "AsyncHelper")]
impl std::ops::Deref for crate::GlobalNamespace::AsyncHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::AsyncHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncHelper")]
impl crate::GlobalNamespace::AsyncHelper {
    pub fn AnyTaskTrueNonAlloc(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnyTaskTrueNonAlloc", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunSync_Func_1_0(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunSync", (asyncTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunSync_Func_1_1<T>(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunSync", (asyncTask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AsyncHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AsyncHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
