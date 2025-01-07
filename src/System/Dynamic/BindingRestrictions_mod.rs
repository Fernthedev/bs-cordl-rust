#[cfg(feature = "System+Dynamic+BindingRestrictions")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Dynamic::BindingRestrictions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "BindingRestrictions";
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
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl std::ops::Deref for crate::System::Dynamic::BindingRestrictions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl std::ops::DerefMut for crate::System::Dynamic::BindingRestrictions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl crate::System::Dynamic::BindingRestrictions {
    #[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
    pub type BindingRestrictionsProxy = crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+CustomRestriction")]
    pub type CustomRestriction = crate::GlobalNamespace::BindingRestrictions_CustomRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+InstanceRestriction")]
    pub type InstanceRestriction = crate::GlobalNamespace::BindingRestrictions_InstanceRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+MergedRestriction")]
    pub type MergedRestriction = crate::GlobalNamespace::BindingRestrictions_MergedRestriction;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
    pub type TestBuilder = crate::System::Dynamic::BindingRestrictions_TestBuilder;
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TypeRestriction")]
    pub type TypeRestriction = crate::GlobalNamespace::BindingRestrictions_TypeRestriction;
    pub fn GetExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetExpression", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceRestriction(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstanceRestriction", (expression, instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeRestriction_DynamicMetaObject1(
        obj: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeRestriction", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeRestriction_Expression_Type0(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeRestriction", (expression, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Merge(
        &mut self,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = __cordl_object.invoke("Merge", (restrictions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("ToExpression", ())?;
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
}
#[cfg(feature = "System+Dynamic+BindingRestrictions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Dynamic::BindingRestrictions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_BindingRestrictionsProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "BindingRestrictionsProxy";
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl std::ops::Deref
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl std::ops::DerefMut
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {}
#[cfg(feature = "System+Dynamic+BindingRestrictions+BindingRestrictionsProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::BindingRestrictions_BindingRestrictionsProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingRestrictions_TestBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _unique: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
        >,
    >,
    pub _tests: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode,
        >,
    >,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "TestBuilder";
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl std::ops::Deref for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl std::ops::DerefMut for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl crate::System::Dynamic::BindingRestrictions_TestBuilder {
    #[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
    pub type AndNode = crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode;
    pub fn Append(
        &mut self,
        restrictions: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Append", (restrictions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Push(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", (node, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToExpression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("ToExpression", ())?;
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
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::BindingRestrictions_TestBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestBuilder_BindingRestrictions_AndNode {
    pub Depth: i32,
    pub Node: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Dynamic";
    const CLASS_NAME: &'static str = "AndNode";
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
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
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Dynamic+BindingRestrictions+TestBuilder+AndNode")]
impl crate::System::Dynamic::TestBuilder_BindingRestrictions_AndNode {}
