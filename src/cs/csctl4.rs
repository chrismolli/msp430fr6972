#[doc = "Reader of register CSCTL4"]
pub type R = crate::R<u16, super::CSCTL4>;
#[doc = "Writer for register CSCTL4"]
pub type W = crate::W<u16, super::CSCTL4>;
#[doc = "Register CSCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFXTOFF`"]
pub type LFXTOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXTOFF`"]
pub struct LFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SMCLKOFF`"]
pub type SMCLKOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMCLKOFF`"]
pub struct SMCLKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `VLOOFF`"]
pub type VLOOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLOOFF`"]
pub struct VLOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VLOOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LFXTBYPASS`"]
pub type LFXTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXTBYPASS`"]
pub struct LFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTBYPASS_W<'a> {
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
#[doc = "LFXT Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFXTDRIVE_A {
    #[doc = "0: LFXT Drive Level mode: 0"]
    LFXTDRIVE_0 = 0,
    #[doc = "1: LFXT Drive Level mode: 1"]
    LFXTDRIVE_1 = 1,
    #[doc = "2: LFXT Drive Level mode: 2"]
    LFXTDRIVE_2 = 2,
    #[doc = "3: LFXT Drive Level mode: 3"]
    LFXTDRIVE_3 = 3,
}
impl From<LFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFXTDRIVE`"]
pub type LFXTDRIVE_R = crate::R<u8, LFXTDRIVE_A>;
impl LFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTDRIVE_A {
        match self.bits {
            0 => LFXTDRIVE_A::LFXTDRIVE_0,
            1 => LFXTDRIVE_A::LFXTDRIVE_1,
            2 => LFXTDRIVE_A::LFXTDRIVE_2,
            3 => LFXTDRIVE_A::LFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_3
    }
}
#[doc = "Write proxy for field `LFXTDRIVE`"]
pub struct LFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTDRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LFXT Drive Level mode: 0"]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_0)
    }
    #[doc = "LFXT Drive Level mode: 1"]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_1)
    }
    #[doc = "LFXT Drive Level mode: 2"]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_2)
    }
    #[doc = "LFXT Drive Level mode: 3"]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `HFXTOFF`"]
pub type HFXTOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXTOFF`"]
pub struct HFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFF_W<'a> {
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
#[doc = "HFXT frequency selection Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFFREQ_A {
    #[doc = "0: HFXT frequency selection: 0"]
    HFFREQ_0 = 0,
    #[doc = "1: HFXT frequency selection: 1"]
    HFFREQ_1 = 1,
    #[doc = "2: HFXT frequency selection: 2"]
    HFFREQ_2 = 2,
    #[doc = "3: HFXT frequency selection: 3"]
    HFFREQ_3 = 3,
}
impl From<HFFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: HFFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFFREQ`"]
pub type HFFREQ_R = crate::R<u8, HFFREQ_A>;
impl HFFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFFREQ_A {
        match self.bits {
            0 => HFFREQ_A::HFFREQ_0,
            1 => HFFREQ_A::HFFREQ_1,
            2 => HFFREQ_A::HFFREQ_2,
            3 => HFFREQ_A::HFFREQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFFREQ_0`"]
    #[inline(always)]
    pub fn is_hffreq_0(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_0
    }
    #[doc = "Checks if the value of the field is `HFFREQ_1`"]
    #[inline(always)]
    pub fn is_hffreq_1(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_1
    }
    #[doc = "Checks if the value of the field is `HFFREQ_2`"]
    #[inline(always)]
    pub fn is_hffreq_2(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_2
    }
    #[doc = "Checks if the value of the field is `HFFREQ_3`"]
    #[inline(always)]
    pub fn is_hffreq_3(&self) -> bool {
        *self == HFFREQ_A::HFFREQ_3
    }
}
#[doc = "Write proxy for field `HFFREQ`"]
pub struct HFFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFFREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFXT frequency selection: 0"]
    #[inline(always)]
    pub fn hffreq_0(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_0)
    }
    #[doc = "HFXT frequency selection: 1"]
    #[inline(always)]
    pub fn hffreq_1(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_1)
    }
    #[doc = "HFXT frequency selection: 2"]
    #[inline(always)]
    pub fn hffreq_2(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_2)
    }
    #[doc = "HFXT frequency selection: 3"]
    #[inline(always)]
    pub fn hffreq_3(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `HFXTBYPASS`"]
pub type HFXTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXTBYPASS`"]
pub struct HFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTBYPASS_W<'a> {
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
#[doc = "HFXT Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXTDRIVE_A {
    #[doc = "0: HFXT Drive Level mode: 0"]
    HFXTDRIVE_0 = 0,
    #[doc = "1: HFXT Drive Level mode: 1"]
    HFXTDRIVE_1 = 1,
    #[doc = "2: HFXT Drive Level mode: 2"]
    HFXTDRIVE_2 = 2,
    #[doc = "3: HFXT Drive Level mode: 3"]
    HFXTDRIVE_3 = 3,
}
impl From<HFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFXTDRIVE`"]
pub type HFXTDRIVE_R = crate::R<u8, HFXTDRIVE_A>;
impl HFXTDRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTDRIVE_A {
        match self.bits {
            0 => HFXTDRIVE_A::HFXTDRIVE_0,
            1 => HFXTDRIVE_A::HFXTDRIVE_1,
            2 => HFXTDRIVE_A::HFXTDRIVE_2,
            3 => HFXTDRIVE_A::HFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_hfxtdrive_2(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_hfxtdrive_3(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_3
    }
}
#[doc = "Write proxy for field `HFXTDRIVE`"]
pub struct HFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTDRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFXT Drive Level mode: 0"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_0)
    }
    #[doc = "HFXT Drive Level mode: 1"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_1)
    }
    #[doc = "HFXT Drive Level mode: 2"]
    #[inline(always)]
    pub fn hfxtdrive_2(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_2)
    }
    #[doc = "HFXT Drive Level mode: 3"]
    #[inline(always)]
    pub fn hfxtdrive_3(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Frequency Oscillator (LFXT) disable"]
    #[inline(always)]
    pub fn lfxtoff(&self) -> LFXTOFF_R {
        LFXTOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VLO Off"]
    #[inline(always)]
    pub fn vlooff(&self) -> VLOOFF_R {
        VLOOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LFXTBYPASS_R {
        LFXTBYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - LFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LFXTDRIVE_R {
        LFXTDRIVE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - High Frequency Oscillator disable"]
    #[inline(always)]
    pub fn hfxtoff(&self) -> HFXTOFF_R {
        HFXTOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - HFXT frequency selection Bit 1"]
    #[inline(always)]
    pub fn hffreq(&self) -> HFFREQ_R {
        HFFREQ_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - HFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HFXTBYPASS_R {
        HFXTBYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - HFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HFXTDRIVE_R {
        HFXTDRIVE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Frequency Oscillator (LFXT) disable"]
    #[inline(always)]
    pub fn lfxtoff(&mut self) -> LFXTOFF_W {
        LFXTOFF_W { w: self }
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W { w: self }
    }
    #[doc = "Bit 3 - VLO Off"]
    #[inline(always)]
    pub fn vlooff(&mut self) -> VLOOFF_W {
        VLOOFF_W { w: self }
    }
    #[doc = "Bit 4 - LFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LFXTBYPASS_W {
        LFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 6:7 - LFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LFXTDRIVE_W {
        LFXTDRIVE_W { w: self }
    }
    #[doc = "Bit 8 - High Frequency Oscillator disable"]
    #[inline(always)]
    pub fn hfxtoff(&mut self) -> HFXTOFF_W {
        HFXTOFF_W { w: self }
    }
    #[doc = "Bits 10:11 - HFXT frequency selection Bit 1"]
    #[inline(always)]
    pub fn hffreq(&mut self) -> HFFREQ_W {
        HFFREQ_W { w: self }
    }
    #[doc = "Bit 12 - HFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HFXTBYPASS_W {
        HFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 14:15 - HFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HFXTDRIVE_W {
        HFXTDRIVE_W { w: self }
    }
}
