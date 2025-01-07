#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NearestNeighbourList_2<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub queue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::KdTree::PriorityQueue_2<TItem, TDistance>,
    >,
    pub distanceMath: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TDistance>,
    >,
    pub maxCapacity: i32,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TDistance: std::marker::PhantomData<TDistance>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
unsafe impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.KdTree";
    const CLASS_NAME: &'static str = "NearestNeighbourList`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.ProBuilder.KdTree",
                        "NearestNeighbourList`2",
                    )
                    .unwrap()
                    .make_generic::<(TItem, TDistance)>()
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
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    pub fn Add(
        &mut self,
        item: TItem,
        distance: TDistance,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Add", (item, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFurtherest(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("GetFurtherest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFurtherestDistance(&mut self) -> quest_hook::libil2cpp::Result<TDistance>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TDistance = __cordl_object.invoke("GetFurtherestDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        maxCapacity: i32,
        distanceMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TDistance>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxCapacity, distanceMath))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveFurtherest(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("RemoveFurtherest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        maxCapacity: i32,
        distanceMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TDistance>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxCapacity, distanceMath))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCapacityReached(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCapacityReached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxCapacity", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> AsRef<
    crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance>,
> for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<
        TItem,
        TDistance,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+NearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> AsMut<
    crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance>,
> for crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<TItem, TDistance> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<
        TItem,
        TDistance,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
