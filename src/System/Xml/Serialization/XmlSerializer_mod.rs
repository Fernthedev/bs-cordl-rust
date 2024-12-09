#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializer {
    __cordl_parent: crate::System::Object,
    pub customSerializer: bool,
    pub typeMapping: *mut crate::System::Xml::Serialization::XmlMapping,
    pub serializerData: *mut crate::System::Xml::Serialization::XmlSerializer_SerializerData,
    pub onUnreferencedObject: *mut crate::System::Xml::Serialization::UnreferencedObjectEventHandler,
    pub onUnknownAttribute: *mut crate::System::Xml::Serialization::XmlAttributeEventHandler,
    pub onUnknownElement: *mut crate::System::Xml::Serialization::XmlElementEventHandler,
    pub onUnknownNode: *mut crate::System::Xml::Serialization::XmlNodeEventHandler,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlSerializer =>
    "System.Xml.Serialization"."XmlSerializer"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl crate::System::Xml::Serialization::XmlSerializer {
    #[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
    pub type SerializerData = crate::System::Xml::Serialization::XmlSerializer_SerializerData;
    pub fn CreateReader_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationReader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationReader = __cordl_object
            .invoke("CreateReader", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateReader_XmlMapping1(
        &mut self,
        typeMapping: *mut crate::System::Xml::Serialization::XmlMapping,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationReader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationReader = __cordl_object
            .invoke("CreateReader", (typeMapping))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWriter_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationWriter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationWriter = __cordl_object
            .invoke("CreateWriter", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateWriter_XmlMapping1(
        &mut self,
        typeMapping: *mut crate::System::Xml::Serialization::XmlMapping,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationWriter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationWriter = __cordl_object
            .invoke("CreateWriter", (typeMapping))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_TextReader0(
        &mut self,
        textReader: *mut crate::System::IO::TextReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (textReader))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_XmlReader1(
        &mut self,
        xmlReader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (xmlReader))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_XmlSerializationReader2(
        &mut self,
        reader: *mut crate::System::Xml::Serialization::XmlSerializationReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        overrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        extraTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, overrides, extraTypes, root, defaultNamespace),
            )?;
        Ok(__cordl_object)
    }
    pub fn OnUnknownAttribute(
        &mut self,
        e: *mut crate::System::Xml::Serialization::XmlAttributeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownAttribute", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnUnknownElement(
        &mut self,
        e: *mut crate::System::Xml::Serialization::XmlElementEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownElement", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnUnknownNode(
        &mut self,
        e: *mut crate::System::Xml::Serialization::XmlNodeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnknownNode", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnUnreferencedObject(
        &mut self,
        e: *mut crate::System::Xml::Serialization::UnreferencedObjectEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnreferencedObject", (e))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_Object_XmlSerializationWriter0(
        &mut self,
        o: *mut crate::System::Object,
        writer: *mut crate::System::Xml::Serialization::XmlSerializationWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (o, writer))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_TextWriter_Object1(
        &mut self,
        textWriter: *mut crate::System::IO::TextWriter,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (textWriter, o))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_XmlWriter_Object2(
        &mut self,
        xmlWriter: *mut crate::System::Xml::XmlWriter,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (xmlWriter, o))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_XmlWriter_Object_XmlSerializerNamespaces3(
        &mut self,
        xmlWriter: *mut crate::System::Xml::XmlWriter,
        o: *mut crate::System::Object,
        namespaces: *mut crate::System::Xml::Serialization::XmlSerializerNamespaces,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (xmlWriter, o, namespaces))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        overrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        extraTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, overrides, extraTypes, root, defaultNamespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Mapping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlMapping = __cordl_object
            .invoke("get_Mapping", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializer_SerializerData {
    __cordl_parent: crate::System::Object,
    pub ReaderMethod: *mut crate::System::Reflection::MethodInfo,
    pub WriterType: *mut crate::System::Type,
    pub WriterMethod: *mut crate::System::Reflection::MethodInfo,
    pub Implementation: *mut crate::System::Xml::Serialization::XmlSerializerImplementation,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializer_SerializerData =>
    "System.Xml.Serialization"."XmlSerializer/SerializerData"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    pub fn CreateWriter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializationWriter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializationWriter = __cordl_object
            .invoke("CreateWriter", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializer+SerializerData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializer_SerializerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
