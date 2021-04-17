#[doc = "Reader of register P9REN"]
pub type R = crate::R<u8, super::P9REN>;
#[doc = "Writer for register P9REN"]
pub type W = crate::W<u8, super::P9REN>;
#[doc = "Register P9REN `reset()`'s with value 0"]
impl crate::ResetValue for super::P9REN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9REN0`"]
pub type P9REN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN0`"]
pub struct P9REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN0_W<'a> {
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
#[doc = "Reader of field `P9REN1`"]
pub type P9REN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN1`"]
pub struct P9REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN1_W<'a> {
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
#[doc = "Reader of field `P9REN2`"]
pub type P9REN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN2`"]
pub struct P9REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN2_W<'a> {
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
#[doc = "Reader of field `P9REN3`"]
pub type P9REN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN3`"]
pub struct P9REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN3_W<'a> {
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
#[doc = "Reader of field `P9REN4`"]
pub type P9REN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN4`"]
pub struct P9REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN4_W<'a> {
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
#[doc = "Reader of field `P9REN5`"]
pub type P9REN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN5`"]
pub struct P9REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN5_W<'a> {
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
#[doc = "Reader of field `P9REN6`"]
pub type P9REN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN6`"]
pub struct P9REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN6_W<'a> {
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
#[doc = "Reader of field `P9REN7`"]
pub type P9REN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9REN7`"]
pub struct P9REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN7_W<'a> {
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
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&self) -> P9REN0_R {
        P9REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&self) -> P9REN1_R {
        P9REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&self) -> P9REN2_R {
        P9REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&self) -> P9REN3_R {
        P9REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&self) -> P9REN4_R {
        P9REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&self) -> P9REN5_R {
        P9REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&self) -> P9REN6_R {
        P9REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&self) -> P9REN7_R {
        P9REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&mut self) -> P9REN0_W {
        P9REN0_W { w: self }
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&mut self) -> P9REN1_W {
        P9REN1_W { w: self }
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&mut self) -> P9REN2_W {
        P9REN2_W { w: self }
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&mut self) -> P9REN3_W {
        P9REN3_W { w: self }
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&mut self) -> P9REN4_W {
        P9REN4_W { w: self }
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&mut self) -> P9REN5_W {
        P9REN5_W { w: self }
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&mut self) -> P9REN6_W {
        P9REN6_W { w: self }
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&mut self) -> P9REN7_W {
        P9REN7_W { w: self }
    }
}
