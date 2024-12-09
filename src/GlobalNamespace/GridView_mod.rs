#[cfg(feature = "GridView")]
#[repr(C)]
#[derive(Debug)]
pub struct GridView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _contentTransform: *mut crate::UnityEngine::RectTransform,
    pub _columnCount: i32,
    pub _visibleColumnCount: i32,
    pub _cellsEnumerator_k__BackingField: *mut crate::GlobalNamespace::GridView_GridViewCellsEnumerator,
    pub _dataSource: *mut crate::GlobalNamespace::GridView_IDataSource,
    pub _rowCount: i32,
    pub _availableCellsPerPrefabDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::MonoBehaviour,
        *mut crate::System::Collections::Generic::Queue_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    >,
    pub _spawnedCellsPerPrefabDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::MonoBehaviour,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    >,
}
#[cfg(feature = "GridView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GridView => ""."GridView"
);
#[cfg(feature = "GridView")]
impl std::ops::Deref for crate::GlobalNamespace::GridView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GridView")]
impl std::ops::DerefMut for crate::GlobalNamespace::GridView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GridView")]
impl crate::GlobalNamespace::GridView {
    #[cfg(feature = "GridView+GridViewCellsEnumerator")]
    pub type GridViewCellsEnumerator = crate::GlobalNamespace::GridView_GridViewCellsEnumerator;
    #[cfg(feature = "GridView+IDataSource")]
    type IDataSource = crate::GlobalNamespace::GridView_IDataSource;
    pub fn GetActiveCellsForIdentifier(
        &mut self,
        prefab: *mut crate::UnityEngine::MonoBehaviour,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::MonoBehaviour,
        > = __cordl_object.invoke("GetActiveCellsForIdentifier", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn GetReusableCellView<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::MonoBehaviour,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetReusableCellView", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReloadData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDataSource(
        &mut self,
        newDataSource: *mut crate::GlobalNamespace::GridView_IDataSource,
        reloadData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataSource", (newDataSource, reloadData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cellsEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::GridView_GridViewCellsEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::GridView_GridViewCellsEnumerator = __cordl_object
            .invoke("get_cellsEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_columnCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_columnCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dataSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::GridView_IDataSource,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::GridView_IDataSource = __cordl_object
            .invoke("get_dataSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rowCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_rowCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visibleColumnCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_visibleColumnCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_cellsEnumerator(
        &mut self,
        value: *mut crate::GlobalNamespace::GridView_GridViewCellsEnumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cellsEnumerator", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GridView")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GridView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct GridView_GridViewCellsEnumerator {
    __cordl_parent: crate::System::Object,
    pub _gridView: *mut crate::GlobalNamespace::GridView,
}
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GridView_GridViewCellsEnumerator => ""
    ."GridView/GridViewCellsEnumerator"
);
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
impl std::ops::Deref for crate::GlobalNamespace::GridView_GridViewCellsEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::GridView_GridViewCellsEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
impl crate::GlobalNamespace::GridView_GridViewCellsEnumerator {
    #[cfg(feature = "GridView+GridViewCellsEnumerator+_GetEnumerator_d__1")]
    pub type _GetEnumerator_d__1 = crate::GlobalNamespace::GridViewCellsEnumerator_GridView__GetEnumerator_d__1;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::MonoBehaviour,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::MonoBehaviour,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        gridView: *mut crate::GlobalNamespace::GridView,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (gridView))?;
        Ok(__cordl_object)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        gridView: *mut crate::GlobalNamespace::GridView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (gridView))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GridView+GridViewCellsEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GridView_GridViewCellsEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GridView+IDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct GridView_IDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "GridView+IDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GridView_IDataSource => ""
    ."GridView/IDataSource"
);
#[cfg(feature = "GridView+IDataSource")]
impl std::ops::Deref for crate::GlobalNamespace::GridView_IDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GridView+IDataSource")]
impl std::ops::DerefMut for crate::GlobalNamespace::GridView_IDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GridView+IDataSource")]
impl crate::GlobalNamespace::GridView_IDataSource {
    pub fn CellForIdx(
        &mut self,
        gridView: *mut crate::GlobalNamespace::GridView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::MonoBehaviour> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::MonoBehaviour = __cordl_object
            .invoke("CellForIdx", (gridView, idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetCellHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCellWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "GridView+IDataSource")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GridView_IDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
