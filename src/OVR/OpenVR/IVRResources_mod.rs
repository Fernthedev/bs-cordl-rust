#[cfg(feature = "OVR+OpenVR+IVRResources")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRResources {
    pub LoadSharedResource: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRResources__LoadSharedResource,
    >,
    pub GetResourceFullPath: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRResources__GetResourceFullPath,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRResources {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRResources";
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
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRResources {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRResources {
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
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRResources {
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
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRResources {
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
#[cfg(feature = "OVR+OpenVR+IVRResources")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRResources {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources")]
impl crate::OVR::OpenVR::IVRResources {
    #[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
    pub type _GetResourceFullPath = crate::OVR::OpenVR::IVRResources__GetResourceFullPath;
    #[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
    pub type _LoadSharedResource = crate::OVR::OpenVR::IVRResources__LoadSharedResource;
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRResources__GetResourceFullPath {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRResources/_GetResourceFullPath";
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
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    pub fn BeginInvoke(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResourceTypeDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchPathBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    pchResourceName,
                    pchResourceTypeDirectory,
                    pchPathBuffer,
                    unBufferLen,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResourceTypeDirectory: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pchPathBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (pchResourceName, pchResourceTypeDirectory, pchPathBuffer, unBufferLen),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_GetResourceFullPath")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRResources__GetResourceFullPath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRResources__LoadSharedResource {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRResources/_LoadSharedResource";
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
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    pub fn BeginInvoke(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unBufferLen: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchResourceName, pchBuffer, unBufferLen, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (pchResourceName, pchBuffer, unBufferLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRResources+_LoadSharedResource")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRResources__LoadSharedResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
