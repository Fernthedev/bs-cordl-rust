#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
#[repr(C)]
#[derive(Debug)]
pub struct TrueOverdraw {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _overdrawType_k__BackingField: crate::TrueOverdraw::TrueOverdraw_OverdrawType,
    pub _renderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        >,
    >,
    pub _cachedSharedMaterials: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    >,
    pub _cachedMaterialInstances: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
unsafe impl quest_hook::libil2cpp::Type for crate::TrueOverdraw::TrueOverdraw {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TrueOverdraw";
    const CLASS_NAME: &'static str = "TrueOverdraw";
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
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl std::ops::Deref for crate::TrueOverdraw::TrueOverdraw {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl std::ops::DerefMut for crate::TrueOverdraw::TrueOverdraw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl crate::TrueOverdraw::TrueOverdraw {
    pub const kOverdrawViewKeyword: &'static str = "OVERDRAW_VIEW";
    #[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
    pub type OverdrawType = crate::TrueOverdraw::TrueOverdraw_OverdrawType;
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        renderers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderers))?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveRendererColor(
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveRendererColor", (renderer, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloats(
        trueOverdraw: f32,
        opaque: f32,
        transparent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetGlobalFloats", (trueOverdraw, opaque, transparent))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaterialValues(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMaterialValues", (material, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOverdrawValues(
        opaque: f32,
        transparent: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOverdrawValues", (opaque, transparent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowEverything(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowEverything", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowOnlyOpaque(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowOnlyOpaque", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowOnlyTransparent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowOnlyTransparent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        renderers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderers))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overdrawType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TrueOverdraw::TrueOverdraw_OverdrawType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TrueOverdraw::TrueOverdraw_OverdrawType = __cordl_object
            .invoke("get_overdrawType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderersLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_renderersLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overdrawType(
        &mut self,
        value: crate::TrueOverdraw::TrueOverdraw_OverdrawType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overdrawType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl quest_hook::libil2cpp::ObjectType for crate::TrueOverdraw::TrueOverdraw {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl AsRef<crate::System::IDisposable> for crate::TrueOverdraw::TrueOverdraw {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw")]
impl AsMut<crate::System::IDisposable> for crate::TrueOverdraw::TrueOverdraw {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrueOverdraw_OverdrawType {
    #[default]
    Everything = 3i32,
    None = 0i32,
    Opaque = 2i32,
    Transparent = 1i32,
}
#[cfg(feature = "TrueOverdraw+TrueOverdraw+OverdrawType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::TrueOverdraw::TrueOverdraw_OverdrawType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TrueOverdraw";
    const CLASS_NAME: &'static str = "OverdrawType";
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
for crate::TrueOverdraw::TrueOverdraw_OverdrawType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TrueOverdraw::TrueOverdraw_OverdrawType {
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
for crate::TrueOverdraw::TrueOverdraw_OverdrawType {
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
for crate::TrueOverdraw::TrueOverdraw_OverdrawType {
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
