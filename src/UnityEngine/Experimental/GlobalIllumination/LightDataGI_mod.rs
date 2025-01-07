#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightDataGI")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightDataGI {
    pub instanceID: i32,
    pub cookieID: i32,
    pub cookieScale: f32,
    pub color: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    pub indirectColor: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    pub orientation: crate::UnityEngine::Quaternion,
    pub position: crate::UnityEngine::Vector3,
    pub range: f32,
    pub coneAngle: f32,
    pub innerConeAngle: f32,
    pub shape0: f32,
    pub shape1: f32,
    pub _cordl_type: crate::UnityEngine::Experimental::GlobalIllumination::LightType,
    pub mode: crate::UnityEngine::Experimental::GlobalIllumination::LightMode,
    pub shadow: u8,
    pub falloff: crate::UnityEngine::Experimental::GlobalIllumination::FalloffType,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightDataGI")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";
    const CLASS_NAME: &'static str = "LightDataGI";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightDataGI")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightDataGI")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LightDataGI {
    pub fn InitNoBake(
        &mut self,
        lightInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitNoBake",
            (lightInstanceID),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_ByRefMut0(
        &mut self,
        light: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DirectionalLight,
        >,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (light, cookie),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_ByRefMut1(
        &mut self,
        light: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::PointLight,
        >,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (light, cookie),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_ByRefMut2(
        &mut self,
        light: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::SpotLight,
        >,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (light, cookie),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_ByRefMut3(
        &mut self,
        light: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::RectangleLight,
        >,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (light, cookie),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_ByRefMut4(
        &mut self,
        light: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::DiscLight,
        >,
        cookie: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Experimental::GlobalIllumination::Cookie,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (light, cookie),
        )?;
        Ok(__cordl_ret.into())
    }
}
