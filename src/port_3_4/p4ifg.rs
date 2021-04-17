#[doc = "Reader of register P4IFG"]
pub type R = crate::R<u8, super::P4IFG>;
#[doc = "Writer for register P4IFG"]
pub type W = crate::W<u8, super::P4IFG>;
#[doc = "Register P4IFG `reset()`'s with value 0"]
impl crate::ResetValue for super::P4IFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4IFG0`"]
pub type P4IFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG0`"]
pub struct P4IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG0_W<'a> {
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
#[doc = "Reader of field `P4IFG1`"]
pub type P4IFG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG1`"]
pub struct P4IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG1_W<'a> {
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
#[doc = "Reader of field `P4IFG2`"]
pub type P4IFG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG2`"]
pub struct P4IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG2_W<'a> {
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
#[doc = "Reader of field `P4IFG3`"]
pub type P4IFG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG3`"]
pub struct P4IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG3_W<'a> {
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
#[doc = "Reader of field `P4IFG4`"]
pub type P4IFG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG4`"]
pub struct P4IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG4_W<'a> {
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
#[doc = "Reader of field `P4IFG5`"]
pub type P4IFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG5`"]
pub struct P4IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG5_W<'a> {
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
#[doc = "Reader of field `P4IFG6`"]
pub type P4IFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG6`"]
pub struct P4IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG6_W<'a> {
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
#[doc = "Reader of field `P4IFG7`"]
pub type P4IFG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IFG7`"]
pub struct P4IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG7_W<'a> {
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
    #[doc = "Bit 0 - P4IFG0"]
    #[inline(always)]
    pub fn p4ifg0(&self) -> P4IFG0_R {
        P4IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IFG1"]
    #[inline(always)]
    pub fn p4ifg1(&self) -> P4IFG1_R {
        P4IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IFG2"]
    #[inline(always)]
    pub fn p4ifg2(&self) -> P4IFG2_R {
        P4IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IFG3"]
    #[inline(always)]
    pub fn p4ifg3(&self) -> P4IFG3_R {
        P4IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IFG4"]
    #[inline(always)]
    pub fn p4ifg4(&self) -> P4IFG4_R {
        P4IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IFG5"]
    #[inline(always)]
    pub fn p4ifg5(&self) -> P4IFG5_R {
        P4IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IFG6"]
    #[inline(always)]
    pub fn p4ifg6(&self) -> P4IFG6_R {
        P4IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IFG7"]
    #[inline(always)]
    pub fn p4ifg7(&self) -> P4IFG7_R {
        P4IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IFG0"]
    #[inline(always)]
    pub fn p4ifg0(&mut self) -> P4IFG0_W {
        P4IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P4IFG1"]
    #[inline(always)]
    pub fn p4ifg1(&mut self) -> P4IFG1_W {
        P4IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P4IFG2"]
    #[inline(always)]
    pub fn p4ifg2(&mut self) -> P4IFG2_W {
        P4IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P4IFG3"]
    #[inline(always)]
    pub fn p4ifg3(&mut self) -> P4IFG3_W {
        P4IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P4IFG4"]
    #[inline(always)]
    pub fn p4ifg4(&mut self) -> P4IFG4_W {
        P4IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P4IFG5"]
    #[inline(always)]
    pub fn p4ifg5(&mut self) -> P4IFG5_W {
        P4IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P4IFG6"]
    #[inline(always)]
    pub fn p4ifg6(&mut self) -> P4IFG6_W {
        P4IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P4IFG7"]
    #[inline(always)]
    pub fn p4ifg7(&mut self) -> P4IFG7_W {
        P4IFG7_W { w: self }
    }
}
