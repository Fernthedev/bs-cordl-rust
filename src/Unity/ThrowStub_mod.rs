#[cfg(feature = "Unity+ThrowStub")]
#[repr(C)]
#[derive(Debug)]
pub struct ThrowStub {
    __cordl_parent: crate::System::ObjectDisposedException,
}
#[cfg(feature = "Unity+ThrowStub")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::ThrowStub {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity";
    const CLASS_NAME: &'static str = "ThrowStub";
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
#[cfg(feature = "Unity+ThrowStub")]
impl std::ops::Deref for crate::Unity::ThrowStub {
    type Target = crate::System::ObjectDisposedException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl std::ops::DerefMut for crate::Unity::ThrowStub {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl crate::Unity::ThrowStub {
    pub fn ThrowNotSupportedException() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ThrowNotSupportedException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowNotSupportedException", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+ThrowStub")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::ThrowStub {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
