#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter {
    __cordl_parent: crate::System::Xml::Serialization::XmlSerializationWriter,
    pub _typeMap: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlMapping,
    >,
    pub _format: crate::System::Xml::Serialization::SerializationFormat,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlSerializationWriterInterpreter";
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
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    type Target = crate::System::Xml::Serialization::XmlSerializationWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    #[cfg(
        feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
    )]
    pub type CallbackInfo = crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo;
    pub fn GetEnumXmlValue(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetEnumXmlValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEnumXmlValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (typeMap, ob)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetListCount(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::TypeData,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                i32,
                2usize,
            >("GetListCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetListCount", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (listType, ob)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapMember,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                3usize,
            >("GetMemberValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMemberValue", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (member, ob, isValueList)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::TypeData,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::TypeData,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("GetStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringValue", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (typeMap, _cordl_type, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn ImplicitConvert(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("ImplicitConvert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ImplicitConvert", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked((), (obj, _cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn InitCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InitCallbacks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitCallbacks", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn MemberHasValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMember,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapMember,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                bool,
                3usize,
            >("MemberHasValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MemberHasValue", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (member, ob, isValueList))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        typeMap: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeMap))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteAnyElementContent(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
        >,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapMemberAnyElement,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteAnyElementContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteAnyElementContent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (member, memberValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteAttributeMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::ClassMap,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteAttributeMembers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteAttributeMembers", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map, ob, isValueList))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteElementMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::ClassMap,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteElementMembers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteElementMembers", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map, ob, isValueList))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEnumElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteEnumElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteEnumElement", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob, element, namesp))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteListContent(
        &mut self,
        container: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        listType: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ListMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetString: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::TypeData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::ListMap,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("WriteListContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteListContent", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, listType, map, ob, targetString))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteListElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteListElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteListElement", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob, element, namesp))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMemberElement(
        &mut self,
        elem: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapElementInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteMemberElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteMemberElement", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (elem, memberValue))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMembers(
        &mut self,
        map: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::ClassMap>,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        isValueList: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::ClassMap,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteMembers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteMembers", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (map, ob, isValueList))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMessage(
        &mut self,
        membersMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlMembersMapping,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlMembersMapping,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (membersMap, parameters))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isNullable: bool,
        needType: bool,
        writeWrappingElem: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("WriteObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObject", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        typeMap,
                        ob,
                        element,
                        namesp,
                        isNullable,
                        needType,
                        writeWrappingElem,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteObjectElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObjectElement", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob, element, namesp))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElementAttributes(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteObjectElementAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObjectElementAttributes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectElementElements(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteObjectElementElements")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObjectElementElements", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveElement(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        element: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        namesp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WritePrimitiveElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WritePrimitiveElement", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap, ob, element, namesp))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveValueEncoded(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xsiType: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        mappedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        wrapped: bool,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::TypeData,
                    >,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("WritePrimitiveValueEncoded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WritePrimitiveValueEncoded", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        memberValue,
                        name,
                        ns,
                        xsiType,
                        mappedType,
                        typeData,
                        wrapped,
                        isNullable,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WritePrimitiveValueLiteral(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mappedType: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
        typeData: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::TypeData>,
        wrapped: bool,
        isNullable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::TypeData,
                    >,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("WritePrimitiveValueLiteral")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WritePrimitiveValueLiteral", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (memberValue, name, ns, mappedType, typeData, wrapped, isNullable),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteRoot(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("WriteRoot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteRoot", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ob))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        typeMap: quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlMapping>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Serialization::XmlMapping,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (typeMap))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationWriterInterpreter_CallbackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _swi: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
    >,
    pub _typeMap: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlTypeMapping,
    >,
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlSerializationWriterInterpreter/CallbackInfo";
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
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    pub fn New(
        swi: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (swi, typeMap))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteEnum(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("WriteEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ob))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("WriteObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ob))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        swi: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
        >,
        typeMap: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapping,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlSerializationWriterInterpreter,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Serialization::XmlTypeMapping,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (swi, typeMap))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Xml+Serialization+XmlSerializationWriterInterpreter+CallbackInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationWriterInterpreter_CallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
