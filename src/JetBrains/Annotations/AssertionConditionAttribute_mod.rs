#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertionConditionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ConditionType_k__BackingField: crate::JetBrains::Annotations::AssertionConditionType,
}
#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::JetBrains::Annotations::AssertionConditionAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "JetBrains.Annotations";
    const CLASS_NAME: &'static str = "AssertionConditionAttribute";
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
#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
impl std::ops::Deref for crate::JetBrains::Annotations::AssertionConditionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
impl std::ops::DerefMut for crate::JetBrains::Annotations::AssertionConditionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
impl crate::JetBrains::Annotations::AssertionConditionAttribute {
    pub fn New(
        conditionType: crate::JetBrains::Annotations::AssertionConditionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (conditionType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        conditionType: crate::JetBrains::Annotations::AssertionConditionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::JetBrains::Annotations::AssertionConditionType),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (conditionType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ConditionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::JetBrains::Annotations::AssertionConditionType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::JetBrains::Annotations::AssertionConditionType,
                0usize,
            >("get_ConditionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ConditionType", 0usize
                )
            });
        let __cordl_ret: crate::JetBrains::Annotations::AssertionConditionType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ConditionType(
        &mut self,
        value: crate::JetBrains::Annotations::AssertionConditionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::JetBrains::Annotations::AssertionConditionType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ConditionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ConditionType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "JetBrains+Annotations+AssertionConditionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::JetBrains::Annotations::AssertionConditionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
