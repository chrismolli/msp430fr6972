#[doc = "Reader of register P3IE"]
pub type R = crate::R<u8, super::P3IE>;
#[doc = "Writer for register P3IE"]
pub type W = crate::W<u8, super::P3IE>;
#[doc = "Register P3IE `reset()`'s with value 0"]
impl crate::ResetValue for super::P3IE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IE0`"]
pub type P3IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE0`"]
pub struct P3IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE0_W<'a> {
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
#[doc = "Reader of field `P3IE1`"]
pub type P3IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE1`"]
pub struct P3IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE1_W<'a> {
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
#[doc = "Reader of field `P3IE2`"]
pub type P3IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE2`"]
pub struct P3IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE2_W<'a> {
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
#[doc = "Reader of field `P3IE3`"]
pub type P3IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE3`"]
pub struct P3IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE3_W<'a> {
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
#[doc = "Reader of field `P3IE4`"]
pub type P3IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE4`"]
pub struct P3IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE4_W<'a> {
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
#[doc = "Reader of field `P3IE5`"]
pub type P3IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE5`"]
pub struct P3IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE5_W<'a> {
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
#[doc = "Reader of field `P3IE6`"]
pub type P3IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE6`"]
pub struct P3IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE6_W<'a> {
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
#[doc = "Reader of field `P3IE7`"]
pub type P3IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IE7`"]
pub struct P3IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE7_W<'a> {
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
    #[doc = "Bit 0 - P3IE0"]
    #[inline(always)]
    pub fn p3ie0(&self) -> P3IE0_R {
        P3IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IE1"]
    #[inline(always)]
    pub fn p3ie1(&self) -> P3IE1_R {
        P3IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IE2"]
    #[inline(always)]
    pub fn p3ie2(&self) -> P3IE2_R {
        P3IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IE3"]
    #[inline(always)]
    pub fn p3ie3(&self) -> P3IE3_R {
        P3IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IE4"]
    #[inline(always)]
    pub fn p3ie4(&self) -> P3IE4_R {
        P3IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IE5"]
    #[inline(always)]
    pub fn p3ie5(&self) -> P3IE5_R {
        P3IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IE6"]
    #[inline(always)]
    pub fn p3ie6(&self) -> P3IE6_R {
        P3IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IE7"]
    #[inline(always)]
    pub fn p3ie7(&self) -> P3IE7_R {
        P3IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IE0"]
    #[inline(always)]
    pub fn p3ie0(&mut self) -> P3IE0_W {
        P3IE0_W { w: self }
    }
    #[doc = "Bit 1 - P3IE1"]
    #[inline(always)]
    pub fn p3ie1(&mut self) -> P3IE1_W {
        P3IE1_W { w: self }
    }
    #[doc = "Bit 2 - P3IE2"]
    #[inline(always)]
    pub fn p3ie2(&mut self) -> P3IE2_W {
        P3IE2_W { w: self }
    }
    #[doc = "Bit 3 - P3IE3"]
    #[inline(always)]
    pub fn p3ie3(&mut self) -> P3IE3_W {
        P3IE3_W { w: self }
    }
    #[doc = "Bit 4 - P3IE4"]
    #[inline(always)]
    pub fn p3ie4(&mut self) -> P3IE4_W {
        P3IE4_W { w: self }
    }
    #[doc = "Bit 5 - P3IE5"]
    #[inline(always)]
    pub fn p3ie5(&mut self) -> P3IE5_W {
        P3IE5_W { w: self }
    }
    #[doc = "Bit 6 - P3IE6"]
    #[inline(always)]
    pub fn p3ie6(&mut self) -> P3IE6_W {
        P3IE6_W { w: self }
    }
    #[doc = "Bit 7 - P3IE7"]
    #[inline(always)]
    pub fn p3ie7(&mut self) -> P3IE7_W {
        P3IE7_W { w: self }
    }
}
