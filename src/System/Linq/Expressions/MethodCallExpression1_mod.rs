#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodCallExpression1 {
    __cordl_parent: crate::System::Linq::Expressions::MethodCallExpression,
    pub _arg0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::MethodCallExpression1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "MethodCallExpression1";
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
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl std::ops::Deref for crate::System::Linq::Expressions::MethodCallExpression1 {
    type Target = crate::System::Linq::Expressions::MethodCallExpression;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MethodCallExpression1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl crate::System::Linq::Expressions::MethodCallExpression1 {
    pub fn GetArgument(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        > = __cordl_object.invoke("GetArgument", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, arg0))?;
        Ok(__cordl_object.into())
    }
    pub fn Rewrite(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::MethodCallExpression>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::MethodCallExpression,
        > = __cordl_object.invoke("Rewrite", (instance, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        arg0: quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::Expression>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, arg0))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ArgumentCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ArgumentCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MethodCallExpression1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl AsRef<crate::System::Linq::Expressions::IArgumentProvider>
for crate::System::Linq::Expressions::MethodCallExpression1 {
    fn as_ref(&self) -> &crate::System::Linq::Expressions::IArgumentProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+Expressions+MethodCallExpression1")]
impl AsMut<crate::System::Linq::Expressions::IArgumentProvider>
for crate::System::Linq::Expressions::MethodCallExpression1 {
    fn as_mut(&mut self) -> &mut crate::System::Linq::Expressions::IArgumentProvider {
        unsafe { std::mem::transmute(self) }
    }
}
