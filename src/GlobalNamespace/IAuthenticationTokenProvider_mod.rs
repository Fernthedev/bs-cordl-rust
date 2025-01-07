#[cfg(feature = "IAuthenticationTokenProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IAuthenticationTokenProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAuthenticationTokenProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IAuthenticationTokenProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IAuthenticationTokenProvider";
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
#[cfg(feature = "IAuthenticationTokenProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IAuthenticationTokenProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IAuthenticationTokenProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl crate::GlobalNamespace::IAuthenticationTokenProvider {
    pub fn GetAuthenticationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        > = __cordl_object.invoke("GetAuthenticationToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTokenPlatform(
        &mut self,
        tokenPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = __cordl_object
            .invoke("GetTokenPlatform", (tokenPlatformEnvironment))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_hashedUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_hashedUserId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_platform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = __cordl_object
            .invoke("get_platform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IAuthenticationTokenProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl AsRef<crate::BGNet::Core::IPlatformAccessTokenFetcher>
for crate::GlobalNamespace::IAuthenticationTokenProvider {
    fn as_ref(&self) -> &crate::BGNet::Core::IPlatformAccessTokenFetcher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl AsMut<crate::BGNet::Core::IPlatformAccessTokenFetcher>
for crate::GlobalNamespace::IAuthenticationTokenProvider {
    fn as_mut(&mut self) -> &mut crate::BGNet::Core::IPlatformAccessTokenFetcher {
        unsafe { std::mem::transmute(self) }
    }
}
