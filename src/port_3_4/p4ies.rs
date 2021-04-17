#[doc = "Reader of register P4IES"]
pub type R = crate::R<u8, super::P4IES>;
#[doc = "Writer for register P4IES"]
pub type W = crate::W<u8, super::P4IES>;
#[doc = "Register P4IES `reset()`'s with value 0"]
impl crate::ResetValue for super::P4IES {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P4IES0`"]
pub type P4IES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES0`"]
pub struct P4IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES0_W<'a> {
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
#[doc = "Reader of field `P4IES1`"]
pub type P4IES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES1`"]
pub struct P4IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES1_W<'a> {
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
#[doc = "Reader of field `P4IES2`"]
pub type P4IES2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES2`"]
pub struct P4IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES2_W<'a> {
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
#[doc = "Reader of field `P4IES3`"]
pub type P4IES3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES3`"]
pub struct P4IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES3_W<'a> {
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
#[doc = "Reader of field `P4IES4`"]
pub type P4IES4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES4`"]
pub struct P4IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES4_W<'a> {
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
#[doc = "Reader of field `P4IES5`"]
pub type P4IES5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES5`"]
pub struct P4IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES5_W<'a> {
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
#[doc = "Reader of field `P4IES6`"]
pub type P4IES6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES6`"]
pub struct P4IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES6_W<'a> {
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
#[doc = "Reader of field `P4IES7`"]
pub type P4IES7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4IES7`"]
pub struct P4IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES7_W<'a> {
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
    #[doc = "Bit 0 - P4IES0"]
    #[inline(always)]
    pub fn p4ies0(&self) -> P4IES0_R {
        P4IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IES1"]
    #[inline(always)]
    pub fn p4ies1(&self) -> P4IES1_R {
        P4IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IES2"]
    #[inline(always)]
    pub fn p4ies2(&self) -> P4IES2_R {
        P4IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IES3"]
    #[inline(always)]
    pub fn p4ies3(&self) -> P4IES3_R {
        P4IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IES4"]
    #[inline(always)]
    pub fn p4ies4(&self) -> P4IES4_R {
        P4IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IES5"]
    #[inline(always)]
    pub fn p4ies5(&self) -> P4IES5_R {
        P4IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IES6"]
    #[inline(always)]
    pub fn p4ies6(&self) -> P4IES6_R {
        P4IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IES7"]
    #[inline(always)]
    pub fn p4ies7(&self) -> P4IES7_R {
        P4IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IES0"]
    #[inline(always)]
    pub fn p4ies0(&mut self) -> P4IES0_W {
        P4IES0_W { w: self }
    }
    #[doc = "Bit 1 - P4IES1"]
    #[inline(always)]
    pub fn p4ies1(&mut self) -> P4IES1_W {
        P4IES1_W { w: self }
    }
    #[doc = "Bit 2 - P4IES2"]
    #[inline(always)]
    pub fn p4ies2(&mut self) -> P4IES2_W {
        P4IES2_W { w: self }
    }
    #[doc = "Bit 3 - P4IES3"]
    #[inline(always)]
    pub fn p4ies3(&mut self) -> P4IES3_W {
        P4IES3_W { w: self }
    }
    #[doc = "Bit 4 - P4IES4"]
    #[inline(always)]
    pub fn p4ies4(&mut self) -> P4IES4_W {
        P4IES4_W { w: self }
    }
    #[doc = "Bit 5 - P4IES5"]
    #[inline(always)]
    pub fn p4ies5(&mut self) -> P4IES5_W {
        P4IES5_W { w: self }
    }
    #[doc = "Bit 6 - P4IES6"]
    #[inline(always)]
    pub fn p4ies6(&mut self) -> P4IES6_W {
        P4IES6_W { w: self }
    }
    #[doc = "Bit 7 - P4IES7"]
    #[inline(always)]
    pub fn p4ies7(&mut self) -> P4IES7_W {
        P4IES7_W { w: self }
    }
}
