#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMeshConstructorCrossedStrips {
    __cordl_parent: crate::GlobalNamespace::SliderMeshConstructor,
    pub _triangleMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SliderMeshConstructorCrossedStrips {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SliderMeshConstructorCrossedStrips";
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
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl std::ops::Deref for crate::GlobalNamespace::SliderMeshConstructorCrossedStrips {
    type Target = crate::GlobalNamespace::SliderMeshConstructor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderMeshConstructorCrossedStrips {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl crate::GlobalNamespace::SliderMeshConstructorCrossedStrips {
    pub fn CreateSliderMeshInternal(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
        zDistanceBetweenNotes: f32,
        bounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CreateSliderMeshInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateSliderMeshInternal", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (path, zDistanceBetweenNotes, bounds))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesCount(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>),
                i32,
                1usize,
            >("GetTrianglesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTrianglesCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (path)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexCount(
        &mut self,
        path: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VertexPath>),
                i32,
                1usize,
            >("GetVertexCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetVertexCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (path)) };
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
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderMeshConstructorCrossedStrips {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
