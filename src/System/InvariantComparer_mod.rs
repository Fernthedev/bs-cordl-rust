#[cfg(feature = "System+InvariantComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct InvariantComparer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_compareInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CompareInfo,
    >,
}
#[cfg(feature = "System+InvariantComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::InvariantComparer => "System"
    ."InvariantComparer"
);
#[cfg(feature = "System+InvariantComparer")]
impl std::ops::Deref for crate::System::InvariantComparer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+InvariantComparer")]
impl std::ops::DerefMut for crate::System::InvariantComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+InvariantComparer")]
impl crate::System::InvariantComparer {
    pub fn Compare(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (a, b))?;
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "System+InvariantComparer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::InvariantComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+InvariantComparer")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>>
for crate::System::InvariantComparer {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+InvariantComparer")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>>
for crate::System::InvariantComparer {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer> {
        unsafe { std::mem::transmute(self) }
    }
}
