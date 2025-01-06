#[cfg(feature = "ModestTree+MiscExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+MiscExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::MiscExtensions => "ModestTree"
    ."MiscExtensions"
);
#[cfg(feature = "ModestTree+MiscExtensions")]
impl std::ops::Deref for crate::ModestTree::MiscExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl std::ops::DerefMut for crate::ModestTree::MiscExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl crate::ModestTree::MiscExtensions {
    pub fn AllocFreeAddRange<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        items: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocFreeAddRange", (list, items))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fmt(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Fmt", (s, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueAndRemove<TKey, TVal>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<TKey, TVal>,
        >,
        key: TKey,
    ) -> quest_hook::libil2cpp::Result<TVal>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TVal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TVal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueAndRemove", (dictionary, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (list, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Join(
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Join", (values, separator))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveWithConfirm_HashSet_1_T3<T>(
        set: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<T>,
        >,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveWithConfirm", (set, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveWithConfirm_IDictionary_2_TKey2<TKey, TVal>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<TKey, TVal>,
        >,
        key: TKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TVal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveWithConfirm", (dictionary, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveWithConfirm_IList_1_T0<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveWithConfirm", (list, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveWithConfirm_LinkedList_1_T1<T>(
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<T>,
        >,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveWithConfirm", (list, item))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+MiscExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::MiscExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
