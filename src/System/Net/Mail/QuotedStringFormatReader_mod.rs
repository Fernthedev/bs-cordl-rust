#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
#[repr(C)]
#[derive(Debug)]
pub struct QuotedStringFormatReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Mail::QuotedStringFormatReader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Mail";
    const CLASS_NAME: &'static str = "QuotedStringFormatReader";
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
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::Deref for crate::System::Net::Mail::QuotedStringFormatReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::QuotedStringFormatReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl crate::System::Net::Mail::QuotedStringFormatReader {
    pub fn IsValidQtext(
        allowUnicode: bool,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidQtext", (allowUnicode, ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadReverseQuoted(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        permitUnicode: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadReverseQuoted", (data, index, permitUnicode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadReverseUnQuoted(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        permitUnicode: bool,
        expectCommaDelimiter: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReadReverseUnQuoted",
                (data, index, permitUnicode, expectCommaDelimiter),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Mail::QuotedStringFormatReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
