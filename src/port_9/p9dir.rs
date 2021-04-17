#[doc = "Reader of register P9DIR"]
pub type R = crate::R<u8, super::P9DIR>;
#[doc = "Writer for register P9DIR"]
pub type W = crate::W<u8, super::P9DIR>;
#[doc = "Register P9DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::P9DIR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9DIR0`"]
pub type P9DIR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR0`"]
pub struct P9DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR0_W<'a> {
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
#[doc = "Reader of field `P9DIR1`"]
pub type P9DIR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR1`"]
pub struct P9DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR1_W<'a> {
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
#[doc = "Reader of field `P9DIR2`"]
pub type P9DIR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR2`"]
pub struct P9DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR2_W<'a> {
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
#[doc = "Reader of field `P9DIR3`"]
pub type P9DIR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR3`"]
pub struct P9DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR3_W<'a> {
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
#[doc = "Reader of field `P9DIR4`"]
pub type P9DIR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR4`"]
pub struct P9DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR4_W<'a> {
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
#[doc = "Reader of field `P9DIR5`"]
pub type P9DIR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR5`"]
pub struct P9DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR5_W<'a> {
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
#[doc = "Reader of field `P9DIR6`"]
pub type P9DIR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR6`"]
pub struct P9DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR6_W<'a> {
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
#[doc = "Reader of field `P9DIR7`"]
pub type P9DIR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9DIR7`"]
pub struct P9DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR7_W<'a> {
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
    #[doc = "Bit 0 - P9DIR0"]
    #[inline(always)]
    pub fn p9dir0(&self) -> P9DIR0_R {
        P9DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9DIR1"]
    #[inline(always)]
    pub fn p9dir1(&self) -> P9DIR1_R {
        P9DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9DIR2"]
    #[inline(always)]
    pub fn p9dir2(&self) -> P9DIR2_R {
        P9DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9DIR3"]
    #[inline(always)]
    pub fn p9dir3(&self) -> P9DIR3_R {
        P9DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9DIR4"]
    #[inline(always)]
    pub fn p9dir4(&self) -> P9DIR4_R {
        P9DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9DIR5"]
    #[inline(always)]
    pub fn p9dir5(&self) -> P9DIR5_R {
        P9DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9DIR6"]
    #[inline(always)]
    pub fn p9dir6(&self) -> P9DIR6_R {
        P9DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9DIR7"]
    #[inline(always)]
    pub fn p9dir7(&self) -> P9DIR7_R {
        P9DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9DIR0"]
    #[inline(always)]
    pub fn p9dir0(&mut self) -> P9DIR0_W {
        P9DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P9DIR1"]
    #[inline(always)]
    pub fn p9dir1(&mut self) -> P9DIR1_W {
        P9DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P9DIR2"]
    #[inline(always)]
    pub fn p9dir2(&mut self) -> P9DIR2_W {
        P9DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P9DIR3"]
    #[inline(always)]
    pub fn p9dir3(&mut self) -> P9DIR3_W {
        P9DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P9DIR4"]
    #[inline(always)]
    pub fn p9dir4(&mut self) -> P9DIR4_W {
        P9DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P9DIR5"]
    #[inline(always)]
    pub fn p9dir5(&mut self) -> P9DIR5_W {
        P9DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P9DIR6"]
    #[inline(always)]
    pub fn p9dir6(&mut self) -> P9DIR6_W {
        P9DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P9DIR7"]
    #[inline(always)]
    pub fn p9dir7(&mut self) -> P9DIR7_W {
        P9DIR7_W { w: self }
    }
}
