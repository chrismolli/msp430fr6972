#[doc = "Reader of register RTCTCMP"]
pub type R = crate::R<u16, super::RTCTCMP>;
#[doc = "Writer for register RTCTCMP"]
pub type W = crate::W<u16, super::RTCTCMP>;
#[doc = "Register RTCTCMP `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCTCMP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCTCMP0`"]
pub type RTCTCMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP0`"]
pub struct RTCTCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP0_W<'a> {
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
#[doc = "Reader of field `RTCTCMP1`"]
pub type RTCTCMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP1`"]
pub struct RTCTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP1_W<'a> {
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
#[doc = "Reader of field `RTCTCMP2`"]
pub type RTCTCMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP2`"]
pub struct RTCTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP2_W<'a> {
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
#[doc = "Reader of field `RTCTCMP3`"]
pub type RTCTCMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP3`"]
pub struct RTCTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP3_W<'a> {
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
#[doc = "Reader of field `RTCTCMP4`"]
pub type RTCTCMP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP4`"]
pub struct RTCTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP4_W<'a> {
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
#[doc = "Reader of field `RTCTCMP5`"]
pub type RTCTCMP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP5`"]
pub struct RTCTCMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP5_W<'a> {
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
#[doc = "Reader of field `RTCTCMP6`"]
pub type RTCTCMP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP6`"]
pub struct RTCTCMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP6_W<'a> {
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
#[doc = "Reader of field `RTCTCMP7`"]
pub type RTCTCMP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMP7`"]
pub struct RTCTCMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP7_W<'a> {
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
#[doc = "Reader of field `RTCTCOK`"]
pub type RTCTCOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCOK`"]
pub struct RTCTCOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCOK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTCTCRDY`"]
pub type RTCTCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCRDY`"]
pub struct RTCTCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTCTCMPS`"]
pub type RTCTCMPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCTCMPS`"]
pub struct RTCTCMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMPS_W<'a> {
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
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&self) -> RTCTCMP0_R {
        RTCTCMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&self) -> RTCTCMP1_R {
        RTCTCMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&self) -> RTCTCMP2_R {
        RTCTCMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&self) -> RTCTCMP3_R {
        RTCTCMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&self) -> RTCTCMP4_R {
        RTCTCMP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&self) -> RTCTCMP5_R {
        RTCTCMP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&self) -> RTCTCMP6_R {
        RTCTCMP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&self) -> RTCTCMP7_R {
        RTCTCMP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RTCTCOK_R {
        RTCTCOK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RTCTCRDY_R {
        RTCTCRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RTCTCMPS_R {
        RTCTCMPS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&mut self) -> RTCTCMP0_W {
        RTCTCMP0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&mut self) -> RTCTCMP1_W {
        RTCTCMP1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&mut self) -> RTCTCMP2_W {
        RTCTCMP2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&mut self) -> RTCTCMP3_W {
        RTCTCMP3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&mut self) -> RTCTCMP4_W {
        RTCTCMP4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&mut self) -> RTCTCMP5_W {
        RTCTCMP5_W { w: self }
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&mut self) -> RTCTCMP6_W {
        RTCTCMP6_W { w: self }
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&mut self) -> RTCTCMP7_W {
        RTCTCMP7_W { w: self }
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&mut self) -> RTCTCOK_W {
        RTCTCOK_W { w: self }
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&mut self) -> RTCTCRDY_W {
        RTCTCRDY_W { w: self }
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RTCTCMPS_W {
        RTCTCMPS_W { w: self }
    }
}
