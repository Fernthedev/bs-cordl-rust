#[cfg(feature = "Zenject+InjectableInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectableInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Optional: bool,
    pub Identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub SourceType: crate::Zenject::InjectSources,
    pub MemberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub MemberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub DefaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+InjectableInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::InjectableInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InjectableInfo";
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
#[cfg(feature = "Zenject+InjectableInfo")]
impl std::ops::Deref for crate::Zenject::InjectableInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectableInfo")]
impl std::ops::DerefMut for crate::Zenject::InjectableInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectableInfo")]
impl crate::Zenject::InjectableInfo {
    pub fn New(
        optional: bool,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sourceType: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (optional, identifier, memberName, memberType, defaultValue, sourceType),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        optional: bool,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sourceType: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (optional, identifier, memberName, memberType, defaultValue, sourceType),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+InjectableInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InjectableInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
