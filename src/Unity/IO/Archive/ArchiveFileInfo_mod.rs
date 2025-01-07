#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArchiveFileInfo {
    pub Filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub FileSize: u64,
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.IO.Archive";
    const CLASS_NAME: &'static str = "ArchiveFileInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::IO::Archive::ArchiveFileInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+IO+Archive+ArchiveFileInfo")]
impl crate::Unity::IO::Archive::ArchiveFileInfo {}
