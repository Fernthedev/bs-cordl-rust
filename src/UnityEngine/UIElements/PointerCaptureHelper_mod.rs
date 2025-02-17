#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerCaptureHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::PointerCaptureHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "PointerCaptureHelper";
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
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerCaptureHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerCaptureHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl crate::UnityEngine::UIElements::PointerCaptureHelper {
    pub fn ActivateCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ActivateCompatibilityMouseEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ActivateCompatibilityMouseEvents", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (panel, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CapturePointer(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IEventHandler,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CapturePointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CapturePointer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handler, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCapturingElement(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>, i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
                2usize,
            >("GetCapturingElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCapturingElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        > = unsafe { method.invoke_unchecked((), (panel, pointerId)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStateFor(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDispatchState>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IEventHandler,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::PointerDispatchState,
                >,
                1usize,
            >("GetStateFor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStateFor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerDispatchState,
        > = unsafe { method.invoke_unchecked((), (handler)) };
        Ok(__cordl_ret.into())
    }
    pub fn HasPointerCapture(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IEventHandler,
                    >,
                    i32,
                ),
                bool,
                2usize,
            >("HasPointerCapture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasPointerCapture", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (handler, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreventCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PreventCompatibilityMouseEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PreventCompatibilityMouseEvents", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (panel, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPointerCapture(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessPointerCapture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessPointerCapture", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (panel, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleasePointer_IEventHandler0(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IEventHandler,
        >,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IEventHandler,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReleasePointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleasePointer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handler, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleasePointer_IPanel1(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReleasePointer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleasePointer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (panel, pointerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendCompatibilityMouseEvents(
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::IPointerEvent,
                    >,
                ),
                bool,
                2usize,
            >("ShouldSendCompatibilityMouseEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShouldSendCompatibilityMouseEvents", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (panel, evt)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerCaptureHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PointerCaptureHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
