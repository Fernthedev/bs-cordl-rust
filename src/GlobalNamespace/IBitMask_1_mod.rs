#[cfg(feature = "IBitMask_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IBitMask_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "IBitMask_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBitMask_1 < T > => ""
    ."IBitMask`1" < T >
);
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::IBitMask_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::IBitMask_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::IBitMask_1<T> {
    pub fn GetBits(
        &mut self,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u64>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetBits", (offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBits(&mut self, offset: i32, bits: u64) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("SetBits", (offset, bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_bitCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bitCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IBitMask_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IEquatable_1<T>>
for crate::GlobalNamespace::IBitMask_1<T> {
    fn as_ref(&self) -> &crate::System::IEquatable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBitMask_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IEquatable_1<T>>
for crate::GlobalNamespace::IBitMask_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
