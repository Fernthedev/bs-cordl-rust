#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerActivePlayersTimeOffsetAverage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _lastReturnedOffsetSyncTime: i64,
    pub _timeOfLastValidReturnedTime: i64,
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerActivePlayersTimeOffsetAverage";
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
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
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
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl AsRef<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerActivePlayersTimeOffsetAverage")]
impl AsMut<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerActivePlayersTimeOffsetAverage {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
