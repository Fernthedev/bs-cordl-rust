#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
#[repr(C)]
#[derive(Debug)]
pub struct Spline {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::Spline {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "Spline";
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
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Spline {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Spline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl crate::UnityEngine::ProBuilder::Spline {
    pub fn Extrude__cordl_bool_ByRefMut_IList_1_2(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        radius: f32,
        radiusRows: i32,
        closeLoop: bool,
        smooth: bool,
        target: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        pointRotations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Quaternion>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Vector3,
                        >,
                    >,
                    f32,
                    i32,
                    bool,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Quaternion,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("Extrude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Extrude", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        points,
                        radius,
                        radiusRows,
                        closeLoop,
                        smooth,
                        target,
                        pointRotations,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_i32__cordl_bool0(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::BezierPoint,
            >,
        >,
        radius: f32,
        columns: i32,
        rows: i32,
        closeLoop: bool,
        smooth: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::ProBuilder::BezierPoint,
                        >,
                    >,
                    f32,
                    i32,
                    i32,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                6usize,
            >("Extrude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Extrude", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = unsafe {
            method
                .invoke_unchecked((), (points, radius, columns, rows, closeLoop, smooth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_i32__cordl_bool_ByRefMut1(
        bezierPoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::BezierPoint,
            >,
        >,
        radius: f32,
        columns: i32,
        rows: i32,
        closeLoop: bool,
        smooth: bool,
        target: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::ProBuilder::BezierPoint,
                        >,
                    >,
                    f32,
                    i32,
                    i32,
                    bool,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("Extrude")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Extrude", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (bezierPoints, radius, columns, rows, closeLoop, smooth, target),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetControlPoints(
        bezierPoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::BezierPoint,
            >,
        >,
        subdivisionsPerSegment: i32,
        closeLoop: bool,
        rotations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Quaternion>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::ProBuilder::BezierPoint,
                        >,
                    >,
                    i32,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Quaternion,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::Vector3,
                    >,
                >,
                4usize,
            >("GetControlPoints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetControlPoints", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (bezierPoints, subdivisionsPerSegment, closeLoop, rotations),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRingRotation(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        i: i32,
        closeLoop: bool,
        secant: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Vector3,
                        >,
                    >,
                    i32,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                crate::UnityEngine::Quaternion,
                4usize,
            >("GetRingRotation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRingRotation", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked((), (points, i, closeLoop, secant))
        };
        Ok(__cordl_ret.into())
    }
    pub fn VertexRing(
        orientation: crate::UnityEngine::Quaternion,
        offset: crate::UnityEngine::Vector3,
        radius: f32,
        segments: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Quaternion, crate::UnityEngine::Vector3, f32, i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                >,
                4usize,
            >("VertexRing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "VertexRing", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe {
            method.invoke_unchecked((), (orientation, offset, radius, segments))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Spline")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Spline {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
