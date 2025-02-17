#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
#[repr(C)]
#[derive(Debug)]
pub struct LineInfoEndElementAnnotation {
    __cordl_parent: crate::System::Xml::Linq::LineInfoAnnotation,
}
#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Linq::LineInfoEndElementAnnotation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "LineInfoEndElementAnnotation";
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
#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
impl std::ops::Deref for crate::System::Xml::Linq::LineInfoEndElementAnnotation {
    type Target = crate::System::Xml::Linq::LineInfoAnnotation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
impl std::ops::DerefMut for crate::System::Xml::Linq::LineInfoEndElementAnnotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
impl crate::System::Xml::Linq::LineInfoEndElementAnnotation {
    pub fn New(
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lineNumber, linePosition))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (lineNumber, linePosition))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+LineInfoEndElementAnnotation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::LineInfoEndElementAnnotation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
