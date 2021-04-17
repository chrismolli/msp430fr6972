#[doc = "Reader of register RTCOCAL"]
pub type R = crate::R<u16, super::RTCOCAL>;
#[doc = "Writer for register RTCOCAL"]
pub type W = crate::W<u16, super::RTCOCAL>;
#[doc = "Register RTCOCAL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCOCAL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCOCAL0`"]
pub type RTCOCAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL0`"]
pub struct RTCOCAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL0_W<'a> {
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
#[doc = "Reader of field `RTCOCAL1`"]
pub type RTCOCAL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL1`"]
pub struct RTCOCAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL1_W<'a> {
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
#[doc = "Reader of field `RTCOCAL2`"]
pub type RTCOCAL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL2`"]
pub struct RTCOCAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCOCAL3`"]
pub type RTCOCAL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL3`"]
pub struct RTCOCAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL3_W<'a> {
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
#[doc = "Reader of field `RTCOCAL4`"]
pub type RTCOCAL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL4`"]
pub struct RTCOCAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL4_W<'a> {
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
#[doc = "Reader of field `RTCOCAL5`"]
pub type RTCOCAL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL5`"]
pub struct RTCOCAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTCOCAL6`"]
pub type RTCOCAL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL6`"]
pub struct RTCOCAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL6_W<'a> {
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
#[doc = "Reader of field `RTCOCAL7`"]
pub type RTCOCAL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCAL7`"]
pub struct RTCOCAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL7_W<'a> {
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
#[doc = "Reader of field `RTCOCALS`"]
pub type RTCOCALS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCOCALS`"]
pub struct RTCOCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCALS_W<'a> {
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
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&self) -> RTCOCAL0_R {
        RTCOCAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&self) -> RTCOCAL1_R {
        RTCOCAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&self) -> RTCOCAL2_R {
        RTCOCAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&self) -> RTCOCAL3_R {
        RTCOCAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&self) -> RTCOCAL4_R {
        RTCOCAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&self) -> RTCOCAL5_R {
        RTCOCAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&self) -> RTCOCAL6_R {
        RTCOCAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&self) -> RTCOCAL7_R {
        RTCOCAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&mut self) -> RTCOCAL0_W {
        RTCOCAL0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&mut self) -> RTCOCAL1_W {
        RTCOCAL1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&mut self) -> RTCOCAL2_W {
        RTCOCAL2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&mut self) -> RTCOCAL3_W {
        RTCOCAL3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&mut self) -> RTCOCAL4_W {
        RTCOCAL4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&mut self) -> RTCOCAL5_W {
        RTCOCAL5_W { w: self }
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&mut self) -> RTCOCAL6_W {
        RTCOCAL6_W { w: self }
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&mut self) -> RTCOCAL7_W {
        RTCOCAL7_W { w: self }
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W {
        RTCOCALS_W { w: self }
    }
}
