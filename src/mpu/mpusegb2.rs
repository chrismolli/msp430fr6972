#[doc = "Reader of register MPUSEGB2"]
pub type R = crate::R<u16, super::MPUSEGB2>;
#[doc = "Writer for register MPUSEGB2"]
pub type W = crate::W<u16, super::MPUSEGB2>;
#[doc = "Register MPUSEGB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUSEGB2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUSEGB20`"]
pub type MPUSEGB20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB20`"]
pub struct MPUSEGB20_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB20_W<'a> {
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
#[doc = "Reader of field `MPUSEGB21`"]
pub type MPUSEGB21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB21`"]
pub struct MPUSEGB21_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB21_W<'a> {
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
#[doc = "Reader of field `MPUSEGB22`"]
pub type MPUSEGB22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB22`"]
pub struct MPUSEGB22_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB22_W<'a> {
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
#[doc = "Reader of field `MPUSEGB23`"]
pub type MPUSEGB23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB23`"]
pub struct MPUSEGB23_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB23_W<'a> {
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
#[doc = "Reader of field `MPUSEGB24`"]
pub type MPUSEGB24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB24`"]
pub struct MPUSEGB24_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB24_W<'a> {
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
#[doc = "Reader of field `MPUSEGB25`"]
pub type MPUSEGB25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB25`"]
pub struct MPUSEGB25_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB25_W<'a> {
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
#[doc = "Reader of field `MPUSEGB26`"]
pub type MPUSEGB26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB26`"]
pub struct MPUSEGB26_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB26_W<'a> {
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
#[doc = "Reader of field `MPUSEGB27`"]
pub type MPUSEGB27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB27`"]
pub struct MPUSEGB27_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB27_W<'a> {
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
#[doc = "Reader of field `MPUSEGB28`"]
pub type MPUSEGB28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB28`"]
pub struct MPUSEGB28_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB28_W<'a> {
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
#[doc = "Reader of field `MPUSEGB29`"]
pub type MPUSEGB29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB29`"]
pub struct MPUSEGB29_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB29_W<'a> {
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
#[doc = "Reader of field `MPUSEGB210`"]
pub type MPUSEGB210_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB210`"]
pub struct MPUSEGB210_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB210_W<'a> {
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
#[doc = "Reader of field `MPUSEGB211`"]
pub type MPUSEGB211_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB211`"]
pub struct MPUSEGB211_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB211_W<'a> {
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
#[doc = "Reader of field `MPUSEGB212`"]
pub type MPUSEGB212_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB212`"]
pub struct MPUSEGB212_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB212_W<'a> {
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
#[doc = "Reader of field `MPUSEGB213`"]
pub type MPUSEGB213_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB213`"]
pub struct MPUSEGB213_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB213_W<'a> {
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
#[doc = "Reader of field `MPUSEGB214`"]
pub type MPUSEGB214_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB214`"]
pub struct MPUSEGB214_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB214_W<'a> {
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
#[doc = "Reader of field `MPUSEGB215`"]
pub type MPUSEGB215_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGB215`"]
pub struct MPUSEGB215_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB215_W<'a> {
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
    #[doc = "Bit 0 - MPU Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb20(&self) -> MPUSEGB20_R {
        MPUSEGB20_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb21(&self) -> MPUSEGB21_R {
        MPUSEGB21_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb22(&self) -> MPUSEGB22_R {
        MPUSEGB22_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb23(&self) -> MPUSEGB23_R {
        MPUSEGB23_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb24(&self) -> MPUSEGB24_R {
        MPUSEGB24_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb25(&self) -> MPUSEGB25_R {
        MPUSEGB25_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb26(&self) -> MPUSEGB26_R {
        MPUSEGB26_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb27(&self) -> MPUSEGB27_R {
        MPUSEGB27_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb28(&self) -> MPUSEGB28_R {
        MPUSEGB28_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb29(&self) -> MPUSEGB29_R {
        MPUSEGB29_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb210(&self) -> MPUSEGB210_R {
        MPUSEGB210_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb211(&self) -> MPUSEGB211_R {
        MPUSEGB211_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb212(&self) -> MPUSEGB212_R {
        MPUSEGB212_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb213(&self) -> MPUSEGB213_R {
        MPUSEGB213_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb214(&self) -> MPUSEGB214_R {
        MPUSEGB214_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb215(&self) -> MPUSEGB215_R {
        MPUSEGB215_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb20(&mut self) -> MPUSEGB20_W {
        MPUSEGB20_W { w: self }
    }
    #[doc = "Bit 1 - MPU Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb21(&mut self) -> MPUSEGB21_W {
        MPUSEGB21_W { w: self }
    }
    #[doc = "Bit 2 - MPU Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb22(&mut self) -> MPUSEGB22_W {
        MPUSEGB22_W { w: self }
    }
    #[doc = "Bit 3 - MPU Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb23(&mut self) -> MPUSEGB23_W {
        MPUSEGB23_W { w: self }
    }
    #[doc = "Bit 4 - MPU Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb24(&mut self) -> MPUSEGB24_W {
        MPUSEGB24_W { w: self }
    }
    #[doc = "Bit 5 - MPU Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb25(&mut self) -> MPUSEGB25_W {
        MPUSEGB25_W { w: self }
    }
    #[doc = "Bit 6 - MPU Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb26(&mut self) -> MPUSEGB26_W {
        MPUSEGB26_W { w: self }
    }
    #[doc = "Bit 7 - MPU Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb27(&mut self) -> MPUSEGB27_W {
        MPUSEGB27_W { w: self }
    }
    #[doc = "Bit 8 - MPU Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb28(&mut self) -> MPUSEGB28_W {
        MPUSEGB28_W { w: self }
    }
    #[doc = "Bit 9 - MPU Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb29(&mut self) -> MPUSEGB29_W {
        MPUSEGB29_W { w: self }
    }
    #[doc = "Bit 10 - MPU Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb210(&mut self) -> MPUSEGB210_W {
        MPUSEGB210_W { w: self }
    }
    #[doc = "Bit 11 - MPU Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb211(&mut self) -> MPUSEGB211_W {
        MPUSEGB211_W { w: self }
    }
    #[doc = "Bit 12 - MPU Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb212(&mut self) -> MPUSEGB212_W {
        MPUSEGB212_W { w: self }
    }
    #[doc = "Bit 13 - MPU Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb213(&mut self) -> MPUSEGB213_W {
        MPUSEGB213_W { w: self }
    }
    #[doc = "Bit 14 - MPU Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb214(&mut self) -> MPUSEGB214_W {
        MPUSEGB214_W { w: self }
    }
    #[doc = "Bit 15 - MPU Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb215(&mut self) -> MPUSEGB215_W {
        MPUSEGB215_W { w: self }
    }
}
