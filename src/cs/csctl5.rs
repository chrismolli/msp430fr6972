#[doc = "Reader of register CSCTL5"]
pub type R = crate::R<u16, super::CSCTL5>;
#[doc = "Writer for register CSCTL5"]
pub type W = crate::W<u16, super::CSCTL5>;
#[doc = "Register CSCTL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFXTOFFG`"]
pub type LFXTOFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFXTOFFG`"]
pub struct LFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFFG_W<'a> {
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
#[doc = "Reader of field `HFXTOFFG`"]
pub type HFXTOFFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFXTOFFG`"]
pub struct HFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFFG_W<'a> {
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
#[doc = "Reader of field `ENSTFCNT1`"]
pub type ENSTFCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSTFCNT1`"]
pub struct ENSTFCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ENSTFCNT2`"]
pub type ENSTFCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENSTFCNT2`"]
pub struct ENSTFCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LFXT Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxtoffg(&self) -> LFXTOFFG_R {
        LFXTOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXT High Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn hfxtoffg(&self) -> HFXTOFFG_R {
        HFXTOFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable start counter for XT2"]
    #[inline(always)]
    pub fn enstfcnt2(&self) -> ENSTFCNT2_R {
        ENSTFCNT2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxtoffg(&mut self) -> LFXTOFFG_W {
        LFXTOFFG_W { w: self }
    }
    #[doc = "Bit 1 - HFXT High Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn hfxtoffg(&mut self) -> HFXTOFFG_W {
        HFXTOFFG_W { w: self }
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W {
        ENSTFCNT1_W { w: self }
    }
    #[doc = "Bit 7 - Enable start counter for XT2"]
    #[inline(always)]
    pub fn enstfcnt2(&mut self) -> ENSTFCNT2_W {
        ENSTFCNT2_W { w: self }
    }
}
