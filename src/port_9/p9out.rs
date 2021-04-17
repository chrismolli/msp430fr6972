#[doc = "Reader of register P9OUT"]
pub type R = crate::R<u8, super::P9OUT>;
#[doc = "Writer for register P9OUT"]
pub type W = crate::W<u8, super::P9OUT>;
#[doc = "Register P9OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::P9OUT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9OUT0`"]
pub type P9OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT0`"]
pub struct P9OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `P9OUT1`"]
pub type P9OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT1`"]
pub struct P9OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `P9OUT2`"]
pub type P9OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT2`"]
pub struct P9OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `P9OUT3`"]
pub type P9OUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT3`"]
pub struct P9OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `P9OUT4`"]
pub type P9OUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT4`"]
pub struct P9OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `P9OUT5`"]
pub type P9OUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT5`"]
pub struct P9OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `P9OUT6`"]
pub type P9OUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT6`"]
pub struct P9OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `P9OUT7`"]
pub type P9OUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9OUT7`"]
pub struct P9OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&self) -> P9OUT0_R {
        P9OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&self) -> P9OUT1_R {
        P9OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&self) -> P9OUT2_R {
        P9OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&self) -> P9OUT3_R {
        P9OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&self) -> P9OUT4_R {
        P9OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&self) -> P9OUT5_R {
        P9OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&self) -> P9OUT6_R {
        P9OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&self) -> P9OUT7_R {
        P9OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&mut self) -> P9OUT0_W {
        P9OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&mut self) -> P9OUT1_W {
        P9OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&mut self) -> P9OUT2_W {
        P9OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&mut self) -> P9OUT3_W {
        P9OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&mut self) -> P9OUT4_W {
        P9OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&mut self) -> P9OUT5_W {
        P9OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&mut self) -> P9OUT6_W {
        P9OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&mut self) -> P9OUT7_W {
        P9OUT7_W { w: self }
    }
}
