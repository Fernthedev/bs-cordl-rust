#[cfg(feature = "OVRTelemetryMarker")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRTelemetryMarker {
    pub _State_k__BackingField: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    pub _MarkerId_k__BackingField: i32,
    pub _InstanceKey_k__BackingField: i32,
    pub _client: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
    >,
}
#[cfg(feature = "OVRTelemetryMarker")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTelemetryMarker {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryMarker";
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
for crate::GlobalNamespace::OVRTelemetryMarker {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTelemetryMarker {
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
for crate::GlobalNamespace::OVRTelemetryMarker {
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
for crate::GlobalNamespace::OVRTelemetryMarker {
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
#[cfg(feature = "OVRTelemetryMarker")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTelemetryMarker {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTelemetryMarker")]
impl crate::GlobalNamespace::OVRTelemetryMarker {
    #[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
    pub type OVRTelemetryMarkerState = crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState;
    pub fn AddAnnotation(
        &mut self,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddAnnotation",
            (annotationKey, annotationValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddPoint(
        &mut self,
        point: crate::GlobalNamespace::OVRTelemetry_MarkerPoint,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddPoint",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn Send(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Send",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendIf(
        &mut self,
        condition: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendIf",
            (condition),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResult(
        &mut self,
        result: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetResult",
            (result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_OVRTelemetry_TelemetryClient_i32_i64_1(
        &mut self,
        client: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRTelemetry_TelemetryClient,
        >,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (client, markerId, instanceKey, timestampMs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i64_0(
        &mut self,
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (markerId, instanceKey, timestampMs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstanceKey(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InstanceKey",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MarkerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MarkerId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
    > {
        let __cordl_ret: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Result",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Sent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Sent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_State(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_State",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_State(
        &mut self,
        value: crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_State",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTelemetryMarker")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::OVRTelemetryMarker {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRTelemetryMarker")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::OVRTelemetryMarker {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRTelemetryMarker_OVRTelemetryMarkerState {
    pub _Sent_k__BackingField: bool,
    pub _Result_k__BackingField: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTelemetryMarkerState";
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
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
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
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
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
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
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
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTelemetryMarker+OVRTelemetryMarkerState")]
impl crate::GlobalNamespace::OVRTelemetryMarker_OVRTelemetryMarkerState {
    pub fn _ctor(
        &mut self,
        sent: bool,
        result: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sent, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
    > {
        let __cordl_ret: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Result",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Sent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Sent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Result(
        &mut self,
        value: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Result",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Sent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Sent",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
