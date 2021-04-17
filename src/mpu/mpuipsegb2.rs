#[doc = "Reader of register MPUIPSEGB2"]
pub type R = crate::R<u16, super::MPUIPSEGB2>;
#[doc = "Writer for register MPUIPSEGB2"]
pub type W = crate::W<u16, super::MPUIPSEGB2>;
#[doc = "Register MPUIPSEGB2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUIPSEGB2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUIPSEGB20`"]
pub type MPUIPSEGB20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB20`"]
pub struct MPUIPSEGB20_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB20_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB21`"]
pub type MPUIPSEGB21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB21`"]
pub struct MPUIPSEGB21_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB21_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB22`"]
pub type MPUIPSEGB22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB22`"]
pub struct MPUIPSEGB22_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB22_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB23`"]
pub type MPUIPSEGB23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB23`"]
pub struct MPUIPSEGB23_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB23_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB24`"]
pub type MPUIPSEGB24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB24`"]
pub struct MPUIPSEGB24_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB24_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB25`"]
pub type MPUIPSEGB25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB25`"]
pub struct MPUIPSEGB25_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB25_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB26`"]
pub type MPUIPSEGB26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB26`"]
pub struct MPUIPSEGB26_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB26_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB27`"]
pub type MPUIPSEGB27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB27`"]
pub struct MPUIPSEGB27_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB27_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB28`"]
pub type MPUIPSEGB28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB28`"]
pub struct MPUIPSEGB28_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB28_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB29`"]
pub type MPUIPSEGB29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB29`"]
pub struct MPUIPSEGB29_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB29_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB210`"]
pub type MPUIPSEGB210_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB210`"]
pub struct MPUIPSEGB210_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB210_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB211`"]
pub type MPUIPSEGB211_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB211`"]
pub struct MPUIPSEGB211_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB211_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB212`"]
pub type MPUIPSEGB212_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB212`"]
pub struct MPUIPSEGB212_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB212_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB213`"]
pub type MPUIPSEGB213_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB213`"]
pub struct MPUIPSEGB213_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB213_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB214`"]
pub type MPUIPSEGB214_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB214`"]
pub struct MPUIPSEGB214_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB214_W<'a> {
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
#[doc = "Reader of field `MPUIPSEGB215`"]
pub type MPUIPSEGB215_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPSEGB215`"]
pub struct MPUIPSEGB215_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB215_W<'a> {
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
    #[doc = "Bit 0 - MPU IP Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb20(&self) -> MPUIPSEGB20_R {
        MPUIPSEGB20_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU IP Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb21(&self) -> MPUIPSEGB21_R {
        MPUIPSEGB21_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU IP Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb22(&self) -> MPUIPSEGB22_R {
        MPUIPSEGB22_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU IP Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb23(&self) -> MPUIPSEGB23_R {
        MPUIPSEGB23_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU IP Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb24(&self) -> MPUIPSEGB24_R {
        MPUIPSEGB24_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU IP Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb25(&self) -> MPUIPSEGB25_R {
        MPUIPSEGB25_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU IP Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb26(&self) -> MPUIPSEGB26_R {
        MPUIPSEGB26_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU IP Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb27(&self) -> MPUIPSEGB27_R {
        MPUIPSEGB27_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU IP Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb28(&self) -> MPUIPSEGB28_R {
        MPUIPSEGB28_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU IP Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb29(&self) -> MPUIPSEGB29_R {
        MPUIPSEGB29_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU IP Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb210(&self) -> MPUIPSEGB210_R {
        MPUIPSEGB210_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU IP Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb211(&self) -> MPUIPSEGB211_R {
        MPUIPSEGB211_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU IP Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb212(&self) -> MPUIPSEGB212_R {
        MPUIPSEGB212_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU IP Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb213(&self) -> MPUIPSEGB213_R {
        MPUIPSEGB213_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU IP Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb214(&self) -> MPUIPSEGB214_R {
        MPUIPSEGB214_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU IP Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb215(&self) -> MPUIPSEGB215_R {
        MPUIPSEGB215_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU IP Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb20(&mut self) -> MPUIPSEGB20_W {
        MPUIPSEGB20_W { w: self }
    }
    #[doc = "Bit 1 - MPU IP Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb21(&mut self) -> MPUIPSEGB21_W {
        MPUIPSEGB21_W { w: self }
    }
    #[doc = "Bit 2 - MPU IP Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb22(&mut self) -> MPUIPSEGB22_W {
        MPUIPSEGB22_W { w: self }
    }
    #[doc = "Bit 3 - MPU IP Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb23(&mut self) -> MPUIPSEGB23_W {
        MPUIPSEGB23_W { w: self }
    }
    #[doc = "Bit 4 - MPU IP Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb24(&mut self) -> MPUIPSEGB24_W {
        MPUIPSEGB24_W { w: self }
    }
    #[doc = "Bit 5 - MPU IP Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb25(&mut self) -> MPUIPSEGB25_W {
        MPUIPSEGB25_W { w: self }
    }
    #[doc = "Bit 6 - MPU IP Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb26(&mut self) -> MPUIPSEGB26_W {
        MPUIPSEGB26_W { w: self }
    }
    #[doc = "Bit 7 - MPU IP Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb27(&mut self) -> MPUIPSEGB27_W {
        MPUIPSEGB27_W { w: self }
    }
    #[doc = "Bit 8 - MPU IP Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb28(&mut self) -> MPUIPSEGB28_W {
        MPUIPSEGB28_W { w: self }
    }
    #[doc = "Bit 9 - MPU IP Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb29(&mut self) -> MPUIPSEGB29_W {
        MPUIPSEGB29_W { w: self }
    }
    #[doc = "Bit 10 - MPU IP Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb210(&mut self) -> MPUIPSEGB210_W {
        MPUIPSEGB210_W { w: self }
    }
    #[doc = "Bit 11 - MPU IP Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb211(&mut self) -> MPUIPSEGB211_W {
        MPUIPSEGB211_W { w: self }
    }
    #[doc = "Bit 12 - MPU IP Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb212(&mut self) -> MPUIPSEGB212_W {
        MPUIPSEGB212_W { w: self }
    }
    #[doc = "Bit 13 - MPU IP Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb213(&mut self) -> MPUIPSEGB213_W {
        MPUIPSEGB213_W { w: self }
    }
    #[doc = "Bit 14 - MPU IP Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb214(&mut self) -> MPUIPSEGB214_W {
        MPUIPSEGB214_W { w: self }
    }
    #[doc = "Bit 15 - MPU IP Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb215(&mut self) -> MPUIPSEGB215_W {
        MPUIPSEGB215_W { w: self }
    }
}
