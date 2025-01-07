#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshBuilderNative {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshBuilderNative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "MeshBuilderNative";
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
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MeshBuilderNative {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MeshBuilderNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl crate::UnityEngine::UIElements::MeshBuilderNative {
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
    pub type NativeBorderParams = crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams;
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
    pub type NativeColorPage = crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage;
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
    pub type NativeRectParams = crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams;
    pub fn MakeBorder(
        borderParams: crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams,
        posZ: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshWriteDataInterface,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshWriteDataInterface = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeBorder", (borderParams, posZ))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeBorder_Injected(
        borderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams,
        >,
        posZ: f32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshWriteDataInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeBorder_Injected", (borderParams, posZ, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeSolidRect(
        rectParams: crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        posZ: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshWriteDataInterface,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshWriteDataInterface = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeSolidRect", (rectParams, posZ))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeSolidRect_Injected(
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        posZ: f32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshWriteDataInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeSolidRect_Injected", (rectParams, posZ, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeTexturedRect(
        rectParams: crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        posZ: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshWriteDataInterface,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshWriteDataInterface = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeTexturedRect", (rectParams, posZ))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeTexturedRect_Injected(
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        posZ: f32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshWriteDataInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeTexturedRect_Injected", (rectParams, posZ, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorGraphics9SliceBackground(
        svgVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::UIElements::Vertex>,
        >,
        svgIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        svgWidth: f32,
        svgHeight: f32,
        targetRect: crate::UnityEngine::Rect,
        sliceLTRB: crate::UnityEngine::Vector4,
        tint: crate::UnityEngine::Color,
        colorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
        settingIndexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshWriteDataInterface,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshWriteDataInterface = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeVectorGraphics9SliceBackground",
                (
                    svgVertices,
                    svgIndices,
                    svgWidth,
                    svgHeight,
                    targetRect,
                    sliceLTRB,
                    tint,
                    colorPage,
                    settingIndexOffset,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorGraphics9SliceBackground_Injected(
        svgVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::UIElements::Vertex>,
        >,
        svgIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        svgWidth: f32,
        svgHeight: f32,
        targetRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        sliceLTRB: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        tint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        colorPage: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
        >,
        settingIndexOffset: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshWriteDataInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeVectorGraphics9SliceBackground_Injected",
                (
                    svgVertices,
                    svgIndices,
                    svgWidth,
                    svgHeight,
                    targetRect,
                    sliceLTRB,
                    tint,
                    colorPage,
                    settingIndexOffset,
                    ret,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorGraphicsStretchBackground(
        svgVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::UIElements::Vertex>,
        >,
        svgIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        svgWidth: f32,
        svgHeight: f32,
        targetRect: crate::UnityEngine::Rect,
        sourceUV: crate::UnityEngine::Rect,
        scaleMode: crate::UnityEngine::ScaleMode,
        tint: crate::UnityEngine::Color,
        colorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
        settingIndexOffset: i32,
        finalVertexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        finalIndexCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshWriteDataInterface,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshWriteDataInterface = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeVectorGraphicsStretchBackground",
                (
                    svgVertices,
                    svgIndices,
                    svgWidth,
                    svgHeight,
                    targetRect,
                    sourceUV,
                    scaleMode,
                    tint,
                    colorPage,
                    settingIndexOffset,
                    finalVertexCount,
                    finalIndexCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorGraphicsStretchBackground_Injected(
        svgVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::UIElements::Vertex>,
        >,
        svgIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        svgWidth: f32,
        svgHeight: f32,
        targetRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        sourceUV: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        scaleMode: crate::UnityEngine::ScaleMode,
        tint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        colorPage: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
        >,
        settingIndexOffset: i32,
        finalVertexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        finalIndexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshWriteDataInterface,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeVectorGraphicsStretchBackground_Injected",
                (
                    svgVertices,
                    svgIndices,
                    svgWidth,
                    svgHeight,
                    targetRect,
                    sourceUV,
                    scaleMode,
                    tint,
                    colorPage,
                    settingIndexOffset,
                    finalVertexCount,
                    finalIndexCount,
                    ret,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MeshBuilderNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshBuilderNative_NativeBorderParams {
    pub rect: crate::UnityEngine::Rect,
    pub leftColor: crate::UnityEngine::Color,
    pub topColor: crate::UnityEngine::Color,
    pub rightColor: crate::UnityEngine::Color,
    pub bottomColor: crate::UnityEngine::Color,
    pub leftWidth: f32,
    pub topWidth: f32,
    pub rightWidth: f32,
    pub bottomWidth: f32,
    pub topLeftRadius: crate::UnityEngine::Vector2,
    pub topRightRadius: crate::UnityEngine::Vector2,
    pub bottomRightRadius: crate::UnityEngine::Vector2,
    pub bottomLeftRadius: crate::UnityEngine::Vector2,
    pub leftColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub topColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub rightColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub bottomColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "NativeBorderParams";
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
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
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshBuilderNative_NativeColorPage {
    pub isValid: i32,
    pub pageAndID: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "NativeColorPage";
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
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
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshBuilderNative_NativeRectParams {
    pub rect: crate::UnityEngine::Rect,
    pub subRect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
    pub uvRegion: crate::UnityEngine::Rect,
    pub color: crate::UnityEngine::Color,
    pub scaleMode: crate::UnityEngine::ScaleMode,
    pub topLeftRadius: crate::UnityEngine::Vector2,
    pub topRightRadius: crate::UnityEngine::Vector2,
    pub bottomRightRadius: crate::UnityEngine::Vector2,
    pub bottomLeftRadius: crate::UnityEngine::Vector2,
    pub contentSize: crate::UnityEngine::Vector2,
    pub textureSize: crate::UnityEngine::Vector2,
    pub texturePixelsPerPoint: f32,
    pub leftSlice: i32,
    pub topSlice: i32,
    pub rightSlice: i32,
    pub bottomSlice: i32,
    pub sliceScale: f32,
    pub rectInset: crate::UnityEngine::Vector4,
    pub colorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "NativeRectParams";
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
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
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
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
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {}
