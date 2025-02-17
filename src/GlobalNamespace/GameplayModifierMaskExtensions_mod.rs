#[cfg(feature = "GameplayModifierMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifierMaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifierMaskExtensions";
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
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl crate::GlobalNamespace::GameplayModifierMaskExtensions {
    pub fn Contains(
        mask: crate::GlobalNamespace::GameplayModifierMask,
        other: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::GameplayModifierMask,
                    crate::GlobalNamespace::GameplayModifierMask,
                ),
                bool,
                2usize,
            >("Contains")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Contains", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (mask, other)) };
        Ok(__cordl_ret.into())
    }
    pub fn DifferenceFrom(
        mask: crate::GlobalNamespace::GameplayModifierMask,
        other: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::GlobalNamespace::GameplayModifierMask,
                    crate::GlobalNamespace::GameplayModifierMask,
                ),
                i32,
                2usize,
            >("DifferenceFrom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DifferenceFrom", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (mask, other)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToGameplayModifierMask(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GameplayModifierMask> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>),
                crate::GlobalNamespace::GameplayModifierMask,
                1usize,
            >("ToGameplayModifierMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToGameplayModifierMask", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::GameplayModifierMask = unsafe {
            method.invoke_unchecked((), (gameplayModifiers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToHexString(
        mask: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::GameplayModifierMask),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToHexString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToHexString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToMask(
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GameplayModifierMask> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>),
                crate::GlobalNamespace::GameplayModifierMask,
                1usize,
            >("ToMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToMask", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::GameplayModifierMask = unsafe {
            method.invoke_unchecked((), (gameplayModifiers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToModifiers(
        gameplayModifierMask: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::GameplayModifierMask),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                1usize,
            >("ToModifiers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToModifiers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = unsafe { method.invoke_unchecked((), (gameplayModifierMask)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifierMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifierMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
