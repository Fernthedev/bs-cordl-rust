#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct XRDeviceDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub deviceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub manufacturer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub serialNumber: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub characteristics: crate::UnityEngine::XR::InputDeviceCharacteristics,
    pub deviceId: i32,
    pub inputFeatures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::XR::XRFeatureDescriptor,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.XR";
    const CLASS_NAME: &'static str = "XRDeviceDescriptor";
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
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromJson", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToJson", ())?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRDeviceDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::XRDeviceDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
