#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct PemWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    pub nlLength: i32,
    pub buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Utilities.IO.Pem";
    const CLASS_NAME: &'static str = "PemWriter";
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
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    pub const LineLength: i32 = 64i32;
    pub fn GetOutputSize(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputSize", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer))?;
        Ok(__cordl_object.into())
    }
    pub fn WriteEncoded(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEncoded", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObject(
        &mut self,
        objGen: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (objGen))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePostEncapsulationBoundary(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePostEncapsulationBoundary", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn WritePreEncapsulationBoundary(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WritePreEncapsulationBoundary", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Writer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter> = __cordl_object
            .invoke("get_Writer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+IO+Pem+PemWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::IO::Pem::PemWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
