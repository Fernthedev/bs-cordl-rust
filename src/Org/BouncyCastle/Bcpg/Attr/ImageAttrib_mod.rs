#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
#[repr(C)]
#[derive(Debug)]
pub struct ImageAttrib {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket,
    pub hdrLength: i32,
    pub _version: i32,
    pub _encoding: i32,
    pub imageData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.Attr";
    const CLASS_NAME: &'static str = "ImageAttrib";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib {
    type Target = crate::Org::BouncyCastle::Bcpg::UserAttributeSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
impl crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib {
    #[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
    pub type Format = crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format;
    pub fn GetImageData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetImageData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ImageAttrib_Format_Il2CppArray2(
        imageType: crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format,
        imageData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (imageType, imageData))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_Il2CppArray1(
        forceLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (forceLongLength, data))?;
        Ok(__cordl_object.into())
    }
    pub fn ToByteArray(
        imageType: crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format,
        imageData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByteArray", (imageType, imageData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ImageAttrib_Format_Il2CppArray2(
        &mut self,
        imageType: crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format,
        imageData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (imageType, imageData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_Il2CppArray1(
        &mut self,
        forceLongLength: bool,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (forceLongLength, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Encoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageAttrib_Format {
    #[default]
    Jpeg = 1u8,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.Attr";
    const CLASS_NAME: &'static str = "Format";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format {
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format {
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+Attr+ImageAttrib+Format")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Bcpg::Attr::ImageAttrib_Format {
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
