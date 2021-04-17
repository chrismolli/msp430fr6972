#[doc = "Reader of register AESACTL0"]
pub type R = crate::R<u16, super::AESACTL0>;
#[doc = "Writer for register AESACTL0"]
pub type W = crate::W<u16, super::AESACTL0>;
#[doc = "Register AESACTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES Operation Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESOP_A {
    #[doc = "0: AES Operation: Encrypt"]
    AESOP_0 = 0,
    #[doc = "1: AES Operation: Decrypt (same Key)"]
    AESOP_1 = 1,
    #[doc = "2: AES Operation: Generate first round Key"]
    AESOP_2 = 2,
    #[doc = "3: AES Operation: Decrypt (first round Key)"]
    AESOP_3 = 3,
}
impl From<AESOP_A> for u8 {
    #[inline(always)]
    fn from(variant: AESOP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESOP`"]
pub type AESOP_R = crate::R<u8, AESOP_A>;
impl AESOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESOP_A {
        match self.bits {
            0 => AESOP_A::AESOP_0,
            1 => AESOP_A::AESOP_1,
            2 => AESOP_A::AESOP_2,
            3 => AESOP_A::AESOP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESOP_0`"]
    #[inline(always)]
    pub fn is_aesop_0(&self) -> bool {
        *self == AESOP_A::AESOP_0
    }
    #[doc = "Checks if the value of the field is `AESOP_1`"]
    #[inline(always)]
    pub fn is_aesop_1(&self) -> bool {
        *self == AESOP_A::AESOP_1
    }
    #[doc = "Checks if the value of the field is `AESOP_2`"]
    #[inline(always)]
    pub fn is_aesop_2(&self) -> bool {
        *self == AESOP_A::AESOP_2
    }
    #[doc = "Checks if the value of the field is `AESOP_3`"]
    #[inline(always)]
    pub fn is_aesop_3(&self) -> bool {
        *self == AESOP_A::AESOP_3
    }
}
#[doc = "Write proxy for field `AESOP`"]
pub struct AESOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AESOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESOP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "AES Operation: Encrypt"]
    #[inline(always)]
    pub fn aesop_0(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_0)
    }
    #[doc = "AES Operation: Decrypt (same Key)"]
    #[inline(always)]
    pub fn aesop_1(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_1)
    }
    #[doc = "AES Operation: Generate first round Key"]
    #[inline(always)]
    pub fn aesop_2(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_2)
    }
    #[doc = "AES Operation: Decrypt (first round Key)"]
    #[inline(always)]
    pub fn aesop_3(self) -> &'a mut W {
        self.variant(AESOP_A::AESOP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "AES Key length Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESKL_A {
    #[doc = "0: AES Key length: AES128"]
    AESKL_0 = 0,
    #[doc = "1: AES Key length: AES192"]
    AESKL_1 = 1,
    #[doc = "2: AES Key length: AES256"]
    AESKL_2 = 2,
}
impl From<AESKL_A> for u8 {
    #[inline(always)]
    fn from(variant: AESKL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKL`"]
pub type AESKL_R = crate::R<u8, AESKL_A>;
impl AESKL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESKL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKL_A::AESKL_0),
            1 => Val(AESKL_A::AESKL_1),
            2 => Val(AESKL_A::AESKL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AESKL_0`"]
    #[inline(always)]
    pub fn is_aeskl_0(&self) -> bool {
        *self == AESKL_A::AESKL_0
    }
    #[doc = "Checks if the value of the field is `AESKL_1`"]
    #[inline(always)]
    pub fn is_aeskl_1(&self) -> bool {
        *self == AESKL_A::AESKL_1
    }
    #[doc = "Checks if the value of the field is `AESKL_2`"]
    #[inline(always)]
    pub fn is_aeskl_2(&self) -> bool {
        *self == AESKL_A::AESKL_2
    }
}
#[doc = "Write proxy for field `AESKL`"]
pub struct AESKL_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES Key length: AES128"]
    #[inline(always)]
    pub fn aeskl_0(self) -> &'a mut W {
        self.variant(AESKL_A::AESKL_0)
    }
    #[doc = "AES Key length: AES192"]
    #[inline(always)]
    pub fn aeskl_1(self) -> &'a mut W {
        self.variant(AESKL_A::AESKL_1)
    }
    #[doc = "AES Key length: AES256"]
    #[inline(always)]
    pub fn aeskl_2(self) -> &'a mut W {
        self.variant(AESKL_A::AESKL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `AESTRIG`"]
pub type AESTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESTRIG`"]
pub struct AESTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESTRIG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "AES Cipher mode select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESCM_A {
    #[doc = "0: AES Cipher mode select: ECB"]
    AESCM_0 = 0,
    #[doc = "1: AES Cipher mode select: CBC"]
    AESCM_1 = 1,
    #[doc = "2: AES Cipher mode select: OFB"]
    AESCM_2 = 2,
    #[doc = "3: AES Cipher mode select: CFB"]
    AESCM_3 = 3,
}
impl From<AESCM_A> for u8 {
    #[inline(always)]
    fn from(variant: AESCM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESCM`"]
pub type AESCM_R = crate::R<u8, AESCM_A>;
impl AESCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCM_A {
        match self.bits {
            0 => AESCM_A::AESCM_0,
            1 => AESCM_A::AESCM_1,
            2 => AESCM_A::AESCM_2,
            3 => AESCM_A::AESCM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESCM_0`"]
    #[inline(always)]
    pub fn is_aescm_0(&self) -> bool {
        *self == AESCM_A::AESCM_0
    }
    #[doc = "Checks if the value of the field is `AESCM_1`"]
    #[inline(always)]
    pub fn is_aescm_1(&self) -> bool {
        *self == AESCM_A::AESCM_1
    }
    #[doc = "Checks if the value of the field is `AESCM_2`"]
    #[inline(always)]
    pub fn is_aescm_2(&self) -> bool {
        *self == AESCM_A::AESCM_2
    }
    #[doc = "Checks if the value of the field is `AESCM_3`"]
    #[inline(always)]
    pub fn is_aescm_3(&self) -> bool {
        *self == AESCM_A::AESCM_3
    }
}
#[doc = "Write proxy for field `AESCM`"]
pub struct AESCM_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESCM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "AES Cipher mode select: ECB"]
    #[inline(always)]
    pub fn aescm_0(self) -> &'a mut W {
        self.variant(AESCM_A::AESCM_0)
    }
    #[doc = "AES Cipher mode select: CBC"]
    #[inline(always)]
    pub fn aescm_1(self) -> &'a mut W {
        self.variant(AESCM_A::AESCM_1)
    }
    #[doc = "AES Cipher mode select: OFB"]
    #[inline(always)]
    pub fn aescm_2(self) -> &'a mut W {
        self.variant(AESCM_A::AESCM_2)
    }
    #[doc = "AES Cipher mode select: CFB"]
    #[inline(always)]
    pub fn aescm_3(self) -> &'a mut W {
        self.variant(AESCM_A::AESCM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `AESSWRST`"]
pub type AESSWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESSWRST`"]
pub struct AESSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `AESRDYIFG`"]
pub type AESRDYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESRDYIFG`"]
pub struct AESRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AESERRFG`"]
pub type AESERRFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESERRFG`"]
pub struct AESERRFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AESERRFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AESRDYIE`"]
pub type AESRDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESRDYIE`"]
pub struct AESRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRDYIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `AESCMEN`"]
pub type AESCMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESCMEN`"]
pub struct AESCMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AES Operation Bit: 0"]
    #[inline(always)]
    pub fn aesop(&self) -> AESOP_R {
        AESOP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - AES Key length Bit: 0"]
    #[inline(always)]
    pub fn aeskl(&self) -> AESKL_R {
        AESKL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - AES Trigger Select"]
    #[inline(always)]
    pub fn aestrig(&self) -> AESTRIG_R {
        AESTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - AES Cipher mode select Bit: 0"]
    #[inline(always)]
    pub fn aescm(&self) -> AESCM_R {
        AESCM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - AES Software Reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AESSWRST_R {
        AESSWRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AESRDYIFG_R {
        AESRDYIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES Error Flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AESERRFG_R {
        AESERRFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AESRDYIE_R {
        AESRDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AES DMA cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AESCMEN_R {
        AESCMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES Operation Bit: 0"]
    #[inline(always)]
    pub fn aesop(&mut self) -> AESOP_W {
        AESOP_W { w: self }
    }
    #[doc = "Bits 2:3 - AES Key length Bit: 0"]
    #[inline(always)]
    pub fn aeskl(&mut self) -> AESKL_W {
        AESKL_W { w: self }
    }
    #[doc = "Bit 4 - AES Trigger Select"]
    #[inline(always)]
    pub fn aestrig(&mut self) -> AESTRIG_W {
        AESTRIG_W { w: self }
    }
    #[doc = "Bits 5:6 - AES Cipher mode select Bit: 0"]
    #[inline(always)]
    pub fn aescm(&mut self) -> AESCM_W {
        AESCM_W { w: self }
    }
    #[doc = "Bit 7 - AES Software Reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AESSWRST_W {
        AESSWRST_W { w: self }
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AESRDYIFG_W {
        AESRDYIFG_W { w: self }
    }
    #[doc = "Bit 11 - AES Error Flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AESERRFG_W {
        AESERRFG_W { w: self }
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AESRDYIE_W {
        AESRDYIE_W { w: self }
    }
    #[doc = "Bit 15 - AES DMA cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AESCMEN_W {
        AESCMEN_W { w: self }
    }
}
