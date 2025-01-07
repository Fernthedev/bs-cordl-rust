#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Utility";
    const CLASS_NAME: &'static str = "SerializationUtilities";
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
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
    )]
    pub type ObjectType = crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType;
    pub fn ReadInt32FromByteArray(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt32FromByteArray", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadObjectFromByteArray(
        keyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadObjectFromByteArray", (keyData, dataIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteInt32ToByteArray(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        val: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteInt32ToByteArray", (data, val, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectToByteList(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteObjectToByteList", (obj, buffer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SerializationUtilities_ObjectType {
    #[default]
    AsciiString = 0i32,
    Hash128 = 5i32,
    Int32 = 4i32,
    JsonObject = 7i32,
    Type = 6i32,
    UInt16 = 2i32,
    UInt32 = 3i32,
    UnicodeString = 1i32,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Utility+SerializationUtilities+ObjectType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Utility";
    const CLASS_NAME: &'static str = "ObjectType";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::AddressableAssets::Utility::SerializationUtilities_ObjectType {
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
