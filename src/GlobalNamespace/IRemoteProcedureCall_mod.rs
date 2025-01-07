#[cfg(feature = "IRemoteProcedureCall")]
#[repr(C)]
#[derive(Debug)]
pub struct IRemoteProcedureCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IRemoteProcedureCall")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IRemoteProcedureCall {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IRemoteProcedureCall";
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
#[cfg(feature = "IRemoteProcedureCall")]
impl std::ops::Deref for crate::GlobalNamespace::IRemoteProcedureCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl std::ops::DerefMut for crate::GlobalNamespace::IRemoteProcedureCall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl crate::GlobalNamespace::IRemoteProcedureCall {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IRemoteProcedureCall")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::IRemoteProcedureCall {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
