#[cfg(feature = "GenericLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GenericLogger")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::GenericLogger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GenericLogger";
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
#[cfg(feature = "GenericLogger")]
impl std::ops::Deref for crate::GlobalNamespace::GenericLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger")]
impl std::ops::DerefMut for crate::GlobalNamespace::GenericLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger")]
impl crate::GlobalNamespace::GenericLogger {
    pub const kVerboseLogDefineSymbol: &'static str = "BS_VERBOSE_LOGGING";
    #[cfg(feature = "GenericLogger+ScopedStopwatch")]
    pub type ScopedStopwatch = crate::GlobalNamespace::GenericLogger_ScopedStopwatch;
    pub fn Format(
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVerboseLogger>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (logger, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWithTimestamp(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWithTimestamp", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_IVerboseLogger0(
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVerboseLogger>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (logger, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_T1<T>(
        logger: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (logger, message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GenericLogger")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GenericLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericLogger_ScopedStopwatch {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _stopwatch: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Stopwatch>,
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GenericLogger/ScopedStopwatch";
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
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl std::ops::Deref for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl std::ops::DerefMut for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (processName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (processName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GenericLogger+ScopedStopwatch")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::GenericLogger_ScopedStopwatch {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
