#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinder0Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinder0Extensions =>
    "Zenject"."FactoryFromBinder0Extensions"
);
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinder0Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinder0Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl crate::Zenject::FactoryFromBinder0Extensions {
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__1_1")]
    pub type __c__1_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__1_1<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__3_1")]
    pub type __c__3_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__3_1<
        TContract,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__5_2")]
    pub type __c__5_2<
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder0Extensions___c__5_2<TContract, TMemoryPool>;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__DisplayClass0_0_2")]
    pub type __c__DisplayClass0_0_2<
        TContract: quest_hook::libil2cpp::Type,
        TMemoryPool: quest_hook::libil2cpp::Type,
    > = crate::Zenject::FactoryFromBinder0Extensions___c__DisplayClass0_0_2<
        TContract,
        TMemoryPool,
    >;
    #[cfg(feature = "Zenject+FactoryFromBinder0Extensions+__c__DisplayClass6_0_1")]
    pub type __c__DisplayClass6_0_1<TContract: quest_hook::libil2cpp::Type> = crate::Zenject::FactoryFromBinder0Extensions___c__DisplayClass6_0_1<
        TContract,
    >;
    pub fn FromIFactory<TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
        factoryBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::ConcreteBinderGeneric_1<
                    *mut crate::Zenject::IFactory_1<TContract>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromIFactory", (fromBinder, factoryBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_Action_1_1<TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMonoPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMonoPoolableMemoryPool_FactoryFromBinder_1_0<TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMonoPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_0<TContract, TMemoryPool>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_Action_1_2<TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
        poolBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::Zenject::MemoryPoolInitialSizeMaxSizeBinder_1<TContract>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder, poolBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_1_1<TContract>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromPoolableMemoryPool_FactoryFromBinder_1_3<TContract, TMemoryPool>(
        fromBinder: quest_hook::libil2cpp::Gc<
            crate::Zenject::FactoryFromBinder_1<TContract>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ArgConditionCopyNonLazyBinder>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMemoryPool: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ArgConditionCopyNonLazyBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromPoolableMemoryPool", (fromBinder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FactoryFromBinder0Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinder0Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
