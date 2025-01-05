#[cfg(feature = "System+TupleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+TupleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TupleExtensions => "System"
    ."TupleExtensions"
);
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::Deref for crate::System::TupleExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl std::ops::DerefMut for crate::System::TupleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl crate::System::TupleExtensions {
    pub fn Deconstruct<T1, T2>(
        value: quest_hook::libil2cpp::Gc<T1, T2>,
        item1: quest_hook::libil2cpp::ByRefMut<T1>,
        item2: quest_hook::libil2cpp::ByRefMut<T2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deconstruct", (value, item1, item2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TupleExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TupleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
