#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfoScanner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_dateWords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _ymdFlags: crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::DateTimeFormatInfoScanner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "DateTimeFormatInfoScanner";
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
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl std::ops::Deref for crate::System::Globalization::DateTimeFormatInfoScanner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl std::ops::DerefMut for crate::System::Globalization::DateTimeFormatInfoScanner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl crate::System::Globalization::DateTimeFormatInfoScanner {
    #[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
    pub type FoundDatePattern = crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern;
    pub fn AddDateWordOrPostfix(
        &mut self,
        formatPostfix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDateWordOrPostfix", (formatPostfix, str))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddDateWords(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        formatPostfix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("AddDateWords", (pattern, index, formatPostfix))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddIgnorableSymbols(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddIgnorableSymbols", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayElementsBeginWithDigit(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayElementsBeginWithDigit", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayElementsHaveSpace(
        array: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayElementsHaveSpace", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn EqualStringArrays(
        array1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        array2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualStringArrays", (array1, array2))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateWordsOfDTFI(
        &mut self,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("GetDateWordsOfDTFI", (dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatFlagGenitiveMonth(
        monthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        genitveMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        abbrevMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        genetiveAbbrevMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::FORMATFLAGS> {
        let __cordl_ret: crate::System::Globalization::FORMATFLAGS = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetFormatFlagGenitiveMonth",
                (
                    monthNames,
                    genitveMonthNames,
                    abbrevMonthNames,
                    genetiveAbbrevMonthNames,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatFlagUseHebrewCalendar(
        calID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::FORMATFLAGS> {
        let __cordl_ret: crate::System::Globalization::FORMATFLAGS = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFormatFlagUseHebrewCalendar", (calID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatFlagUseSpaceInDayNames(
        dayNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        abbrevDayNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::FORMATFLAGS> {
        let __cordl_ret: crate::System::Globalization::FORMATFLAGS = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFormatFlagUseSpaceInDayNames", (dayNames, abbrevDayNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatFlagUseSpaceInMonthNames(
        monthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        genitveMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        abbrevMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        genetiveAbbrevMonthNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::FORMATFLAGS> {
        let __cordl_ret: crate::System::Globalization::FORMATFLAGS = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetFormatFlagUseSpaceInMonthNames",
                (
                    monthNames,
                    genitveMonthNames,
                    abbrevMonthNames,
                    genetiveAbbrevMonthNames,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ScanDateWord(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanDateWord", (pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanRepeatChar(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ch: char,
        index: i32,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScanRepeatChar", (pattern, ch, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipWhiteSpacesAndNonLetter(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        currentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SkipWhiteSpacesAndNonLetter", (pattern, currentIndex))?;
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
    pub fn get_KnownWords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_KnownWords", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::DateTimeFormatInfoScanner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateTimeFormatInfoScanner_FoundDatePattern {
    #[default]
    FoundDayPatternFlag = 4i32,
    FoundMonthPatternFlag = 2i32,
    FoundYMDPatternFlag = 7i32,
    FoundYearPatternFlag = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "FoundDatePattern";
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
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern {
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
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern {
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
#[cfg(feature = "System+Globalization+DateTimeFormatInfoScanner+FoundDatePattern")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::DateTimeFormatInfoScanner_FoundDatePattern {
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
