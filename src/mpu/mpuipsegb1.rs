#[doc = "Reader of register MPUIPSEGB1"]
pub type R = crate::R<u16, super::MPUIPSEGB1>;
#[doc = "Writer for register MPUIPSEGB1"]
pub type W = crate::W<u16, super::MPUIPSEGB1>;
#[doc = "Register MPUIPSEGB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUIPSEGB1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUIPSEGB10`"]
pub type MPUIPSEGB10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB10`"]
pub struct MPUIPSEGB10_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB10_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB11`"]
pub type MPUIPSEGB11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB11`"]
pub struct MPUIPSEGB11_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB11_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB12`"]
pub type MPUIPSEGB12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB12`"]
pub struct MPUIPSEGB12_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB12_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB13`"]
pub type MPUIPSEGB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB13`"]
pub struct MPUIPSEGB13_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB13_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB14`"]
pub type MPUIPSEGB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB14`"]
pub struct MPUIPSEGB14_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB14_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB15`"]
pub type MPUIPSEGB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB15`"]
pub struct MPUIPSEGB15_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB15_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB16`"]
pub type MPUIPSEGB16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB16`"]
pub struct MPUIPSEGB16_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB16_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB17`"]
pub type MPUIPSEGB17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB17`"]
pub struct MPUIPSEGB17_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB17_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB18`"]
pub type MPUIPSEGB18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB18`"]
pub struct MPUIPSEGB18_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB18_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB19`"]
pub type MPUIPSEGB19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB19`"]
pub struct MPUIPSEGB19_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB19_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB110`"]
pub type MPUIPSEGB110_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB110`"]
pub struct MPUIPSEGB110_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB110_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MPUIPSEGB111`"]
pub type MPUIPSEGB111_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB111`"]
pub struct MPUIPSEGB111_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB111_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB112`"]
pub type MPUIPSEGB112_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB112`"]
pub struct MPUIPSEGB112_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB112_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB113`"]
pub type MPUIPSEGB113_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB113`"]
pub struct MPUIPSEGB113_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB113_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB114`"]
pub type MPUIPSEGB114_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB114`"]
pub struct MPUIPSEGB114_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB114_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB115`"]
pub type MPUIPSEGB115_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB115`"]
pub struct MPUIPSEGB115_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB115_W<'a> {
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
    #[doc = "Bit 0 - MPU IP Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb10(&self) -> MPUIPSEGB10_R {
        MPUIPSEGB10_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU IP Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb11(&self) -> MPUIPSEGB11_R {
        MPUIPSEGB11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU IP Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb12(&self) -> MPUIPSEGB12_R {
        MPUIPSEGB12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU IP Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb13(&self) -> MPUIPSEGB13_R {
        MPUIPSEGB13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU IP Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb14(&self) -> MPUIPSEGB14_R {
        MPUIPSEGB14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU IP Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb15(&self) -> MPUIPSEGB15_R {
        MPUIPSEGB15_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU IP Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb16(&self) -> MPUIPSEGB16_R {
        MPUIPSEGB16_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU IP Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb17(&self) -> MPUIPSEGB17_R {
        MPUIPSEGB17_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU IP Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb18(&self) -> MPUIPSEGB18_R {
        MPUIPSEGB18_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU IP Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb19(&self) -> MPUIPSEGB19_R {
        MPUIPSEGB19_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU IP Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb110(&self) -> MPUIPSEGB110_R {
        MPUIPSEGB110_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU IP Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb111(&self) -> MPUIPSEGB111_R {
        MPUIPSEGB111_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU IP Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb112(&self) -> MPUIPSEGB112_R {
        MPUIPSEGB112_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU IP Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb113(&self) -> MPUIPSEGB113_R {
        MPUIPSEGB113_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU IP Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb114(&self) -> MPUIPSEGB114_R {
        MPUIPSEGB114_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU IP Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb115(&self) -> MPUIPSEGB115_R {
        MPUIPSEGB115_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU IP Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb10(&mut self) -> MPUIPSEGB10_W {
        MPUIPSEGB10_W { w: self }
    }
    #[doc = "Bit 1 - MPU IP Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb11(&mut self) -> MPUIPSEGB11_W {
        MPUIPSEGB11_W { w: self }
    }
    #[doc = "Bit 2 - MPU IP Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb12(&mut self) -> MPUIPSEGB12_W {
        MPUIPSEGB12_W { w: self }
    }
    #[doc = "Bit 3 - MPU IP Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb13(&mut self) -> MPUIPSEGB13_W {
        MPUIPSEGB13_W { w: self }
    }
    #[doc = "Bit 4 - MPU IP Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb14(&mut self) -> MPUIPSEGB14_W {
        MPUIPSEGB14_W { w: self }
    }
    #[doc = "Bit 5 - MPU IP Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb15(&mut self) -> MPUIPSEGB15_W {
        MPUIPSEGB15_W { w: self }
    }
    #[doc = "Bit 6 - MPU IP Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb16(&mut self) -> MPUIPSEGB16_W {
        MPUIPSEGB16_W { w: self }
    }
    #[doc = "Bit 7 - MPU IP Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb17(&mut self) -> MPUIPSEGB17_W {
        MPUIPSEGB17_W { w: self }
    }
    #[doc = "Bit 8 - MPU IP Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb18(&mut self) -> MPUIPSEGB18_W {
        MPUIPSEGB18_W { w: self }
    }
    #[doc = "Bit 9 - MPU IP Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb19(&mut self) -> MPUIPSEGB19_W {
        MPUIPSEGB19_W { w: self }
    }
    #[doc = "Bit 10 - MPU IP Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb110(&mut self) -> MPUIPSEGB110_W {
        MPUIPSEGB110_W { w: self }
    }
    #[doc = "Bit 11 - MPU IP Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb111(&mut self) -> MPUIPSEGB111_W {
        MPUIPSEGB111_W { w: self }
    }
    #[doc = "Bit 12 - MPU IP Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb112(&mut self) -> MPUIPSEGB112_W {
        MPUIPSEGB112_W { w: self }
    }
    #[doc = "Bit 13 - MPU IP Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb113(&mut self) -> MPUIPSEGB113_W {
        MPUIPSEGB113_W { w: self }
    }
    #[doc = "Bit 14 - MPU IP Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb114(&mut self) -> MPUIPSEGB114_W {
        MPUIPSEGB114_W { w: self }
    }
    #[doc = "Bit 15 - MPU IP Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb115(&mut self) -> MPUIPSEGB115_W {
        MPUIPSEGB115_W { w: self }
    }
}
