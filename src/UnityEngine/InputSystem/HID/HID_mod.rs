#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
#[repr(C)]
#[derive(Debug)]
pub struct HID {
    __cordl_parent: crate::UnityEngine::InputSystem::InputDevice,
    pub m_HaveParsedHIDDescriptor: bool,
    pub m_HIDDescriptor: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputSystem::HID::HID {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HID";
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HID {
    type Target = crate::UnityEngine::InputSystem::InputDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
impl crate::UnityEngine::InputSystem::HID::HID {
    pub const kHIDInterface: &'static str = "HID";
    pub const kHIDNamespace: &'static str = "HID";
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+Button")]
    pub type Button = crate::UnityEngine::InputSystem::HID::HID_Button;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+GenericDesktop")]
    pub type GenericDesktop = crate::UnityEngine::InputSystem::HID::HID_GenericDesktop;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionDescriptor")]
    pub type HIDCollectionDescriptor = crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionType")]
    pub type HIDCollectionType = crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptor")]
    pub type HIDDeviceDescriptor = crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptorBuilder")]
    pub type HIDDeviceDescriptorBuilder = crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementDescriptor")]
    pub type HIDElementDescriptor = crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementFlags")]
    pub type HIDElementFlags = crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
    pub type HIDLayoutBuilder = crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDReportType")]
    pub type HIDReportType = crate::UnityEngine::InputSystem::HID::HID_HIDReportType;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+Simulation")]
    pub type Simulation = crate::UnityEngine::InputSystem::HID::HID_Simulation;
    #[cfg(feature = "UnityEngine+InputSystem+HID+HID+UsagePage")]
    pub type UsagePage = crate::UnityEngine::InputSystem::HID::HID_UsagePage;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnFindLayoutForDevice(
        description: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        matchedLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        executeDeviceCommand: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputDeviceExecuteCommandDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnFindLayoutForDevice",
                (description, matchedLayout, executeDeviceCommand),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadHIDDeviceDescriptor(
        deviceDescription: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        executeCommandDelegate: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputDeviceExecuteCommandDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReadHIDDeviceDescriptor",
                (deviceDescription, executeCommandDelegate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UsagePageToString(
        usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UsagePageToString", (usagePage))?;
        Ok(__cordl_ret.into())
    }
    pub fn UsageToString(
        usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UsageToString", (usagePage, usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_QueryHIDParsedReportDescriptorDeviceCommandType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_QueryHIDParsedReportDescriptorDeviceCommandType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_QueryHIDReportDescriptorDeviceCommandType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_QueryHIDReportDescriptorDeviceCommandType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_QueryHIDReportDescriptorSizeDeviceCommandType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_QueryHIDReportDescriptorSizeDeviceCommandType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hidDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor = __cordl_object
            .invoke("get_hidDescriptor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::HID::HID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+Button")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_Button {
    #[default]
    Primary = 1i32,
    Secondary = 2i32,
    Tertiary = 3i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+Button")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_Button {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "Button";
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
for crate::UnityEngine::InputSystem::HID::HID_Button {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_Button {
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
for crate::UnityEngine::InputSystem::HID::HID_Button {
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
for crate::UnityEngine::InputSystem::HID::HID_Button {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+GenericDesktop")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_GenericDesktop {
    #[default]
    ApplicationBreak = 165i32,
    ApplicationDebuggerBreak = 166i32,
    AssistiveControl = 10i32,
    ByteCount = 59i32,
    CountedBuffer = 58i32,
    Dial = 55i32,
    DpadDown = 145i32,
    DpadLeft = 147i32,
    DpadRight = 146i32,
    DpadUp = 144i32,
    FeatureNotification = 71i32,
    Gamepad = 5i32,
    HatSwitch = 57i32,
    Joystick = 4i32,
    Keyboard = 6i32,
    Keypad = 7i32,
    MotionWakeup = 60i32,
    Mouse = 2i32,
    MultiAxisController = 8i32,
    Pointer = 1i32,
    ResolutionMultiplier = 72i32,
    Rx = 51i32,
    Ry = 52i32,
    Rz = 53i32,
    Select = 62i32,
    Slider = 54i32,
    Start = 61i32,
    SystemAppMenu = 134i32,
    SystemBreak = 163i32,
    SystemColdRestart = 142i32,
    SystemContextMenu = 132i32,
    SystemControl = 128i32,
    SystemDebuggerBreak = 164i32,
    SystemDisplayBoth = 179i32,
    SystemDisplayDual = 180i32,
    SystemDisplayExternal = 178i32,
    SystemDisplayInternal = 177i32,
    SystemDisplayInvert = 176i32,
    SystemDisplayLCDAutoScale = 183i32,
    SystemDisplaySwapPrimarySecondary = 182i32,
    SystemDisplayToggleIntExt = 181i32,
    SystemDock = 160i32,
    SystemHibernate = 168i32,
    SystemMainMenu = 133i32,
    SystemMenuDown = 141i32,
    SystemMenuExit = 136i32,
    SystemMenuHelp = 135i32,
    SystemMenuLeft = 139i32,
    SystemMenuRight = 138i32,
    SystemMenuSelect = 137i32,
    SystemMenuUp = 140i32,
    SystemPowerDown = 129i32,
    SystemSetup = 162i32,
    SystemSleep = 130i32,
    SystemSpeakerMute = 167i32,
    SystemUndock = 161i32,
    SystemWakeUp = 131i32,
    SystemWarmRestart = 143i32,
    TabletPCControls = 9i32,
    Undefined = 0i32,
    Vbrx = 67i32,
    Vbry = 68i32,
    Vbrz = 69i32,
    Vno = 70i32,
    Vx = 64i32,
    Vy = 65i32,
    Vz = 66i32,
    Wheel = 56i32,
    X = 48i32,
    Y = 49i32,
    Z = 50i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+GenericDesktop")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_GenericDesktop {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "GenericDesktop";
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
for crate::UnityEngine::InputSystem::HID::HID_GenericDesktop {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_GenericDesktop {
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
for crate::UnityEngine::InputSystem::HID::HID_GenericDesktop {
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
for crate::UnityEngine::InputSystem::HID::HID_GenericDesktop {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HID_HIDCollectionDescriptor {
    pub _cordl_type: crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType,
    pub usage: i32,
    pub usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub parent: i32,
    pub childCount: i32,
    pub firstChild: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDCollectionDescriptor";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionDescriptor")]
impl crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor {}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_HIDCollectionType {
    #[default]
    Application = 1i32,
    Logical = 2i32,
    NamedArray = 4i32,
    Physical = 0i32,
    Report = 3i32,
    UsageModifier = 6i32,
    UsageSwitch = 5i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDCollectionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDCollectionType";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDCollectionType {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HID_HIDDeviceDescriptor {
    pub vendorId: i32,
    pub productId: i32,
    pub usage: i32,
    pub usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub inputReportSize: i32,
    pub outputReportSize: i32,
    pub featureReportSize: i32,
    pub elements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor,
        >,
    >,
    pub collections: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDDeviceDescriptor";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptor")]
impl crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor {
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromJson", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToJson", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptorBuilder")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HID_HIDDeviceDescriptorBuilder {
    pub usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub usage: i32,
    pub m_CurrentReportId: i32,
    pub m_CurrentReportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
    pub m_CurrentReportOffsetInBits: i32,
    pub m_Elements: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor,
        >,
    >,
    pub m_Collections: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::HID::HID_HIDCollectionDescriptor,
        >,
    >,
    pub m_InputReportSize: i32,
    pub m_OutputReportSize: i32,
    pub m_FeatureReportSize: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptorBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDDeviceDescriptorBuilder";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptorBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDDeviceDescriptorBuilder")]
impl crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder {
    pub fn AddElement_HID_GenericDesktop1(
        &mut self,
        usage: crate::UnityEngine::InputSystem::HID::HID_GenericDesktop,
        sizeInBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddElement",
            (usage, sizeInBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddElement_HID_UsagePage_i32_0(
        &mut self,
        usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
        sizeInBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddElement",
            (usagePage, usage, sizeInBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Finish",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartReport(
        &mut self,
        reportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
        reportId: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StartReport",
            (reportType, reportId),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithLogicalMinMax(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithLogicalMinMax",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WithPhysicalMinMax(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptorBuilder = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WithPhysicalMinMax",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HID_GenericDesktop1(
        &mut self,
        usage: crate::UnityEngine::InputSystem::HID::HID_GenericDesktop,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (usage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HID_UsagePage_i32_0(
        &mut self,
        usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (usagePage, usage),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HID_HIDElementDescriptor {
    pub usage: i32,
    pub usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub unit: i32,
    pub unitExponent: i32,
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub reportType: crate::UnityEngine::InputSystem::HID::HID_HIDReportType,
    pub collectionIndex: i32,
    pub reportId: i32,
    pub reportSizeInBits: i32,
    pub reportOffsetInBits: i32,
    pub flags: crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags,
    pub usageMin: crate::System::Nullable_1<i32>,
    pub usageMax: crate::System::Nullable_1<i32>,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDElementDescriptor";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementDescriptor")]
impl crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor {
    pub fn AddChildControls(
        &mut self,
        element: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::HID::HID_HIDElementDescriptor,
        >,
        controlName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        builder: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Builder,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddChildControls",
            (element, controlName, builder),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineAxisNormalizationParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineAxisNormalizationParameters",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineDefaultState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::PrimitiveValue,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::PrimitiveValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineDefaultState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineDisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineDisplayName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineFormat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "DetermineLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "DetermineName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineParameters",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineProcessors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DetermineProcessors",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::InputSystem::Utilities::InternedString,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "DetermineUsages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Is(
        &mut self,
        usagePage: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Is",
            (usagePage, usage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUsableElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsUsableElement",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasNullState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasNullState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasPreferredState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasPreferredState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isArray",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConstant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isConstant",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isNonLinear(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isNonLinear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isRelative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isRelative",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSigned(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isSigned",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWrapping(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isWrapping",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxFloatValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxFloatValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minFloatValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_minFloatValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_HIDElementFlags {
    #[default]
    BufferedBytes = 256i32,
    Constant = 1i32,
    NoPreferred = 32i32,
    NonLinear = 16i32,
    NullState = 64i32,
    Relative = 4i32,
    Variable = 2i32,
    Volatile = 128i32,
    Wrap = 8i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDElementFlags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDElementFlags";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDElementFlags {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct HID_HIDLayoutBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub displayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub hidDescriptor: crate::UnityEngine::InputSystem::HID::HID_HIDDeviceDescriptor,
    pub parentLayout: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub deviceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDLayoutBuilder";
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
impl crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder {
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDLayoutBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::HID::HID_HIDLayoutBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDReportType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_HIDReportType {
    #[default]
    Feature = 3i32,
    Input = 1i32,
    Output = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+HIDReportType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_HIDReportType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDReportType";
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
for crate::UnityEngine::InputSystem::HID::HID_HIDReportType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_HIDReportType {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDReportType {
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
for crate::UnityEngine::InputSystem::HID::HID_HIDReportType {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+Simulation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_Simulation {
    #[default]
    Accelerator = 196i32,
    Aileron = 176i32,
    AileronTrim = 177i32,
    AirplaneSimulationDevice = 9i32,
    AntiTorqueControl = 178i32,
    AutomobileSimulationDevice = 2i32,
    AutopilotEnable = 179i32,
    Ballast = 204i32,
    BarrelElevation = 202i32,
    BicycleCrank = 205i32,
    BicylcleSimulationDevice = 12i32,
    Brake = 197i32,
    ChaffRelease = 180i32,
    Clutch = 198i32,
    CollectiveControl = 181i32,
    CyclicControl = 34i32,
    CyclicTrim = 35i32,
    DiveBreak = 182i32,
    DivePlane = 203i32,
    ElectronicCountermeasures = 183i32,
    Elevator = 184i32,
    ElevatorTrim = 185i32,
    FlareRelease = 189i32,
    FlightCommunications = 188i32,
    FlightControlStick = 32i32,
    FlightSimulationDevice = 1i32,
    FlightStick = 33i32,
    FlightYoke = 36i32,
    FrontBrake = 207i32,
    HandleBars = 206i32,
    HelicopterSimulationDevice = 10i32,
    LandingGear = 190i32,
    MagicCarpetSimulationDevice = 11i32,
    MotorcycleSimulationDevice = 7i32,
    RearBrake = 208i32,
    Rudder = 186i32,
    SailingSimulationDevice = 6i32,
    Shifter = 199i32,
    SpaceshipSimulationDevice = 4i32,
    SportsSimulationDevice = 8i32,
    Steering = 200i32,
    SubmarineSimulationDevice = 5i32,
    TankSimulationDevice = 3i32,
    Throttle = 187i32,
    ToeBreak = 191i32,
    TrackControl = 37i32,
    Trigger = 192i32,
    TurretDirection = 201i32,
    Undefined = 0i32,
    WeaponsArm = 193i32,
    WeaponsSelect = 194i32,
    WingFlaps = 195i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+Simulation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_Simulation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "Simulation";
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
for crate::UnityEngine::InputSystem::HID::HID_Simulation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_Simulation {
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
for crate::UnityEngine::InputSystem::HID::HID_Simulation {
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
for crate::UnityEngine::InputSystem::HID::HID_Simulation {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+UsagePage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HID_UsagePage {
    #[default]
    AlphanumericDisplay = 20i32,
    Arcade = 145i32,
    BarCodeScanner = 140i32,
    Button = 9i32,
    Camera = 144i32,
    Consumer = 12i32,
    Digitizer = 13i32,
    GameControls = 5i32,
    GenericDesktop = 1i32,
    GenericDeviceControls = 6i32,
    Keyboard = 7i32,
    LEDs = 8i32,
    MagneticStripeReader = 142i32,
    MedicalInstruments = 64i32,
    Monitor = 128i32,
    Ordinal = 10i32,
    PID = 15i32,
    Power = 132i32,
    Simulation = 2i32,
    SportControls = 4i32,
    Telephony = 11i32,
    Undefined = 0i32,
    Unicode = 16i32,
    VRControls = 3i32,
    VendorDefined = 65280i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HID+UsagePage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HID_UsagePage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "UsagePage";
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
for crate::UnityEngine::InputSystem::HID::HID_UsagePage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HID_UsagePage {
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
for crate::UnityEngine::InputSystem::HID::HID_UsagePage {
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
for crate::UnityEngine::InputSystem::HID::HID_UsagePage {
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
