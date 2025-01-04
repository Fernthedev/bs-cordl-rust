#[cfg(feature = "UnityEngine+ConfigurableJointMotion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfigurableJointMotion {
    #[default]
    Free = 2i32,
    Limited = 1i32,
    Locked = 0i32,
}
#[cfg(feature = "UnityEngine+ConfigurableJointMotion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ConfigurableJointMotion =>
    "UnityEngine"."ConfigurableJointMotion"
);
