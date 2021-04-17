#[doc = "Reader of register P3IFG"]
pub type R = crate::R<u8, super::P3IFG>;
#[doc = "Writer for register P3IFG"]
pub type W = crate::W<u8, super::P3IFG>;
#[doc = "Register P3IFG `reset()`'s with value 0"]
impl crate::ResetValue for super::P3IFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IFG0`"]
pub type P3IFG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG0`"]
pub struct P3IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG0_W<'a> {
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
#[doc = "Reader of field `P3IFG1`"]
pub type P3IFG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG1`"]
pub struct P3IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG1_W<'a> {
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
#[doc = "Reader of field `P3IFG2`"]
pub type P3IFG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG2`"]
pub struct P3IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG2_W<'a> {
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
#[doc = "Reader of field `P3IFG3`"]
pub type P3IFG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG3`"]
pub struct P3IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG3_W<'a> {
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
#[doc = "Reader of field `P3IFG4`"]
pub type P3IFG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG4`"]
pub struct P3IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG4_W<'a> {
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
#[doc = "Reader of field `P3IFG5`"]
pub type P3IFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG5`"]
pub struct P3IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG5_W<'a> {
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
#[doc = "Reader of field `P3IFG6`"]
pub type P3IFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG6`"]
pub struct P3IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG6_W<'a> {
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
#[doc = "Reader of field `P3IFG7`"]
pub type P3IFG7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IFG7`"]
pub struct P3IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG7_W<'a> {
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
    #[doc = "Bit 0 - P3IFG0"]
    #[inline(always)]
    pub fn p3ifg0(&self) -> P3IFG0_R {
        P3IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IFG1"]
    #[inline(always)]
    pub fn p3ifg1(&self) -> P3IFG1_R {
        P3IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IFG2"]
    #[inline(always)]
    pub fn p3ifg2(&self) -> P3IFG2_R {
        P3IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IFG3"]
    #[inline(always)]
    pub fn p3ifg3(&self) -> P3IFG3_R {
        P3IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IFG4"]
    #[inline(always)]
    pub fn p3ifg4(&self) -> P3IFG4_R {
        P3IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IFG5"]
    #[inline(always)]
    pub fn p3ifg5(&self) -> P3IFG5_R {
        P3IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IFG6"]
    #[inline(always)]
    pub fn p3ifg6(&self) -> P3IFG6_R {
        P3IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IFG7"]
    #[inline(always)]
    pub fn p3ifg7(&self) -> P3IFG7_R {
        P3IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IFG0"]
    #[inline(always)]
    pub fn p3ifg0(&mut self) -> P3IFG0_W {
        P3IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P3IFG1"]
    #[inline(always)]
    pub fn p3ifg1(&mut self) -> P3IFG1_W {
        P3IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P3IFG2"]
    #[inline(always)]
    pub fn p3ifg2(&mut self) -> P3IFG2_W {
        P3IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P3IFG3"]
    #[inline(always)]
    pub fn p3ifg3(&mut self) -> P3IFG3_W {
        P3IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P3IFG4"]
    #[inline(always)]
    pub fn p3ifg4(&mut self) -> P3IFG4_W {
        P3IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P3IFG5"]
    #[inline(always)]
    pub fn p3ifg5(&mut self) -> P3IFG5_W {
        P3IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P3IFG6"]
    #[inline(always)]
    pub fn p3ifg6(&mut self) -> P3IFG6_W {
        P3IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P3IFG7"]
    #[inline(always)]
    pub fn p3ifg7(&mut self) -> P3IFG7_W {
        P3IFG7_W { w: self }
    }
}
