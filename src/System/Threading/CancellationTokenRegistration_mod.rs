#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancellationTokenRegistration {
    pub m_callbackInfo: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationCallbackInfo,
    >,
    pub m_registrationInfo: crate::System::Threading::SparselyPopulatedArrayAddInfo_1<
        quest_hook::libil2cpp::Gc<crate::System::Threading::CancellationCallbackInfo>,
    >,
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::CancellationTokenRegistration {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationTokenRegistration";
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
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::CancellationTokenRegistration {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::CancellationTokenRegistration {
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
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::CancellationTokenRegistration {
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
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::CancellationTokenRegistration {
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
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::CancellationTokenRegistration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl crate::System::Threading::CancellationTokenRegistration {
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
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DisposeAsync",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_CancellationTokenRegistration1(
        &mut self,
        other: crate::System::Threading::CancellationTokenRegistration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Unregister(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Unregister",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        callbackInfo: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationCallbackInfo,
        >,
        registrationInfo: crate::System::Threading::SparselyPopulatedArrayAddInfo_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::CancellationCallbackInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (callbackInfo, registrationInfo),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsRef<crate::System::IAsyncDisposable>
for crate::System::Threading::CancellationTokenRegistration {
    fn as_ref(&self) -> &crate::System::IAsyncDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsMut<crate::System::IAsyncDisposable>
for crate::System::Threading::CancellationTokenRegistration {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsRef<crate::System::IDisposable>
for crate::System::Threading::CancellationTokenRegistration {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsMut<crate::System::IDisposable>
for crate::System::Threading::CancellationTokenRegistration {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsRef<
    crate::System::IEquatable_1<crate::System::Threading::CancellationTokenRegistration>,
> for crate::System::Threading::CancellationTokenRegistration {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Threading+CancellationTokenRegistration")]
impl AsMut<
    crate::System::IEquatable_1<crate::System::Threading::CancellationTokenRegistration>,
> for crate::System::Threading::CancellationTokenRegistration {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        todo!()
    }
}
