#[doc = "Reader of register MPUSEGB1"]
pub type R = crate::R<u16, super::MPUSEGB1>;
#[doc = "Writer for register MPUSEGB1"]
pub type W = crate::W<u16, super::MPUSEGB1>;
#[doc = "Register MPUSEGB1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUSEGB1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUSEGB10`"]
pub type MPUSEGB10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB10`"]
pub struct MPUSEGB10_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB10_W<'a> {
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
#[doc = "Reader of field `MPUSEGB11`"]
pub type MPUSEGB11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB11`"]
pub struct MPUSEGB11_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB11_W<'a> {
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
#[doc = "Reader of field `MPUSEGB12`"]
pub type MPUSEGB12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB12`"]
pub struct MPUSEGB12_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB12_W<'a> {
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
#[doc = "Reader of field `MPUSEGB13`"]
pub type MPUSEGB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB13`"]
pub struct MPUSEGB13_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB13_W<'a> {
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
#[doc = "Reader of field `MPUSEGB14`"]
pub type MPUSEGB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB14`"]
pub struct MPUSEGB14_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB14_W<'a> {
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
#[doc = "Reader of field `MPUSEGB15`"]
pub type MPUSEGB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB15`"]
pub struct MPUSEGB15_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB15_W<'a> {
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
#[doc = "Reader of field `MPUSEGB16`"]
pub type MPUSEGB16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB16`"]
pub struct MPUSEGB16_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB16_W<'a> {
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
#[doc = "Reader of field `MPUSEGB17`"]
pub type MPUSEGB17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB17`"]
pub struct MPUSEGB17_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB17_W<'a> {
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
#[doc = "Reader of field `MPUSEGB18`"]
pub type MPUSEGB18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB18`"]
pub struct MPUSEGB18_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB18_W<'a> {
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
#[doc = "Reader of field `MPUSEGB19`"]
pub type MPUSEGB19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB19`"]
pub struct MPUSEGB19_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB19_W<'a> {
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
#[doc = "Reader of field `MPUSEGB110`"]
pub type MPUSEGB110_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB110`"]
pub struct MPUSEGB110_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB110_W<'a> {
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
#[doc = "Reader of field `MPUSEGB111`"]
pub type MPUSEGB111_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB111`"]
pub struct MPUSEGB111_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB111_W<'a> {
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
#[doc = "Reader of field `MPUSEGB112`"]
pub type MPUSEGB112_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB112`"]
pub struct MPUSEGB112_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB112_W<'a> {
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
#[doc = "Reader of field `MPUSEGB113`"]
pub type MPUSEGB113_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB113`"]
pub struct MPUSEGB113_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB113_W<'a> {
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
#[doc = "Reader of field `MPUSEGB114`"]
pub type MPUSEGB114_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB114`"]
pub struct MPUSEGB114_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB114_W<'a> {
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
#[doc = "Reader of field `MPUSEGB115`"]
pub type MPUSEGB115_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB115`"]
pub struct MPUSEGB115_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB115_W<'a> {
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
    #[doc = "Bit 0 - MPU Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb10(&self) -> MPUSEGB10_R {
        MPUSEGB10_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb11(&self) -> MPUSEGB11_R {
        MPUSEGB11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb12(&self) -> MPUSEGB12_R {
        MPUSEGB12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb13(&self) -> MPUSEGB13_R {
        MPUSEGB13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb14(&self) -> MPUSEGB14_R {
        MPUSEGB14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb15(&self) -> MPUSEGB15_R {
        MPUSEGB15_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb16(&self) -> MPUSEGB16_R {
        MPUSEGB16_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb17(&self) -> MPUSEGB17_R {
        MPUSEGB17_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb18(&self) -> MPUSEGB18_R {
        MPUSEGB18_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb19(&self) -> MPUSEGB19_R {
        MPUSEGB19_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb110(&self) -> MPUSEGB110_R {
        MPUSEGB110_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb111(&self) -> MPUSEGB111_R {
        MPUSEGB111_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb112(&self) -> MPUSEGB112_R {
        MPUSEGB112_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb113(&self) -> MPUSEGB113_R {
        MPUSEGB113_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb114(&self) -> MPUSEGB114_R {
        MPUSEGB114_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb115(&self) -> MPUSEGB115_R {
        MPUSEGB115_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb10(&mut self) -> MPUSEGB10_W {
        MPUSEGB10_W { w: self }
    }
    #[doc = "Bit 1 - MPU Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb11(&mut self) -> MPUSEGB11_W {
        MPUSEGB11_W { w: self }
    }
    #[doc = "Bit 2 - MPU Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb12(&mut self) -> MPUSEGB12_W {
        MPUSEGB12_W { w: self }
    }
    #[doc = "Bit 3 - MPU Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb13(&mut self) -> MPUSEGB13_W {
        MPUSEGB13_W { w: self }
    }
    #[doc = "Bit 4 - MPU Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb14(&mut self) -> MPUSEGB14_W {
        MPUSEGB14_W { w: self }
    }
    #[doc = "Bit 5 - MPU Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb15(&mut self) -> MPUSEGB15_W {
        MPUSEGB15_W { w: self }
    }
    #[doc = "Bit 6 - MPU Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb16(&mut self) -> MPUSEGB16_W {
        MPUSEGB16_W { w: self }
    }
    #[doc = "Bit 7 - MPU Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb17(&mut self) -> MPUSEGB17_W {
        MPUSEGB17_W { w: self }
    }
    #[doc = "Bit 8 - MPU Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb18(&mut self) -> MPUSEGB18_W {
        MPUSEGB18_W { w: self }
    }
    #[doc = "Bit 9 - MPU Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb19(&mut self) -> MPUSEGB19_W {
        MPUSEGB19_W { w: self }
    }
    #[doc = "Bit 10 - MPU Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb110(&mut self) -> MPUSEGB110_W {
        MPUSEGB110_W { w: self }
    }
    #[doc = "Bit 11 - MPU Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb111(&mut self) -> MPUSEGB111_W {
        MPUSEGB111_W { w: self }
    }
    #[doc = "Bit 12 - MPU Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb112(&mut self) -> MPUSEGB112_W {
        MPUSEGB112_W { w: self }
    }
    #[doc = "Bit 13 - MPU Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb113(&mut self) -> MPUSEGB113_W {
        MPUSEGB113_W { w: self }
    }
    #[doc = "Bit 14 - MPU Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb114(&mut self) -> MPUSEGB114_W {
        MPUSEGB114_W { w: self }
    }
    #[doc = "Bit 15 - MPU Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb115(&mut self) -> MPUSEGB115_W {
        MPUSEGB115_W { w: self }
    }
}
