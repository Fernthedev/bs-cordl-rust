#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
#[repr(C)]
#[derive(Debug)]
pub struct ElementSelection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "ElementSelection";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    pub const k_MaxHoleIterations: i32 = 2048i32;
    pub fn EdgeRingNext(
        edge: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
                1usize,
            >("EdgeRingNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EdgeRingNext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = unsafe { method.invoke_unchecked((), (edge)) };
        Ok(__cordl_ret.into())
    }
    pub fn FindHoles_List_1_HashSet_1_1(
        wings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
        common: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::WingedEdge,
                        >,
                    >,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::HashSet_1<i32>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::WingedEdge,
                                >,
                            >,
                        >,
                    >,
                >,
                2usize,
            >("FindHoles")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindHoles", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::WingedEdge,
                        >,
                    >,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (wings, common)) };
        Ok(__cordl_ret.into())
    }
    pub fn FindHoles_ProBuilderMesh_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<i32>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                    >,
                >,
                2usize,
            >("FindHoles")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindHoles", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, indexes)) };
        Ok(__cordl_ret.into())
    }
    pub fn FindNextEdgeInHole(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        common: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
                2usize,
            >("FindNextEdgeInHole")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindNextEdgeInHole", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = unsafe { method.invoke_unchecked((), (wing, common)) };
        Ok(__cordl_ret.into())
    }
    pub fn FloodSelection(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        maxAngleDiff: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                3usize,
            >("FloodSelection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FloodSelection", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces, maxAngleDiff)) };
        Ok(__cordl_ret.into())
    }
    pub fn Flood_ProBuilderMesh_WingedEdge_Vector3_f32_HashSet_1_1(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        wingNrm: crate::UnityEngine::Vector3,
        maxAngle: f32,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    crate::UnityEngine::Vector3,
                    f32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::HashSet_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("Flood")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Flood", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (pb, wing, wingNrm, maxAngle, selection))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Flood_WingedEdge_HashSet_1_0(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::HashSet_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Flood")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Flood", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (wing, selection))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectedEdges(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
                2usize,
            >("GetConnectedEdges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetConnectedEdges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        > = unsafe { method.invoke_unchecked((), (mesh, indexes)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeLoop(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        _cordl_loop: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("GetEdgeLoop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeLoop", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (mesh, edges, _cordl_loop))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeLoopInternal(
        start: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        startIndex: i32,
        used: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::UnityEngine::ProBuilder::EdgeLookup,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::HashSet_1<
                            crate::UnityEngine::ProBuilder::EdgeLookup,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("GetEdgeLoopInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeLoopInternal", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (start, startIndex, used))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeLoopInternalIterative(
        start: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        edge: crate::UnityEngine::ProBuilder::Edge,
        used: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                crate::UnityEngine::ProBuilder::EdgeLookup,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    crate::UnityEngine::ProBuilder::Edge,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::HashSet_1<
                            crate::UnityEngine::ProBuilder::EdgeLookup,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetEdgeLoopInternalIterative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeLoopInternalIterative", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (start, edge, used))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeLoopIterative(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        _cordl_loop: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                    >,
                ),
                bool,
                3usize,
            >("GetEdgeLoopIterative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeLoopIterative", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (mesh, edges, _cordl_loop))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeRing(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
                2usize,
            >("GetEdgeRing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeRing", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = unsafe { method.invoke_unchecked((), (pb, edges)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeRingIterative(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
                2usize,
            >("GetEdgeRingIterative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEdgeRingIterative", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = unsafe { method.invoke_unchecked((), (pb, edges)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceLoop_List_1_Face1(
        wings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        ring: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::WingedEdge,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                3usize,
            >("GetFaceLoop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFaceLoop", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (wings, face, ring)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceLoop_ProBuilderMesh_Il2CppArray0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        ring: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                3usize,
            >("GetFaceLoop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFaceLoop", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces, ring)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceRingAndLoop(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                2usize,
            >("GetFaceRingAndLoop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFaceRingAndLoop", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNeighborFaces_Edge1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    crate::UnityEngine::ProBuilder::Edge,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    crate::UnityEngine::ProBuilder::Edge,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::SimpleTuple_2<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                >,
                2usize,
            >("GetNeighborFaces")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNeighborFaces", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    crate::UnityEngine::ProBuilder::Edge,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, edge)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNeighborFaces_Edge_List_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
        neighborFaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    crate::UnityEngine::ProBuilder::Edge,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetNeighborFaces")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNeighborFaces", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mesh, edge, neighborFaces))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNeighborFaces_Il2CppArray2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                2usize,
            >("GetNeighborFaces")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNeighborFaces", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, indexes)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPerimeterEdges_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
                2usize,
            >("GetPerimeterEdges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPerimeterEdges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPerimeterEdges_IList_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                2usize,
            >("GetPerimeterEdges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPerimeterEdges", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked((), (mesh, edges)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPerimeterFaces(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                2usize,
            >("GetPerimeterFaces")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPerimeterFaces", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPerimeterVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        universal_edges_all: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                3usize,
            >("GetPerimeterVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPerimeterVertices", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked((), (mesh, indexes, universal_edges_all)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpokes(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        sharedIndex: i32,
        allowHoles: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::WingedEdge,
                        >,
                    >,
                >,
                3usize,
            >("GetSpokes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSpokes", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
            >,
        > = unsafe { method.invoke_unchecked((), (wing, sharedIndex, allowHoles)) };
        Ok(__cordl_ret.into())
    }
    pub fn GrowSelection(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        maxAngleDiff: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Face,
                            >,
                        >,
                    >,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
                3usize,
            >("GrowSelection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GrowSelection", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe { method.invoke_unchecked((), (mesh, faces, maxAngleDiff)) };
        Ok(__cordl_ret.into())
    }
    pub fn NextSpoke(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        pivot: i32,
        opp: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
                3usize,
            >("NextSpoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NextSpoke", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::WingedEdge,
        > = unsafe { method.invoke_unchecked((), (wing, pivot, opp)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ElementSelection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ElementSelection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
