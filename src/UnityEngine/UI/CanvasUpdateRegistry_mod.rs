#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct CanvasUpdateRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_PerformingLayoutUpdate: bool,
    pub m_PerformingGraphicUpdate: bool,
    pub m_CanvasUpdateProfilerStrings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub m_LayoutRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Collections::IndexedSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
    pub m_GraphicRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Collections::IndexedSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::CanvasUpdateRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "CanvasUpdateRegistry";
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
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl std::ops::Deref for crate::UnityEngine::UI::CanvasUpdateRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::CanvasUpdateRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl crate::UnityEngine::UI::CanvasUpdateRegistry {
    pub const m_CullingUpdateProfilerString: &'static str = "ClipperRegistry.Cull";
    pub fn CleanInvalidItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CleanInvalidItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CleanInvalidItems", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableCanvasElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DisableCanvasElementForRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisableCanvasElementForRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDisableCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalDisableCanvasElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalDisableCanvasElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalDisableCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalDisableCanvasElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalDisableCanvasElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                bool,
                1usize,
            >("InternalRegisterCanvasElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalRegisterCanvasElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (element)) };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                bool,
                1usize,
            >("InternalRegisterCanvasElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalRegisterCanvasElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (element)) };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalUnRegisterCanvasElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalUnRegisterCanvasElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InternalUnRegisterCanvasElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalUnRegisterCanvasElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsRebuildingGraphics() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("IsRebuildingGraphics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsRebuildingGraphics", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsRebuildingLayout() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("IsRebuildingLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsRebuildingLayout", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ObjectValidForUpdate(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                bool,
                1usize,
            >("ObjectValidForUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ObjectValidForUpdate", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (element)) };
        Ok(__cordl_ret.into())
    }
    pub fn ParentCount(
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                i32,
                1usize,
            >("ParentCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParentCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (child)) };
        Ok(__cordl_ret.into())
    }
    pub fn PerformUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PerformUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PerformUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterCanvasElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterCanvasElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterCanvasElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterCanvasElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SortLayoutList(
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
                ),
                i32,
                2usize,
            >("SortLayoutList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SortLayoutList", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (x, y)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryRegisterCanvasElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                bool,
                1usize,
            >("TryRegisterCanvasElementForGraphicRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryRegisterCanvasElementForGraphicRebuild", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (element)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryRegisterCanvasElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                bool,
                1usize,
            >("TryRegisterCanvasElementForLayoutRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryRegisterCanvasElementForLayoutRebuild", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (element)) };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterCanvasElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnRegisterCanvasElementForRebuild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnRegisterCanvasElementForRebuild", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::CanvasUpdateRegistry>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::CanvasUpdateRegistry>,
                0usize,
            >("get_instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::CanvasUpdateRegistry,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::CanvasUpdateRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
