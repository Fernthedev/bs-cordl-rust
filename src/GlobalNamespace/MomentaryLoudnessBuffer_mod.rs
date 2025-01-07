#[cfg(feature = "MomentaryLoudnessBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct MomentaryLoudnessBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nextDataIndex: i32,
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _readingInterval: i32,
}
#[cfg(feature = "MomentaryLoudnessBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MomentaryLoudnessBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MomentaryLoudnessBuffer";
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
#[cfg(feature = "MomentaryLoudnessBuffer")]
impl std::ops::Deref for crate::GlobalNamespace::MomentaryLoudnessBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MomentaryLoudnessBuffer")]
impl std::ops::DerefMut for crate::GlobalNamespace::MomentaryLoudnessBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MomentaryLoudnessBuffer")]
impl crate::GlobalNamespace::MomentaryLoudnessBuffer {
    pub fn AddSample(
        &mut self,
        data: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSample", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        numChannels: i32,
        sampleFrequency: i32,
        momentaryWindowDuration: f32,
        readingsPerBuffer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    numChannels,
                    sampleFrequency,
                    momentaryWindowDuration,
                    readingsPerBuffer,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        numChannels: i32,
        sampleFrequency: i32,
        momentaryWindowDuration: f32,
        readingsPerBuffer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    numChannels,
                    sampleFrequency,
                    momentaryWindowDuration,
                    readingsPerBuffer,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_buffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = __cordl_object.invoke("get_buffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isNextReadingIntervalReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isNextReadingIntervalReady", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MomentaryLoudnessBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MomentaryLoudnessBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
