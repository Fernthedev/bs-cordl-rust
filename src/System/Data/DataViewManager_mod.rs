#[cfg(feature = "System+Data+DataViewManager")]
#[repr(C)]
#[derive(Debug)]
pub struct DataViewManager {
    __cordl_parent: crate::System::ComponentModel::MarshalByValueComponent,
    pub _dataViewSettingsCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataViewSettingCollection,
    >,
    pub _nViews: i32,
}
#[cfg(feature = "System+Data+DataViewManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::DataViewManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataViewManager";
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
#[cfg(feature = "System+Data+DataViewManager")]
impl std::ops::Deref for crate::System::Data::DataViewManager {
    type Target = crate::System::ComponentModel::MarshalByValueComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManager")]
impl std::ops::DerefMut for crate::System::Data::DataViewManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManager")]
impl crate::System::Data::DataViewManager {
    pub fn get_DataViewSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataViewSettingCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataViewSettingCollection,
        > = __cordl_object.invoke("get_DataViewSettings", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataViewManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataViewManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
