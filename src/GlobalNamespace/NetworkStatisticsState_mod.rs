#[cfg(feature = "NetworkStatisticsState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetworkStatisticsState {
    pub packetsSent: i64,
    pub packetsReceived: i64,
    pub bytesSent: i64,
    pub bytesReceived: i64,
    pub packetsLost: i64,
    pub packetsSentEncrypted: i64,
    pub packetsSentPlaintext: i64,
    pub packetsSentRejected: i64,
    pub packetsReceivedEncrypted: i64,
    pub packetsReceivedPlaintext: i64,
    pub packetsReceivedRejected: i64,
    pub encryptionProcessingTime: i64,
    pub decryptionProcessingTime: i64,
}
#[cfg(feature = "NetworkStatisticsState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NetworkStatisticsState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetworkStatisticsState";
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
for crate::GlobalNamespace::NetworkStatisticsState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NetworkStatisticsState {
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
for crate::GlobalNamespace::NetworkStatisticsState {
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
for crate::GlobalNamespace::NetworkStatisticsState {
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
#[cfg(feature = "NetworkStatisticsState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::NetworkStatisticsState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "NetworkStatisticsState")]
impl crate::GlobalNamespace::NetworkStatisticsState {
    #[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
    pub type NetworkStatisticsUpdateDelegate = crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate;
    pub fn _ctor(
        &mut self,
        packetsSent: i64,
        packetsReceived: i64,
        bytesSent: i64,
        bytesReceived: i64,
        packetsLost: i64,
        packetsSentEncrypted: i64,
        packetsSentPlaintext: i64,
        packetsSentRejected: i64,
        packetsReceivedEncrypted: i64,
        packetsReceivedPlaintext: i64,
        packetsReceivedRejected: i64,
        encryptionProcessingTime: i64,
        decryptionProcessingTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                packetsSent,
                packetsReceived,
                bytesSent,
                bytesReceived,
                packetsLost,
                packetsSentEncrypted,
                packetsSentPlaintext,
                packetsSentRejected,
                packetsReceivedEncrypted,
                packetsReceivedPlaintext,
                packetsReceivedRejected,
                encryptionProcessingTime,
                decryptionProcessingTime,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NetworkStatisticsState,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NetworkStatisticsState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NetworkStatisticsDelta> {
        let __cordl_ret: crate::GlobalNamespace::NetworkStatisticsDelta = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetworkStatisticsUpdateDelegate";
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
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
impl crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    pub fn BeginInvoke(
        &mut self,
        statisticsState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NetworkStatisticsState,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (statisticsState, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        statisticsState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NetworkStatisticsState,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (statisticsState, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        statisticsState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::NetworkStatisticsState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (statisticsState))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NetworkStatisticsState+NetworkStatisticsUpdateDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkStatisticsState_NetworkStatisticsUpdateDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
