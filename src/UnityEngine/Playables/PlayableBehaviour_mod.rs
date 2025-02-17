#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayableBehaviour {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Playables::PlayableBehaviour {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Playables";
    const CLASS_NAME: &'static str = "PlayableBehaviour";
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
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl std::ops::Deref for crate::UnityEngine::Playables::PlayableBehaviour {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::PlayableBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl crate::UnityEngine::Playables::PlayableBehaviour {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clone", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBehaviourPause(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnBehaviourPause")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBehaviourPause", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnBehaviourPlay(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnBehaviourPlay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBehaviourPlay", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnGraphStart(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnGraphStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnGraphStart", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnGraphStop(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnGraphStop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnGraphStop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPlayableCreate(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPlayableCreate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPlayableCreate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPlayableDestroy(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Playables::Playable),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPlayableDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnPlayableDestroy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PrepareFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PrepareFrame", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Playables::Playable,
                    crate::UnityEngine::Playables::FrameData,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ProcessFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessFrame", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (playable, info, playerData))
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
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::PlayableBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl AsRef<crate::System::ICloneable>
for crate::UnityEngine::Playables::PlayableBehaviour {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl AsMut<crate::System::ICloneable>
for crate::UnityEngine::Playables::PlayableBehaviour {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl AsRef<crate::UnityEngine::Playables::IPlayableBehaviour>
for crate::UnityEngine::Playables::PlayableBehaviour {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::IPlayableBehaviour {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Playables+PlayableBehaviour")]
impl AsMut<crate::UnityEngine::Playables::IPlayableBehaviour>
for crate::UnityEngine::Playables::PlayableBehaviour {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::IPlayableBehaviour {
        unsafe { std::mem::transmute(self) }
    }
}
