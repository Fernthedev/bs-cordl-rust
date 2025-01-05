#[cfg(feature = "TMPro+MaterialReferenceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialReferenceManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_FontMaterialReferenceLookup: quest_hook::libil2cpp::Gc<
        i32,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    >,
    pub m_FontAssetReferenceLookup: quest_hook::libil2cpp::Gc<
        i32,
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    >,
    pub m_SpriteAssetReferenceLookup: quest_hook::libil2cpp::Gc<
        i32,
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    >,
    pub m_ColorGradientReferenceLookup: quest_hook::libil2cpp::Gc<
        i32,
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
    >,
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::MaterialReferenceManager => "TMPro"
    ."MaterialReferenceManager"
);
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl std::ops::Deref for crate::TMPro::MaterialReferenceManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl std::ops::DerefMut for crate::TMPro::MaterialReferenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl crate::TMPro::MaterialReferenceManager {
    pub fn AddColorGradientPreset(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddColorGradientPreset", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddColorGradientPreset_Internal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddColorGradientPreset_Internal", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFontAsset(
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddFontAsset", (fontAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFontAssetInternal(
        &mut self,
        fontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFontAssetInternal", (fontAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFontMaterial(
        hashCode: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddFontMaterial", (hashCode, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFontMaterialInternal(
        &mut self,
        hashCode: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFontMaterialInternal", (hashCode, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAssetInternal_Gc0(
        &mut self,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpriteAssetInternal", (spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAssetInternal_i32_Gc1(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSpriteAssetInternal", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAsset_Gc0(
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSpriteAsset", (spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpriteAsset_i32_Gc1(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSpriteAsset", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc0(
        &mut self,
        font: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (font))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc1(
        &mut self,
        sprite: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetColorGradientPreset(
        hashCode: i32,
        gradientPreset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetColorGradientPreset", (hashCode, gradientPreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetColorGradientPresetInternal(
        &mut self,
        hashCode: i32,
        gradientPreset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetColorGradientPresetInternal", (hashCode, gradientPreset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetFontAsset(
        hashCode: i32,
        fontAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetFontAsset", (hashCode, fontAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetFontAssetInternal(
        &mut self,
        hashCode: i32,
        fontAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetFontAssetInternal", (hashCode, fontAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMaterial(
        hashCode: i32,
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetMaterial", (hashCode, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMaterialInternal(
        &mut self,
        hashCode: i32,
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetMaterialInternal", (hashCode, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSpriteAsset(
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSpriteAsset", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSpriteAssetInternal(
        &mut self,
        hashCode: i32,
        spriteAsset: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSpriteAssetInternal", (hashCode, spriteAsset))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::MaterialReferenceManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::TMPro::MaterialReferenceManager,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+MaterialReferenceManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::MaterialReferenceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
