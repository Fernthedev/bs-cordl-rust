#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_patchSize: crate::UnityEngine::Vector3,
    pub m_errorCode: crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
    pub m_terrainTiles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::TerrainUtils::TerrainTileCoord,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TerrainUtils::TerrainMap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TerrainUtils";
    const CLASS_NAME: &'static str = "TerrainMap";
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
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainUtils::TerrainMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl crate::UnityEngine::TerrainUtils::TerrainMap {
    pub fn AddTerrainInternal(
        &mut self,
        x: i32,
        z: i32,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddTerrainInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTerrainInternal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x, z, terrain))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromPlacement_Terrain_Predicate_1__cordl_bool0(
        originTerrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Predicate_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        >,
        fullValidation: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Predicate_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
                3usize,
            >("CreateFromPlacement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateFromPlacement", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TerrainUtils::TerrainMap,
        > = unsafe {
            method.invoke_unchecked((), (originTerrain, filter, fullValidation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromPlacement_Vector2_Vector2_Predicate_1__cordl_bool1(
        gridOrigin: crate::UnityEngine::Vector2,
        gridSize: crate::UnityEngine::Vector2,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Predicate_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        >,
        fullValidation: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Predicate_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
                4usize,
            >("CreateFromPlacement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateFromPlacement", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TerrainUtils::TerrainMap,
        > = unsafe {
            method.invoke_unchecked((), (gridOrigin, gridSize, filter, fullValidation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
                2usize,
            >("GetTerrain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTerrain", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain> = unsafe {
            method.invoke_unchecked(self, (tileX, tileZ))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryToAddTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>),
                bool,
                3usize,
            >("TryToAddTerrain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryToAddTerrain", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (tileX, tileZ, terrain))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
                0usize,
            >("Validate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Validate", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::TerrainUtils::TerrainMapStatusCode = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ValidateTerrain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ValidateTerrain", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tileX, tileZ))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_terrainTiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::TerrainUtils::TerrainTileCoord,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        crate::UnityEngine::TerrainUtils::TerrainTileCoord,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
                    >,
                >,
                0usize,
            >("get_terrainTiles")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_terrainTiles", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::TerrainUtils::TerrainTileCoord,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TerrainUtils::TerrainMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
