#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarPartsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _headTopCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _eyesCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
            >,
        >,
    >,
    pub _mouthCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
            >,
        >,
    >,
    pub _glassesCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _facialHairCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _handsCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _clothesCollection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _skinColors_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
        >,
    >,
    pub _indexById: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            i32,
        >,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "AvatarPartsModel";
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    pub fn GetColorIndexById(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetColorIndexById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetColorIndexById", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRandomColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
                0usize,
            >("GetRandomColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRandomColor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSkinColorById(
        &mut self,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
                1usize,
            >("GetSkinColorById")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSkinColorById", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
        > = unsafe { method.invoke_unchecked(self, (id)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        avatarPartData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
        >,
        skinColorSet: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (avatarPartData, skinColorSet))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        avatarPartData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
        >,
        skinColorSet: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (avatarPartData, skinColorSet))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_clothesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_clothesCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_clothesCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_eyesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_eyesCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_eyesCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_facialHairCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_facialHairCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_facialHairCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_glassesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_glassesCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_glassesCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_handsCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_handsCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_handsCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_headTopCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_headTopCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_headTopCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_mouthCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                        >,
                    >,
                >,
                0usize,
            >("get_mouthCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_mouthCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_skinColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
                        >,
                    >,
                >,
                0usize,
            >("get_skinColors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_skinColors", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::SkinColorSO>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
