#[doc = "Reader of register P3IES"]
pub type R = crate::R<u8, super::P3IES>;
#[doc = "Writer for register P3IES"]
pub type W = crate::W<u8, super::P3IES>;
#[doc = "Register P3IES `reset()`'s with value 0"]
impl crate::ResetValue for super::P3IES {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3IES0`"]
pub type P3IES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES0`"]
pub struct P3IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES0_W<'a> {
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
#[doc = "Reader of field `P3IES1`"]
pub type P3IES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES1`"]
pub struct P3IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES1_W<'a> {
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
#[doc = "Reader of field `P3IES2`"]
pub type P3IES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES2`"]
pub struct P3IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES2_W<'a> {
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
#[doc = "Reader of field `P3IES3`"]
pub type P3IES3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES3`"]
pub struct P3IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES3_W<'a> {
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
#[doc = "Reader of field `P3IES4`"]
pub type P3IES4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES4`"]
pub struct P3IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES4_W<'a> {
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
#[doc = "Reader of field `P3IES5`"]
pub type P3IES5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES5`"]
pub struct P3IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES5_W<'a> {
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
#[doc = "Reader of field `P3IES6`"]
pub type P3IES6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES6`"]
pub struct P3IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES6_W<'a> {
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
#[doc = "Reader of field `P3IES7`"]
pub type P3IES7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3IES7`"]
pub struct P3IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IES7_W<'a> {
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
    #[doc = "Bit 0 - P3IES0"]
    #[inline(always)]
    pub fn p3ies0(&self) -> P3IES0_R {
        P3IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IES1"]
    #[inline(always)]
    pub fn p3ies1(&self) -> P3IES1_R {
        P3IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IES2"]
    #[inline(always)]
    pub fn p3ies2(&self) -> P3IES2_R {
        P3IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IES3"]
    #[inline(always)]
    pub fn p3ies3(&self) -> P3IES3_R {
        P3IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IES4"]
    #[inline(always)]
    pub fn p3ies4(&self) -> P3IES4_R {
        P3IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IES5"]
    #[inline(always)]
    pub fn p3ies5(&self) -> P3IES5_R {
        P3IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IES6"]
    #[inline(always)]
    pub fn p3ies6(&self) -> P3IES6_R {
        P3IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IES7"]
    #[inline(always)]
    pub fn p3ies7(&self) -> P3IES7_R {
        P3IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IES0"]
    #[inline(always)]
    pub fn p3ies0(&mut self) -> P3IES0_W {
        P3IES0_W { w: self }
    }
    #[doc = "Bit 1 - P3IES1"]
    #[inline(always)]
    pub fn p3ies1(&mut self) -> P3IES1_W {
        P3IES1_W { w: self }
    }
    #[doc = "Bit 2 - P3IES2"]
    #[inline(always)]
    pub fn p3ies2(&mut self) -> P3IES2_W {
        P3IES2_W { w: self }
    }
    #[doc = "Bit 3 - P3IES3"]
    #[inline(always)]
    pub fn p3ies3(&mut self) -> P3IES3_W {
        P3IES3_W { w: self }
    }
    #[doc = "Bit 4 - P3IES4"]
    #[inline(always)]
    pub fn p3ies4(&mut self) -> P3IES4_W {
        P3IES4_W { w: self }
    }
    #[doc = "Bit 5 - P3IES5"]
    #[inline(always)]
    pub fn p3ies5(&mut self) -> P3IES5_W {
        P3IES5_W { w: self }
    }
    #[doc = "Bit 6 - P3IES6"]
    #[inline(always)]
    pub fn p3ies6(&mut self) -> P3IES6_W {
        P3IES6_W { w: self }
    }
    #[doc = "Bit 7 - P3IES7"]
    #[inline(always)]
    pub fn p3ies7(&mut self) -> P3IES7_W {
        P3IES7_W { w: self }
    }
}
