#[cfg(feature = "System+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct Number {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Number")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Number {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Number";
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
#[cfg(feature = "System+Number")]
impl std::ops::Deref for crate::System::Number {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl std::ops::DerefMut for crate::System::Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl crate::System::Number {
    #[cfg(feature = "System+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Number_NumberBuffer;
    pub fn DecimalToNumber(
        value: crate::System::Decimal,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Decimal,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DecimalToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecimalToNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DigitsToInt(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                u32,
                2usize,
            >("DigitsToInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DigitsToInt", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (p, count)) };
        Ok(__cordl_ret.into())
    }
    pub fn DoubleToNumber(
        value: f64,
        precision: i32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f64,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DoubleToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoubleToNumber", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, precision, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindSection(
        format: crate::System::ReadOnlySpan_1<char>,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>, i32),
                i32,
                2usize,
            >("FindSection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindSection", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (format, section)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatCurrency(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("FormatCurrency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatCurrency", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDecimal(
        value: crate::System::Decimal,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Decimal,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatDecimal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDouble_ByRefMut_f64_ReadOnlySpan_1_NumberFormatInfo1(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        value: f64,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    f64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("FormatDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatDouble", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (sb, value, format, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDouble_f64_Il2CppString_NumberFormatInfo0(
        value: f64,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f64,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatDouble", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatExponent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        value: i32,
        expChar: char,
        minDigits: i32,
        positiveSign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    i32,
                    char,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("FormatExponent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatExponent", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (sb, info, value, expChar, minDigits, positiveSign),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatFixed(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        groupDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        sDecimal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("FormatFixed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatFixed", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sb,
                        number,
                        nMinDigits,
                        nMaxDigits,
                        info,
                        groupDigits,
                        sDecimal,
                        sGroup,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatGeneral(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
        bSuppressScientific: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    char,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("FormatGeneral")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatGeneral", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        sb,
                        number,
                        nMinDigits,
                        nMaxDigits,
                        info,
                        expChar,
                        bSuppressScientific,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatInt32(
        value: i32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatInt32", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatInt64(
        value: i64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatInt64", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatNumber(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("FormatNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatNumber", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatPercent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("FormatPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatPercent", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, nMinDigits, nMaxDigits, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatScientific(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    char,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("FormatScientific")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatScientific", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (sb, number, nMinDigits, nMaxDigits, info, expChar),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatSingle_ByRefMut_f32_ReadOnlySpan_1_NumberFormatInfo1(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        value: f32,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    f32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                4usize,
            >("FormatSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatSingle", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (sb, value, format, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatSingle_f32_Il2CppString_NumberFormatInfo0(
        value: f32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatSingle", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, info)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatUInt32(
        value: u32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatUInt32", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn FormatUInt64(
        value: u64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("FormatUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FormatUInt64", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn High32(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u32, 1usize>("High32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "High32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToHexChars(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u32,
        hexBase: i32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    u32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("Int32ToHexChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int32ToHexChars", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (buffer, value, hexBase, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToHexStr(
        value: i32,
        hexBase: char,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, char, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("Int32ToHexStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int32ToHexStr", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, hexBase, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToNumber(
        value: i32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Int32ToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int32ToNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Int64DivMod1E9(
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<u64>),
                u32,
                1usize,
            >("Int64DivMod1E9")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int64DivMod1E9", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Int64ToHexStr(
        value: i64,
        hexBase: char,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, char, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("Int64ToHexStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int64ToHexStr", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, hexBase, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn Int64ToNumber(
        input: i64,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Int64ToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Int64ToNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (input, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("IsDigit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsDigit", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ch)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsWhite(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("IsWhite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsWhite", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (ch)) };
        Ok(__cordl_ret.into())
    }
    pub fn Low32(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u32, 1usize>("Low32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Low32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("MatchChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MatchChars", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (p, pEnd, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Mul32x32To64(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u64, 2usize>("Mul32x32To64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Mul32x32To64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn Mul64Lossy(
        a: u64,
        b: u64,
        pexp: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, u64, quest_hook::libil2cpp::ByRefMut<i32>),
                u64,
                3usize,
            >("Mul64Lossy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Mul64Lossy", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, b, pexp)) };
        Ok(__cordl_ret.into())
    }
    pub fn NegativeInt32ToDecStr(
        value: i32,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("NegativeInt32ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NegativeInt32ToDecStr", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, digits, sNegative)) };
        Ok(__cordl_ret.into())
    }
    pub fn NegativeInt64ToDecStr(
        input: i64,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("NegativeInt64ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NegativeInt64ToDecStr", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (input, digits, sNegative)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberBufferToDecimal(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
                ),
                bool,
                2usize,
            >("NumberBufferToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberBufferToDecimal", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberBufferToDouble(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<f64>,
                ),
                bool,
                2usize,
            >("NumberBufferToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberBufferToDouble", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToDouble(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>),
                f64,
                1usize,
            >("NumberToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (number)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToInt32(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                2usize,
            >("NumberToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToInt32", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToInt64(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<i64>,
                ),
                bool,
                2usize,
            >("NumberToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToInt64", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToString(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        format: char,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        isDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    char,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("NumberToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToString", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (sb, number, format, nMaxDigits, info, isDecimal))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToStringFormat(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Text::ValueStringBuilder,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("NumberToStringFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToStringFormat", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, number, format, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToUInt32(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                ),
                bool,
                2usize,
            >("NumberToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToUInt32", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn NumberToUInt64(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                ),
                bool,
                2usize,
            >("NumberToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NumberToUInt64", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (number, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDecimal(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                crate::System::Decimal,
                3usize,
            >("ParseDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseDecimal", 3usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseDouble(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                f64,
                3usize,
            >("ParseDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseDouble", 3usize
                )
            });
        let __cordl_ret: f64 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseFormatSpecifier(
        format: crate::System::ReadOnlySpan_1<char>,
        digits: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                char,
                2usize,
            >("ParseFormatSpecifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseFormatSpecifier", 2usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked((), (format, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn ParseInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                i32,
                3usize,
            >("ParseInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseInt32", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                i64,
                3usize,
            >("ParseInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseInt64", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumber(
        str: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        strEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    bool,
                ),
                bool,
                6usize,
            >("ParseNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseNumber", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked((), (str, strEnd, styles, number, info, parseDecimal))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSingle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                f32,
                3usize,
            >("ParseSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseSingle", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseUInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                u32,
                3usize,
            >("ParseUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseUInt32", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseUInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                ),
                u64,
                3usize,
            >("ParseUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseUInt64", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe {
            method.invoke_unchecked((), (value, styles, info))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RoundNumber(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RoundNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RoundNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (number, pos))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StringToNumber(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("StringToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StringToNumber", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, styles, number, info, parseDecimal))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowOrFormatException(
        overflow: bool,
        overflowResourceKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ThrowOverflowOrFormatException")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ThrowOverflowOrFormatException", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (overflow, overflowResourceKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrailingZeros(
        value: crate::System::ReadOnlySpan_1<char>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>, i32),
                bool,
                2usize,
            >("TrailingZeros")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrailingZeros", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryCopyTo(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                3usize,
            >("TryCopyTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryCopyTo", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (source, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatDecimal(
        value: crate::System::Decimal,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Decimal,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatDecimal", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, format, info, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatDouble(
        value: f64,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatDouble", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, format, info, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatInt32(
        value: i32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatInt32", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, format, provider, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatInt64(
        value: i64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatInt64", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, format, provider, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatSingle(
        value: f32,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatSingle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, format, info, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatUInt32(
        value: u32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u32,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatUInt32", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, format, provider, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatUInt64(
        value: u64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryFormatUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormatUInt64", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, format, provider, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryInt32ToHexStr(
        value: i32,
        hexBase: char,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    char,
                    i32,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryInt32ToHexStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryInt32ToHexStr", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, hexBase, digits, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryInt64ToHexStr(
        value: i64,
        hexBase: char,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    char,
                    i32,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryInt64ToHexStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryInt64ToHexStr", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, hexBase, digits, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryNegativeInt32ToDecStr(
        value: i32,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryNegativeInt32ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryNegativeInt32ToDecStr", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, digits, sNegative, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryNegativeInt64ToDecStr(
        input: i64,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i64,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                5usize,
            >("TryNegativeInt64ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryNegativeInt64ToDecStr", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (input, digits, sNegative, destination, charsWritten),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDecimal(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
                ),
                bool,
                4usize,
            >("TryParseDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseDecimal", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDouble(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<f64>,
                ),
                bool,
                4usize,
            >("TryParseDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseDouble", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("TryParseInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseInt32", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt32IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseInt32IntegerStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseInt32IntegerStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i64>,
                ),
                bool,
                4usize,
            >("TryParseInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseInt64", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt64IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<i64>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseInt64IntegerStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseInt64IntegerStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseSingle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                4usize,
            >("TryParseSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseSingle", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                ),
                bool,
                4usize,
            >("TryParseUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt32", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32HexNumberStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseUInt32HexNumberStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt32HexNumberStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u32>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseUInt32IntegerStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt32IntegerStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                ),
                bool,
                4usize,
            >("TryParseUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt64", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64HexNumberStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseUInt64HexNumberStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt64HexNumberStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<u64>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                5usize,
            >("TryParseUInt64IntegerStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseUInt64IntegerStyle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, info, result, failureIsOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToNumber(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    bool,
                ),
                bool,
                5usize,
            >("TryStringToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryStringToNumber", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, styles, number, info, parseDecimal))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryUInt32ToDecStr(
        value: u32,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u32,
                    i32,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("TryUInt32ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryUInt32ToDecStr", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, digits, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryUInt64ToDecStr(
        value: u64,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    i32,
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                4usize,
            >("TryUInt64ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryUInt64ToDecStr", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (value, digits, destination, charsWritten))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToDecChars(
        bufferEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    u32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("UInt32ToDecChars")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UInt32ToDecChars", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (bufferEnd, value, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToDecStr(
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("UInt32ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UInt32ToDecStr", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToNumber(
        value: u32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UInt32ToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UInt32ToNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UInt64ToDecStr(
        value: u64,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("UInt64ToDecStr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UInt64ToDecStr", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, digits)) };
        Ok(__cordl_ret.into())
    }
    pub fn UInt64ToNumber(
        value: u64,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u64,
                    quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UInt64ToNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UInt64ToNumber", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value, number))
        };
        Ok(__cordl_ret.into())
    }
    pub fn abs(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i32, 1usize>("abs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "abs", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Number")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Number/NumberBuffer/DigitsAndNullTerminator";
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
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
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
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
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
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
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
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
impl crate::System::NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Number_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub _sign: i32,
    pub _digits: crate::System::NumberBuffer_Number_DigitsAndNullTerminator,
    pub _allDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Number_NumberBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Number/NumberBuffer";
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
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Number_NumberBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Number_NumberBuffer {
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
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Number_NumberBuffer {
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
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Number_NumberBuffer {
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
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Number_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer")]
impl crate::System::Number_NumberBuffer {
    #[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
    pub type DigitsAndNullTerminator = crate::System::NumberBuffer_Number_DigitsAndNullTerminator;
    pub fn get_digits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_digits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_digits", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_sign(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_sign")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sign", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_sign(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_sign")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_sign", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
