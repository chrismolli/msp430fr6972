#[doc = "Reader of register P1SELC"]
pub type R = crate::R<u8, super::P1SELC>;
#[doc = "Writer for register P1SELC"]
pub type W = crate::W<u8, super::P1SELC>;
#[doc = "Register P1SELC `reset()`'s with value 0"]
impl crate::ResetValue for super::P1SELC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P1SELC_0`"]
pub type P1SELC_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_0`"]
pub struct P1SELC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_0_W<'a> {
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
#[doc = "Reader of field `P1SELC_1`"]
pub type P1SELC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_1`"]
pub struct P1SELC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_1_W<'a> {
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
#[doc = "Reader of field `P1SELC_2`"]
pub type P1SELC_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_2`"]
pub struct P1SELC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_2_W<'a> {
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
#[doc = "Reader of field `P1SELC_3`"]
pub type P1SELC_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_3`"]
pub struct P1SELC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_3_W<'a> {
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
#[doc = "Reader of field `P1SELC_4`"]
pub type P1SELC_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_4`"]
pub struct P1SELC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_4_W<'a> {
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
#[doc = "Reader of field `P1SELC_5`"]
pub type P1SELC_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_5`"]
pub struct P1SELC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_5_W<'a> {
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
#[doc = "Reader of field `P1SELC_6`"]
pub type P1SELC_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_6`"]
pub struct P1SELC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_6_W<'a> {
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
#[doc = "Reader of field `P1SELC_7`"]
pub type P1SELC_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1SELC_7`"]
pub struct P1SELC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_7_W<'a> {
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
    #[doc = "Bit 0 - P1SELC_0"]
    #[inline(always)]
    pub fn p1selc_0(&self) -> P1SELC_0_R {
        P1SELC_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1SELC_1"]
    #[inline(always)]
    pub fn p1selc_1(&self) -> P1SELC_1_R {
        P1SELC_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1SELC_2"]
    #[inline(always)]
    pub fn p1selc_2(&self) -> P1SELC_2_R {
        P1SELC_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1SELC_3"]
    #[inline(always)]
    pub fn p1selc_3(&self) -> P1SELC_3_R {
        P1SELC_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1SELC_4"]
    #[inline(always)]
    pub fn p1selc_4(&self) -> P1SELC_4_R {
        P1SELC_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1SELC_5"]
    #[inline(always)]
    pub fn p1selc_5(&self) -> P1SELC_5_R {
        P1SELC_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1SELC_6"]
    #[inline(always)]
    pub fn p1selc_6(&self) -> P1SELC_6_R {
        P1SELC_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1SELC_7"]
    #[inline(always)]
    pub fn p1selc_7(&self) -> P1SELC_7_R {
        P1SELC_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SELC_0"]
    #[inline(always)]
    pub fn p1selc_0(&mut self) -> P1SELC_0_W {
        P1SELC_0_W { w: self }
    }
    #[doc = "Bit 1 - P1SELC_1"]
    #[inline(always)]
    pub fn p1selc_1(&mut self) -> P1SELC_1_W {
        P1SELC_1_W { w: self }
    }
    #[doc = "Bit 2 - P1SELC_2"]
    #[inline(always)]
    pub fn p1selc_2(&mut self) -> P1SELC_2_W {
        P1SELC_2_W { w: self }
    }
    #[doc = "Bit 3 - P1SELC_3"]
    #[inline(always)]
    pub fn p1selc_3(&mut self) -> P1SELC_3_W {
        P1SELC_3_W { w: self }
    }
    #[doc = "Bit 4 - P1SELC_4"]
    #[inline(always)]
    pub fn p1selc_4(&mut self) -> P1SELC_4_W {
        P1SELC_4_W { w: self }
    }
    #[doc = "Bit 5 - P1SELC_5"]
    #[inline(always)]
    pub fn p1selc_5(&mut self) -> P1SELC_5_W {
        P1SELC_5_W { w: self }
    }
    #[doc = "Bit 6 - P1SELC_6"]
    #[inline(always)]
    pub fn p1selc_6(&mut self) -> P1SELC_6_W {
        P1SELC_6_W { w: self }
    }
    #[doc = "Bit 7 - P1SELC_7"]
    #[inline(always)]
    pub fn p1selc_7(&mut self) -> P1SELC_7_W {
        P1SELC_7_W { w: self }
    }
}
