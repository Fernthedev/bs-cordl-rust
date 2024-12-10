#[cfg(feature = "System+Net+Configuration+ProxyElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ProxyElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+ProxyElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Configuration::ProxyElement =>
    "System.Net.Configuration"."ProxyElement"
);
#[cfg(feature = "System+Net+Configuration+ProxyElement")]
impl std::ops::Deref for crate::System::Net::Configuration::ProxyElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+ProxyElement")]
impl std::ops::DerefMut for crate::System::Net::Configuration::ProxyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+ProxyElement")]
impl crate::System::Net::Configuration::ProxyElement {
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
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Configuration::ConfigurationPropertyCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Configuration::ConfigurationPropertyCollection,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Configuration+ProxyElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::ProxyElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
