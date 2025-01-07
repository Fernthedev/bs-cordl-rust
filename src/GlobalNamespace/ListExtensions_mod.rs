#[cfg(feature = "ListExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ListExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ListExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ListExtensions";
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
#[cfg(feature = "ListExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ListExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ListExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ListExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ListExtensions")]
impl crate::GlobalNamespace::ListExtensions {
    pub fn FindIndex<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        _cordl_match: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindIndex", (list, _cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (_cordl_self, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertIntoSortedListFromEnd<T>(
        sortedList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        newItem: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertIntoSortedListFromEnd", (sortedList, newItem))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ListExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
