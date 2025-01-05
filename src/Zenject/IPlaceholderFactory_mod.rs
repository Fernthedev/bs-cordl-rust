#[cfg(feature = "Zenject+IPlaceholderFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlaceholderFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IPlaceholderFactory => "Zenject"
    ."IPlaceholderFactory"
);
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl std::ops::Deref for crate::Zenject::IPlaceholderFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl std::ops::DerefMut for crate::Zenject::IPlaceholderFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl crate::Zenject::IPlaceholderFactory {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IPlaceholderFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::IValidatable>>
for crate::Zenject::IPlaceholderFactory {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::IValidatable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+IPlaceholderFactory")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::IValidatable>>
for crate::Zenject::IPlaceholderFactory {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::IValidatable> {
        unsafe { std::mem::transmute(self) }
    }
}
