#[cfg(feature = "Oculus+Haptics+Haptics")]
#[repr(C)]
#[derive(Debug)]
pub struct Haptics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Haptics::Haptics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Haptics";
    const CLASS_NAME: &'static str = "Haptics";
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
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl std::ops::Deref for crate::Oculus::Haptics::Haptics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl std::ops::DerefMut for crate::Oculus::Haptics::Haptics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl crate::Oculus::Haptics::Haptics {
    pub fn CreateHapticPlayer(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CreateHapticPlayer", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn EnsureInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAmplitudeHapticPlayer(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetAmplitudeHapticPlayer", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClipDuration(
        &mut self,
        clipId: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetClipDuration", (clipId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFrequencyShiftHapticPlayer(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetFrequencyShiftHapticPlayer", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPriorityHapticPlayer(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetPriorityHapticPlayer", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHapticPlayerLooping(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsHapticPlayerLooping", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSupportedPlatform() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSupportedPlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadClip(
        &mut self,
        clipJson: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LoadClip", (clipJson))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoopHapticPlayer(
        &mut self,
        playerId: i32,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoopHapticPlayer", (playerId, enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn MapPriority(
        input: u32,
        inMin: i32,
        inMax: i32,
        outMin: i32,
        outMax: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MapPriority", (input, inMin, inMax, outMin, outMax))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PlayHapticPlayer(
        &mut self,
        playerId: i32,
        controller: crate::Oculus::Haptics::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayHapticPlayer", (playerId, controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseClip(&mut self, clipId: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseClip", (clipId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseHapticPlayer(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReleaseHapticPlayer", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAmplitudeHapticPlayer(
        &mut self,
        playerId: i32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAmplitudeHapticPlayer", (playerId, amplitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFrequencyShiftHapticPlayer(
        &mut self,
        playerId: i32,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFrequencyShiftHapticPlayer", (playerId, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHapticPlayerClip(
        &mut self,
        playerId: i32,
        clipId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHapticPlayerClip", (playerId, clipId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPriorityHapticPlayer(
        &mut self,
        playerId: i32,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPriorityHapticPlayer", (playerId, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopHapticPlayer(
        &mut self,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopHapticPlayer", (playerId))?;
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
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::Haptics>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Haptics::Haptics> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Haptics::Haptics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl AsRef<crate::System::IDisposable> for crate::Oculus::Haptics::Haptics {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Oculus+Haptics+Haptics")]
impl AsMut<crate::System::IDisposable> for crate::Oculus::Haptics::Haptics {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
