#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ShelfToolData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _toolType: crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType,
    pub _toolTip: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _iconPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _helpURL: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _targets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _jsonPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ShelfToolData";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl crate::HoudiniEngineUnity::HEU_ShelfToolData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
    pub type ToolType = crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_ShelfToolData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ShelfToolData_ToolType {
    #[default]
    BATCH = 3i32,
    GENERATOR = 0i32,
    OPERATOR_MULTI = 2i32,
    OPERATOR_SINGLE = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "ToolType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ShelfToolData+ToolType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_ShelfToolData_ToolType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
