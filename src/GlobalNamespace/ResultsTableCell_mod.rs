#[cfg(feature = "ResultsTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct ResultsTableCell {
    __cordl_parent: crate::GlobalNamespace::TableCellWithSeparator,
    pub _border: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _orderText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _nameText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _scoreText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _rankText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
}
#[cfg(feature = "ResultsTableCell")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ResultsTableCell {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ResultsTableCell";
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
#[cfg(feature = "ResultsTableCell")]
impl std::ops::Deref for crate::GlobalNamespace::ResultsTableCell {
    type Target = crate::GlobalNamespace::TableCellWithSeparator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::ResultsTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsTableCell")]
impl crate::GlobalNamespace::ResultsTableCell {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetData(
        &mut self,
        order: i32,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (order, connectedPlayer, levelCompletionResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ResultsTableCell")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ResultsTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
