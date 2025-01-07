#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct NotificationUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::NotificationUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "NotificationUtilities";
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
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl std::ops::Deref for crate::UnityEngine::Timeline::NotificationUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::NotificationUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl crate::UnityEngine::Timeline::NotificationUtilities {
    pub fn CreateNotificationsPlayable_IPlayableAsset_PlayableDirector2(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        markers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        >,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Playables::IPlayableAsset>,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNotificationsPlayable", (graph, markers, asset, director))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNotificationsPlayable_PlayableDirector0(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        markers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        >,
        director: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNotificationsPlayable", (graph, markers, director))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNotificationsPlayable_TimelineAsset1(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        markers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::IMarker>,
            >,
        >,
        timelineAsset: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::TimelineAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::Playables::ScriptPlayable_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Timeline::TimeNotificationBehaviour,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNotificationsPlayable", (graph, markers, timelineAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackTypeSupportsNotifications(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrackTypeSupportsNotifications", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+NotificationUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::NotificationUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
