#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenUtilInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::Internal::ZenUtilInternal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ZenUtilInternal";
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
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::Deref for crate::Zenject::Internal::ZenUtilInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl std::ops::DerefMut for crate::Zenject::Internal::ZenUtilInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl crate::Zenject::Internal::ZenUtilInternal {
    pub fn AddStateMachineBehaviourAutoInjectersInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::SceneManagement::Scene),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddStateMachineBehaviourAutoInjectersInScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddStateMachineBehaviourAutoInjectersInScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddStateMachineBehaviourAutoInjectersUnderGameObject(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddStateMachineBehaviourAutoInjectersUnderGameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddStateMachineBehaviourAutoInjectersUnderGameObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (root))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AreFunctionsEqual(
        left: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        right: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                    quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                ),
                bool,
                2usize,
            >("AreFunctionsEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AreFunctionsEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (left, right)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllSceneContexts() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
                    >,
                >,
                0usize,
            >("GetAllSceneContexts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAllSceneContexts", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SceneContext>,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInheritanceDelta(
        derived: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                i32,
                2usize,
            >("GetInheritanceDelta")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInheritanceDelta", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (derived, parent)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
        monoBehaviours: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::SceneManagement::Scene,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetInjectableMonoBehavioursInScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInjectableMonoBehavioursInScene", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene, monoBehaviours))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursUnderGameObject(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        injectableComponents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetInjectableMonoBehavioursUnderGameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInjectableMonoBehavioursUnderGameObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (gameObject, injectableComponents))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableMonoBehavioursUnderGameObjectInternal(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        injectableComponents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetInjectableMonoBehavioursUnderGameObjectInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInjectableMonoBehavioursUnderGameObjectInternal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (gameObject, injectableComponents))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRootGameObjects(
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::SceneManagement::Scene),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    >,
                >,
                1usize,
            >("GetRootGameObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRootGameObjects", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = unsafe { method.invoke_unchecked((), (scene)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsInjectableMonoBehaviourType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsInjectableMonoBehaviourType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsInjectableMonoBehaviourType", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("IsNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNull", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (obj)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ZenUtilInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ZenUtilInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
