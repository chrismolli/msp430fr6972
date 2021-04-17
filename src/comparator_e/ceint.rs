#[doc = "Reader of register CEINT"]
pub type R = crate::R<u16, super::CEINT>;
#[doc = "Writer for register CEINT"]
pub type W = crate::W<u16, super::CEINT>;
#[doc = "Register CEINT `reset()`'s with value 0"]
impl crate::ResetValue for super::CEINT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEIFG`"]
pub type CEIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIFG`"]
pub struct CEIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIFG_W<'a> {
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
#[doc = "Reader of field `CEIIFG`"]
pub type CEIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIIFG`"]
pub struct CEIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIIFG_W<'a> {
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
#[doc = "Reader of field `CERDYIFG`"]
pub type CERDYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERDYIFG`"]
pub struct CERDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CERDYIFG_W<'a> {
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
#[doc = "Reader of field `CEIE`"]
pub type CEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIE`"]
pub struct CEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIE_W<'a> {
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
#[doc = "Reader of field `CEIIE`"]
pub type CEIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIIE`"]
pub struct CEIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CERDYIE`"]
pub type CERDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERDYIE`"]
pub struct CERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CERDYIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comp. E Interrupt Flag"]
    #[inline(always)]
    pub fn ceifg(&self) -> CEIFG_R {
        CEIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. E Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn ceiifg(&self) -> CEIIFG_R {
        CEIIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. E Comparator_E ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&self) -> CERDYIFG_R {
        CERDYIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. E Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. E Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn ceiie(&self) -> CEIIE_R {
        CEIIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. E Comparator_E ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&self) -> CERDYIE_R {
        CERDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. E Interrupt Flag"]
    #[inline(always)]
    pub fn ceifg(&mut self) -> CEIFG_W {
        CEIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comp. E Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn ceiifg(&mut self) -> CEIIFG_W {
        CEIIFG_W { w: self }
    }
    #[doc = "Bit 4 - Comp. E Comparator_E ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&mut self) -> CERDYIFG_W {
        CERDYIFG_W { w: self }
    }
    #[doc = "Bit 8 - Comp. E Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W {
        CEIE_W { w: self }
    }
    #[doc = "Bit 9 - Comp. E Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn ceiie(&mut self) -> CEIIE_W {
        CEIIE_W { w: self }
    }
    #[doc = "Bit 12 - Comp. E Comparator_E ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&mut self) -> CERDYIE_W {
        CERDYIE_W { w: self }
    }
}
