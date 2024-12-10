#[cfg(feature = "System+Threading+LockHolder")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LockHolder {
    pub _lock: *mut crate::System::Threading::Lock,
}
#[cfg(feature = "System+Threading+LockHolder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::LockHolder =>
    "System.Threading"."LockHolder"
);
#[cfg(feature = "System+Threading+LockHolder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::LockHolder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+LockHolder")]
impl crate::System::Threading::LockHolder {
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
}
