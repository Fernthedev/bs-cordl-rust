#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
#[repr(C)]
#[derive(Debug)]
pub struct FileBasedResourceGroveler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _mediator: quest_hook::libil2cpp::Gc<
        crate::System::Resources::ResourceManager_ResourceManagerMediator,
    >,
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Resources::FileBasedResourceGroveler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Resources";
    const CLASS_NAME: &'static str = "FileBasedResourceGroveler";
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
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl std::ops::Deref for crate::System::Resources::FileBasedResourceGroveler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl std::ops::DerefMut for crate::System::Resources::FileBasedResourceGroveler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl crate::System::Resources::FileBasedResourceGroveler {
    pub fn New(
        mediator: quest_hook::libil2cpp::Gc<
            crate::System::Resources::ResourceManager_ResourceManagerMediator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mediator))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mediator: quest_hook::libil2cpp::Gc<
            crate::System::Resources::ResourceManager_ResourceManagerMediator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mediator))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Resources::FileBasedResourceGroveler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl AsRef<crate::System::Resources::IResourceGroveler>
for crate::System::Resources::FileBasedResourceGroveler {
    fn as_ref(&self) -> &crate::System::Resources::IResourceGroveler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Resources+FileBasedResourceGroveler")]
impl AsMut<crate::System::Resources::IResourceGroveler>
for crate::System::Resources::FileBasedResourceGroveler {
    fn as_mut(&mut self) -> &mut crate::System::Resources::IResourceGroveler {
        unsafe { std::mem::transmute(self) }
    }
}
