#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OpacityIdAccelerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Jobs: crate::Unity::Collections::NativeArray_1<crate::Unity::Jobs::JobHandle>,
    pub m_NextJobIndex: i32,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "OpacityIdAccelerator";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    #[cfg(
        feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob"
    )]
    pub type OpacityIdUpdateJob = crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob;
    pub fn CompleteJobs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteJobs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJob(
        &mut self,
        oldVerts: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
        newVerts: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
        opacityData: crate::UnityEngine::Color32,
        vertexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateJob", (oldVerts, newVerts, opacityData, vertexCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpacityIdAccelerator_OpacityIdUpdateJob {
    pub oldVerts: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub newVerts: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub opacityData: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "OpacityIdUpdateJob";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
impl crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+OpacityIdAccelerator+OpacityIdUpdateJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::UIElements::UIR::OpacityIdAccelerator_OpacityIdUpdateJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
