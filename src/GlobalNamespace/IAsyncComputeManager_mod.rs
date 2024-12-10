#[cfg(feature = "IAsyncComputeManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsyncComputeManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAsyncComputeManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IAsyncComputeManager => ""
    ."IAsyncComputeManager"
);
#[cfg(feature = "IAsyncComputeManager")]
impl std::ops::Deref for crate::GlobalNamespace::IAsyncComputeManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAsyncComputeManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IAsyncComputeManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAsyncComputeManager")]
impl crate::GlobalNamespace::IAsyncComputeManager {
    pub fn BeginOperation_AsyncComputeOperation0(
        &mut self,
        asyncComputeOperation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AsyncComputeOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginOperation", (asyncComputeOperation))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginOperation_AsyncComputeOperation_1_1<T>(
        &mut self,
        asyncComputeOperation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AsyncComputeOperation_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = __cordl_object.invoke("BeginOperation", (asyncComputeOperation))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IAsyncComputeManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IAsyncComputeManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
