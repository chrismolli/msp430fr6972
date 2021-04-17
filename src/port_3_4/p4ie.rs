#[doc = "Reader of register P4IE"]
pub type R = crate::R<u8, super::P4IE>;
#[doc = "Writer for register P4IE"]
pub type W = crate::W<u8, super::P4IE>;
#[doc = "Register P4IE `reset()`'s with value 0"]
impl crate::ResetValue for super::P4IE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4IE0`"]
pub type P4IE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE0`"]
pub struct P4IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE0_W<'a> {
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
#[doc = "Reader of field `P4IE1`"]
pub type P4IE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE1`"]
pub struct P4IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE1_W<'a> {
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
#[doc = "Reader of field `P4IE2`"]
pub type P4IE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE2`"]
pub struct P4IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE2_W<'a> {
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
#[doc = "Reader of field `P4IE3`"]
pub type P4IE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE3`"]
pub struct P4IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE3_W<'a> {
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
#[doc = "Reader of field `P4IE4`"]
pub type P4IE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE4`"]
pub struct P4IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE4_W<'a> {
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
#[doc = "Reader of field `P4IE5`"]
pub type P4IE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE5`"]
pub struct P4IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE5_W<'a> {
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
#[doc = "Reader of field `P4IE6`"]
pub type P4IE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE6`"]
pub struct P4IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE6_W<'a> {
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
#[doc = "Reader of field `P4IE7`"]
pub type P4IE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IE7`"]
pub struct P4IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE7_W<'a> {
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
    #[doc = "Bit 0 - P4IE0"]
    #[inline(always)]
    pub fn p4ie0(&self) -> P4IE0_R {
        P4IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IE1"]
    #[inline(always)]
    pub fn p4ie1(&self) -> P4IE1_R {
        P4IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IE2"]
    #[inline(always)]
    pub fn p4ie2(&self) -> P4IE2_R {
        P4IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IE3"]
    #[inline(always)]
    pub fn p4ie3(&self) -> P4IE3_R {
        P4IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IE4"]
    #[inline(always)]
    pub fn p4ie4(&self) -> P4IE4_R {
        P4IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IE5"]
    #[inline(always)]
    pub fn p4ie5(&self) -> P4IE5_R {
        P4IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IE6"]
    #[inline(always)]
    pub fn p4ie6(&self) -> P4IE6_R {
        P4IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IE7"]
    #[inline(always)]
    pub fn p4ie7(&self) -> P4IE7_R {
        P4IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IE0"]
    #[inline(always)]
    pub fn p4ie0(&mut self) -> P4IE0_W {
        P4IE0_W { w: self }
    }
    #[doc = "Bit 1 - P4IE1"]
    #[inline(always)]
    pub fn p4ie1(&mut self) -> P4IE1_W {
        P4IE1_W { w: self }
    }
    #[doc = "Bit 2 - P4IE2"]
    #[inline(always)]
    pub fn p4ie2(&mut self) -> P4IE2_W {
        P4IE2_W { w: self }
    }
    #[doc = "Bit 3 - P4IE3"]
    #[inline(always)]
    pub fn p4ie3(&mut self) -> P4IE3_W {
        P4IE3_W { w: self }
    }
    #[doc = "Bit 4 - P4IE4"]
    #[inline(always)]
    pub fn p4ie4(&mut self) -> P4IE4_W {
        P4IE4_W { w: self }
    }
    #[doc = "Bit 5 - P4IE5"]
    #[inline(always)]
    pub fn p4ie5(&mut self) -> P4IE5_W {
        P4IE5_W { w: self }
    }
    #[doc = "Bit 6 - P4IE6"]
    #[inline(always)]
    pub fn p4ie6(&mut self) -> P4IE6_W {
        P4IE6_W { w: self }
    }
    #[doc = "Bit 7 - P4IE7"]
    #[inline(always)]
    pub fn p4ie7(&mut self) -> P4IE7_W {
        P4IE7_W { w: self }
    }
}
