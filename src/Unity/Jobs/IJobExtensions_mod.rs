#[cfg(feature = "Unity+Jobs+IJobExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IJobExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::IJobExtensions => "Unity.Jobs"
    ."IJobExtensions"
);
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl std::ops::Deref for crate::Unity::Jobs::IJobExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl std::ops::DerefMut for crate::Unity::Jobs::IJobExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl crate::Unity::Jobs::IJobExtensions {
    #[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
    pub type JobStruct_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Jobs::IJobExtensions_JobStruct_1<
        T,
    >;
}
#[cfg(feature = "Unity+Jobs+IJobExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Jobs::IJobExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IJobExtensions_JobStruct_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::IJobExtensions_JobStruct_1 < T > =>
    "Unity.Jobs"."IJobExtensions/JobStruct`1<T>" < T >
);
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Jobs::IJobExtensions_JobStruct_1<T> {
    #[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
    pub type ExecuteJobFunction = crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<
        T,
    >;
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct JobStruct_1_IJobExtensions_ExecuteJobFunction<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction < T > => "Unity.Jobs"
    ."IJobExtensions/JobStruct`1/ExecuteJobFunction" < T >
);
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    pub fn Invoke(
        &mut self,
        data: quest_hook::libil2cpp::ByRefMut<T>,
        additionalPtr: crate::System::IntPtr,
        bufferRangePatchData: crate::System::IntPtr,
        ranges: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobRanges,
        >,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (data, additionalPtr, bufferRangePatchData, ranges, jobIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Jobs+IJobExtensions+JobStruct_1+ExecuteJobFunction")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::JobStruct_1_IJobExtensions_ExecuteJobFunction<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
