#[cfg(feature = "UnityEngine+GUITargetAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct GUITargetAttribute {
    __cordl_parent: crate::System::Attribute,
    pub displayMask: i32,
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::GUITargetAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "GUITargetAttribute";
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
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl std::ops::Deref for crate::UnityEngine::GUITargetAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::GUITargetAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl crate::UnityEngine::GUITargetAttribute {
    pub fn GetGUITargetAttrValue(
        klass: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGUITargetAttrValue", (klass, methodName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUITargetAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUITargetAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
