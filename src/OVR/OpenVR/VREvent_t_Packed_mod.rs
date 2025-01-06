#[cfg(feature = "OVR+OpenVR+VREvent_t_Packed")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VREvent_t_Packed {
    pub eventType: u32,
    pub trackedDeviceIndex: u32,
    pub eventAgeSeconds: f32,
    pub data: crate::OVR::OpenVR::VREvent_Data_t,
}
#[cfg(feature = "OVR+OpenVR+VREvent_t_Packed")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_t_Packed => "OVR.OpenVR"
    ."VREvent_t_Packed"
);
#[cfg(feature = "OVR+OpenVR+VREvent_t_Packed")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_t_Packed {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_t_Packed")]
impl crate::OVR::OpenVR::VREvent_t_Packed {
    pub fn Unpack(
        &mut self,
        unpacked: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Unpack",
            (unpacked),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        unpacked: crate::OVR::OpenVR::VREvent_t,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (unpacked),
        )?;
        Ok(__cordl_ret.into())
    }
}
