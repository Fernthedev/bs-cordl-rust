#[cfg(feature = "Oculus+Platform+Challenges")]
#[repr(C)]
#[derive(Debug)]
pub struct Challenges {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Challenges")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Challenges {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Challenges";
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
#[cfg(feature = "Oculus+Platform+Challenges")]
impl std::ops::Deref for crate::Oculus::Platform::Challenges {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl std::ops::DerefMut for crate::Oculus::Platform::Challenges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl crate::Oculus::Platform::Challenges {
    pub fn Create(
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::Oculus::Platform::ChallengeOptions>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                2usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (leaderboardName, challengeOptions)) };
        Ok(__cordl_ret.into())
    }
    pub fn DeclineInvite(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                1usize,
            >("DeclineInvite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeclineInvite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn Delete(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("Delete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Delete", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (challengeID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                1usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntries(
        challengeID: u64,
        limit: i32,
        filter: crate::Oculus::Platform::LeaderboardFilterType,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    i32,
                    crate::Oculus::Platform::LeaderboardFilterType,
                    crate::Oculus::Platform::LeaderboardStartAt,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeEntryList,
                        >,
                    >,
                >,
                4usize,
            >("GetEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEntries", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = unsafe {
            method.invoke_unchecked((), (challengeID, limit, filter, startAt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesAfterRank(
        challengeID: u64,
        limit: i32,
        afterRank: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, i32, u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeEntryList,
                        >,
                    >,
                >,
                3usize,
            >("GetEntriesAfterRank")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEntriesAfterRank", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID, limit, afterRank)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEntriesByIds(
        challengeID: u64,
        limit: i32,
        startAt: crate::Oculus::Platform::LeaderboardStartAt,
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    i32,
                    crate::Oculus::Platform::LeaderboardStartAt,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeEntryList,
                        >,
                    >,
                >,
                4usize,
            >("GetEntriesByIds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEntriesByIds", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = unsafe {
            method.invoke_unchecked((), (challengeID, limit, startAt, userIDs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetList(
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Oculus::Platform::ChallengeOptions>,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeList,
                        >,
                    >,
                >,
                2usize,
            >("GetList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetList", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeOptions, limit)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextChallenges(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextChallenges", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeEntryList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextEntries", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousChallenges(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeList,
                        >,
                    >,
                >,
                1usize,
            >("GetPreviousChallenges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPreviousChallenges", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousEntries(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ChallengeEntryList,
                        >,
                    >,
                >,
                1usize,
            >("GetPreviousEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPreviousEntries", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ChallengeEntryList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn Join(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                1usize,
            >("Join")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Join", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn Leave(
        challengeID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                1usize,
            >("Leave")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Leave", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID)) };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInfo(
        challengeID: u64,
        challengeOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::ChallengeOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::Gc<crate::Oculus::Platform::ChallengeOptions>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::Challenge,
                        >,
                    >,
                >,
                2usize,
            >("UpdateInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
            >,
        > = unsafe { method.invoke_unchecked((), (challengeID, challengeOptions)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Challenges")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Challenges {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
