#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupedEnumerable_3<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub source: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<TSource>,
    >,
    pub keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
    pub elementSelector: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<TSource, TElement>,
    >,
    pub comparer: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
    >,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "GroupedEnumerable`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "GroupedEnumerable`3",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TKey, TElement)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::IGrouping_2<TKey, TElement>,
                >,
            >,
        >,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::IGrouping_2<TKey, TElement>,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
        elementSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, TElement>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TSource>,
        >,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TSource, TKey>>,
        elementSelector: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TSource, TElement>,
        >,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (source, keySelector, elementSelector, comparer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IGrouping_2<TKey, TElement>>,
    >,
> for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IGrouping_2<TKey, TElement>>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IGrouping_2<TKey, TElement>>,
    >,
> for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IGrouping_2<TKey, TElement>>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+GroupedEnumerable_3")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::GroupedEnumerable_3<TSource, TKey, TElement> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
