#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
#[repr(C)]
#[derive(Debug)]
pub struct DomainLiteralReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::DomainLiteralReader =>
    "System.Net.Mail"."DomainLiteralReader"
);
#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
impl std::ops::Deref for crate::System::Net::Mail::DomainLiteralReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::DomainLiteralReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
impl crate::System::Net::Mail::DomainLiteralReader {
    pub fn ReadReverse(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadReverse", (data, index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mail+DomainLiteralReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Mail::DomainLiteralReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
