#[cfg(feature = "AveragingValueRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct AveragingValueRecorder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _averageWindowDuration: f32,
    pub _historyValuesPerSecond: f32,
    pub _historyValuesCount: i32,
    pub _averageWindowValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::AveragingValueRecorder_AverageValueData,
        >,
    >,
    pub _historyValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<f32>,
    >,
    pub _time: f32,
    pub _historyTime: f32,
    pub _averageValue: f32,
    pub _averageWindowValuesDuration: f32,
    pub _lastValue: f32,
}
#[cfg(feature = "AveragingValueRecorder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AveragingValueRecorder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AveragingValueRecorder";
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
#[cfg(feature = "AveragingValueRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::AveragingValueRecorder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::AveragingValueRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl crate::GlobalNamespace::AveragingValueRecorder {
    #[cfg(feature = "AveragingValueRecorder+AverageValueData")]
    pub type AverageValueData = crate::GlobalNamespace::AveragingValueRecorder_AverageValueData;
    pub fn GetAverageValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAverageValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHistoryValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::Queue_1<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Queue_1<f32>,
        > = __cordl_object.invoke("GetHistoryValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLastValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        averageWindowDuration: f32,
        historyWindowDuration: f32,
        historyValuesPerSecond: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (averageWindowDuration, historyWindowDuration, historyValuesPerSecond),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        value: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (value, deltaTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        averageWindowDuration: f32,
        historyWindowDuration: f32,
        historyValuesPerSecond: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (averageWindowDuration, historyWindowDuration, historyValuesPerSecond),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AveragingValueRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AveragingValueRecorder_AverageValueData {
    pub _value_k__BackingField: f32,
    pub _time_k__BackingField: f32,
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AverageValueData";
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
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
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
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
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
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
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
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
impl crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    pub fn _ctor(
        &mut self,
        value: f32,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, _cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_time",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
