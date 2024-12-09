#[cfg(feature = "TMPro+FastAction_3")]
#[repr(C)]
#[derive(Debug)]
pub struct FastAction_3<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub delegates: *mut crate::System::Collections::Generic::LinkedList_1<
        *mut crate::System::Action_3<A, B, C>,
    >,
    pub lookup: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Action_3<A, B, C>,
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::System::Action_3<A, B, C>,
        >,
    >,
    __cordl_phantom_A: std::marker::PhantomData<A>,
    __cordl_phantom_B: std::marker::PhantomData<B>,
    __cordl_phantom_C: std::marker::PhantomData<C>,
}
#[cfg(feature = "TMPro+FastAction_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::FastAction_3 < A, B, C > => "TMPro"
    ."FastAction`3" < A, B, C >
);
#[cfg(feature = "TMPro+FastAction_3")]
impl<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::TMPro::FastAction_3<A, B, C> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+FastAction_3")]
impl<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::TMPro::FastAction_3<A, B, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+FastAction_3")]
impl<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> crate::TMPro::FastAction_3<A, B, C> {
    pub fn Add(
        &mut self,
        rhs: *mut crate::System::Action_3<A, B, C>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (rhs))?;
        Ok(__cordl_ret)
    }
    pub fn Call(
        &mut self,
        a: A,
        b: B,
        c: C,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Call", (a, b, c))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        rhs: *mut crate::System::Action_3<A, B, C>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (rhs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+FastAction_3")]
impl<
    A: quest_hook::libil2cpp::Type,
    B: quest_hook::libil2cpp::Type,
    C: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::TMPro::FastAction_3<A, B, C> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
