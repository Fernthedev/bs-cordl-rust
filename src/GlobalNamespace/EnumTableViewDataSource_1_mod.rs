#[cfg(feature = "EnumTableViewDataSource_1")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumTableViewDataSource_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _cellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TextOnlyTableCell,
    >,
    pub _cellHeight: f32,
    pub _values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "EnumTableViewDataSource_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EnumTableViewDataSource`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("", "EnumTableViewDataSource`1")
                    .unwrap()
                    .make_generic::<(T)>()
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
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    pub const kCellReuseIdentifier: &'static str = "Cell";
    pub fn CellForIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIdForValue(&mut self, value: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIdForValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLabelForId(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLabelForId", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLabelForValue(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLabelForValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueForId(&mut self, id: i32) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetValueForId", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
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
}
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    fn as_ref(&self) -> &crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnumTableViewDataSource_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::EnumTableViewDataSource_1<T> {
    fn as_mut(&mut self) -> &mut crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
