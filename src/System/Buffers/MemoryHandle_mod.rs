#[cfg(feature = "System+Buffers+MemoryHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MemoryHandle {
    pub _pointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _handle: crate::System::Runtime::InteropServices::GCHandle,
    pub _pinnable: quest_hook::libil2cpp::Gc<crate::System::Buffers::IPinnable>,
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Buffers::MemoryHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Buffers";
    const CLASS_NAME: &'static str = "MemoryHandle";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Buffers::MemoryHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Buffers::MemoryHandle {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Buffers::MemoryHandle {
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
unsafe impl quest_hook::libil2cpp::Return for crate::System::Buffers::MemoryHandle {
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
#[cfg(feature = "System+Buffers+MemoryHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Buffers::MemoryHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
impl crate::System::Buffers::MemoryHandle {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        handle: crate::System::Runtime::InteropServices::GCHandle,
        pinnable: quest_hook::libil2cpp::Gc<crate::System::Buffers::IPinnable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (pointer, handle, pinnable),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Pointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Pointer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
impl AsRef<crate::System::IDisposable> for crate::System::Buffers::MemoryHandle {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Buffers+MemoryHandle")]
impl AsMut<crate::System::IDisposable> for crate::System::Buffers::MemoryHandle {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
