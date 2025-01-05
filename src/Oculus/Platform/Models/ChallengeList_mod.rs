#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
#[repr(C)]
#[derive(Debug)]
pub struct ChallengeList {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
    >,
    pub TotalCount: u64,
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::ChallengeList =>
    "Oculus.Platform.Models"."ChallengeList"
);
#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
impl std::ops::Deref for crate::Oculus::Platform::Models::ChallengeList {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::ChallengeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
impl crate::Oculus::Platform::Models::ChallengeList {
    pub fn New(
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        a: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+ChallengeList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::ChallengeList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
