#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabResourceFactory_5<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: *mut crate::Zenject::DiContainer,
    __cordl_phantom_P1: std::marker::PhantomData<P1>,
    __cordl_phantom_P2: std::marker::PhantomData<P2>,
    __cordl_phantom_P3: std::marker::PhantomData<P3>,
    __cordl_phantom_P4: std::marker::PhantomData<P4>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabResourceFactory_5 < P1, P2, P3,
    P4, T > => "Zenject"."PrefabResourceFactory`5" < P1, P2, P3, P4, T >
);
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    pub fn Create(
        &mut self,
        prefabResourceName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        param: P1,
        param2: P2,
        param3: P3,
        param4: P4,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("Create", (prefabResourceName, param, param2, param3, param4))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    >
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IFactory>
for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn as_ref(&self) -> &crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IFactory>
for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsRef<
    crate::Zenject::IFactory_6<
        *mut quest_hook::libil2cpp::Il2CppString,
        P1,
        P2,
        P3,
        P4,
        T,
    >,
> for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn as_ref(
        &self,
    ) -> &crate::Zenject::IFactory_6<
        *mut quest_hook::libil2cpp::Il2CppString,
        P1,
        P2,
        P3,
        P4,
        T,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabResourceFactory_5")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    P4: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsMut<
    crate::Zenject::IFactory_6<
        *mut quest_hook::libil2cpp::Il2CppString,
        P1,
        P2,
        P3,
        P4,
        T,
    >,
> for crate::Zenject::PrefabResourceFactory_5<P1, P2, P3, P4, T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IFactory_6<
        *mut quest_hook::libil2cpp::Il2CppString,
        P1,
        P2,
        P3,
        P4,
        T,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
