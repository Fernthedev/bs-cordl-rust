#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::DelegateHelpers =>
    "UnityEngine.InputSystem.Utilities"."DelegateHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    pub fn InvokeCallbacksSafe_AndInvokeReturnedActions<TValue>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<
                    TValue,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                >,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeCallbacksSafe_AndInvokeReturnedActions",
                (callbacks, argument, callbackName, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_AnyCallbackReturnsObject<TValue, TReturn>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<TValue, TReturn>,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeCallbacksSafe_AnyCallbackReturnsObject",
                (callbacks, argument, callbackName, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_AnyCallbackReturnsTrue<TValue1, TValue2>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<TValue1, TValue2, bool>,
            >,
        >,
        argument1: TValue1,
        argument2: TValue2,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeCallbacksSafe_AnyCallbackReturnsTrue",
                (callbacks, argument1, argument2, callbackName, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_Gc_Gc0(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Action>,
            >,
        >,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeCallbacksSafe", (callbacks, callbackName, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_TValue1_TValue2_Gc_Gc2<TValue1, TValue2>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<TValue1, TValue2>,
            >,
        >,
        argument1: TValue1,
        argument2: TValue2,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeCallbacksSafe",
                (callbacks, argument1, argument2, callbackName, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_TValue_Gc_Gc1<TValue>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<TValue>,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InvokeCallbacksSafe",
                (callbacks, argument, callbackName, context),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
