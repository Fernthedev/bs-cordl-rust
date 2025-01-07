#[cfg(feature = "System+Xml+Linq+ElementWriter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElementWriter {
    pub _writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    pub _resolver: crate::System::Xml::Linq::NamespaceResolver,
}
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Linq::ElementWriter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "ElementWriter";
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
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::Linq::ElementWriter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Linq::ElementWriter {
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
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::Linq::ElementWriter {
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
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::Linq::ElementWriter {
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
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Linq::ElementWriter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Linq+ElementWriter")]
impl crate::System::Xml::Linq::ElementWriter {
    pub fn GetPrefixOfNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XNamespace>,
        allowDefaultNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetPrefixOfNamespace",
            (ns, allowDefaultNamespace),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PushAncestors(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushAncestors",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PushElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PushElement",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteElement",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteEndElement",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFullEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteFullEndElement",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::Linq::XElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteStartElement",
            (e),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (writer),
        )?;
        Ok(__cordl_ret.into())
    }
}
