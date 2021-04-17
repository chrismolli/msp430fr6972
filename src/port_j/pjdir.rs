#[doc = "Reader of register PJDIR"]
pub type R = crate::R<u16, super::PJDIR>;
#[doc = "Writer for register PJDIR"]
pub type W = crate::W<u16, super::PJDIR>;
#[doc = "Register PJDIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PJDIR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJDIR0`"]
pub type PJDIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR0`"]
pub struct PJDIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR0_W<'a> {
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
#[doc = "Reader of field `PJDIR1`"]
pub type PJDIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR1`"]
pub struct PJDIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR1_W<'a> {
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
#[doc = "Reader of field `PJDIR2`"]
pub type PJDIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR2`"]
pub struct PJDIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR2_W<'a> {
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
#[doc = "Reader of field `PJDIR3`"]
pub type PJDIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR3`"]
pub struct PJDIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR3_W<'a> {
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
#[doc = "Reader of field `PJDIR4`"]
pub type PJDIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR4`"]
pub struct PJDIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR4_W<'a> {
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
#[doc = "Reader of field `PJDIR5`"]
pub type PJDIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR5`"]
pub struct PJDIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR5_W<'a> {
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
#[doc = "Reader of field `PJDIR6`"]
pub type PJDIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR6`"]
pub struct PJDIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR6_W<'a> {
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
#[doc = "Reader of field `PJDIR7`"]
pub type PJDIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJDIR7`"]
pub struct PJDIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR7_W<'a> {
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
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&self) -> PJDIR0_R {
        PJDIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&self) -> PJDIR1_R {
        PJDIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&self) -> PJDIR2_R {
        PJDIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&self) -> PJDIR3_R {
        PJDIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJDIR4"]
    #[inline(always)]
    pub fn pjdir4(&self) -> PJDIR4_R {
        PJDIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJDIR5"]
    #[inline(always)]
    pub fn pjdir5(&self) -> PJDIR5_R {
        PJDIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJDIR6"]
    #[inline(always)]
    pub fn pjdir6(&self) -> PJDIR6_R {
        PJDIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJDIR7"]
    #[inline(always)]
    pub fn pjdir7(&self) -> PJDIR7_R {
        PJDIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJDIR0"]
    #[inline(always)]
    pub fn pjdir0(&mut self) -> PJDIR0_W {
        PJDIR0_W { w: self }
    }
    #[doc = "Bit 1 - PJDIR1"]
    #[inline(always)]
    pub fn pjdir1(&mut self) -> PJDIR1_W {
        PJDIR1_W { w: self }
    }
    #[doc = "Bit 2 - PJDIR2"]
    #[inline(always)]
    pub fn pjdir2(&mut self) -> PJDIR2_W {
        PJDIR2_W { w: self }
    }
    #[doc = "Bit 3 - PJDIR3"]
    #[inline(always)]
    pub fn pjdir3(&mut self) -> PJDIR3_W {
        PJDIR3_W { w: self }
    }
    #[doc = "Bit 4 - PJDIR4"]
    #[inline(always)]
    pub fn pjdir4(&mut self) -> PJDIR4_W {
        PJDIR4_W { w: self }
    }
    #[doc = "Bit 5 - PJDIR5"]
    #[inline(always)]
    pub fn pjdir5(&mut self) -> PJDIR5_W {
        PJDIR5_W { w: self }
    }
    #[doc = "Bit 6 - PJDIR6"]
    #[inline(always)]
    pub fn pjdir6(&mut self) -> PJDIR6_W {
        PJDIR6_W { w: self }
    }
    #[doc = "Bit 7 - PJDIR7"]
    #[inline(always)]
    pub fn pjdir7(&mut self) -> PJDIR7_W {
        PJDIR7_W { w: self }
    }
}
