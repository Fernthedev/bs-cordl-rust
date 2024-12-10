#[cfg(feature = "Mono+Math+BigInteger")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub length: u32,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "Mono+Math+BigInteger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::BigInteger => "Mono.Math"
    ."BigInteger"
);
#[cfg(feature = "Mono+Math+BigInteger")]
impl std::ops::Deref for crate::Mono::Math::BigInteger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl crate::Mono::Math::BigInteger {
    #[cfg(feature = "Mono+Math+BigInteger+Kernel")]
    pub type Kernel = crate::Mono::Math::BigInteger_Kernel;
    #[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
    pub type ModulusRing = crate::Mono::Math::BigInteger_ModulusRing;
    #[cfg(feature = "Mono+Math+BigInteger+Sign")]
    pub type Sign = crate::Mono::Math::BigInteger_Sign;
    pub fn BitCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("BitCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Incr2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Incr2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsProbablePrime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsProbablePrime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LowestSetBit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LowestSetBit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ModInverse(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("ModInverse", (modulus))?;
        Ok(__cordl_ret.into())
    }
    pub fn ModPow(
        &mut self,
        exp: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("ModPow", (exp, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_BigInteger1(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bi))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_Sign_u32_0(
        sign: crate::Mono::Math::BigInteger_Sign,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sign, len))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_u32_2(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bi, len))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray3(
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_4(
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ui))?;
        Ok(__cordl_object.into())
    }
    pub fn Normalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Normalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Randomize_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Randomize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Randomize_RandomNumberGenerator0(
        &mut self,
        rng: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Randomize", (rng))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBit__cordl_bool1(
        &mut self,
        bitNum: u32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBit", (bitNum, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBit_u32_0(
        &mut self,
        bitNum: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBit", (bitNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn TestBit_i32_1(&mut self, bitNum: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TestBit", (bitNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn TestBit_u32_0(&mut self, bitNum: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TestBit", (bitNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_0(
        &mut self,
        radix: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (radix))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_Il2CppString1(
        &mut self,
        radix: u32,
        characterSet: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (radix, characterSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger1(
        &mut self,
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bi))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_Sign_u32_0(
        &mut self,
        sign: crate::Mono::Math::BigInteger_Sign,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sign, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_u32_2(
        &mut self,
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bi, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray3(
        &mut self,
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_4(
        &mut self,
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ui))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger_Kernel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::BigInteger_Kernel => "Mono.Math"
    ."BigInteger/Kernel"
);
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl std::ops::Deref for crate::Mono::Math::BigInteger_Kernel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger_Kernel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl crate::Mono::Math::BigInteger_Kernel {}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger_Kernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger_ModulusRing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_mod: *mut crate::Mono::Math::BigInteger,
    pub constant: *mut crate::Mono::Math::BigInteger,
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::BigInteger_ModulusRing =>
    "Mono.Math"."BigInteger/ModulusRing"
);
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl std::ops::Deref for crate::Mono::Math::BigInteger_ModulusRing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger_ModulusRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl crate::Mono::Math::BigInteger_ModulusRing {
    pub fn BarrettReduction(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BarrettReduction", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Difference(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        b: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("Difference", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        b: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("Multiply", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (modulus))?;
        Ok(__cordl_object.into())
    }
    pub fn Pow_BigInteger0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        k: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("Pow", (a, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pow_u32_1(
        &mut self,
        b: u32,
        exp: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("Pow", (b, exp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (modulus))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger_ModulusRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigInteger_Sign {
    Negative = -1i32,
    Positive = 1i32,
    Zero = 0i32,
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::BigInteger_Sign => "Mono.Math"
    ."BigInteger/Sign"
);
