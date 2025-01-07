#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
#[repr(C)]
#[derive(Debug)]
pub struct IPolynomialExtensionField {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.Field";
    const CLASS_NAME: &'static str = "IPolynomialExtensionField";
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
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_MinimalPolynomial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Field::IPolynomial>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IPolynomial,
        > = __cordl_object.invoke("get_MinimalPolynomial", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IExtensionField>
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::Field::IExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IExtensionField>
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::Field::IExtensionField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl AsRef<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Field+IPolynomialExtensionField")]
impl AsMut<crate::Org::BouncyCastle::Math::Field::IFiniteField>
for crate::Org::BouncyCastle::Math::Field::IPolynomialExtensionField {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Math::Field::IFiniteField {
        unsafe { std::mem::transmute(self) }
    }
}
