#[doc = "Reader of register PJSEL0"]
pub type R = crate::R<u16, super::PJSEL0>;
#[doc = "Writer for register PJSEL0"]
pub type W = crate::W<u16, super::PJSEL0>;
#[doc = "Register PJSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PJSEL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJSEL0_0`"]
pub type PJSEL0_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_0`"]
pub struct PJSEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_1`"]
pub type PJSEL0_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_1`"]
pub struct PJSEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_2`"]
pub type PJSEL0_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_2`"]
pub struct PJSEL0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_3`"]
pub type PJSEL0_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_3`"]
pub struct PJSEL0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_4`"]
pub type PJSEL0_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_4`"]
pub struct PJSEL0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_5`"]
pub type PJSEL0_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_5`"]
pub struct PJSEL0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_6`"]
pub type PJSEL0_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_6`"]
pub struct PJSEL0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PJSEL0_7`"]
pub type PJSEL0_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL0_7`"]
pub struct PJSEL0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PJSEL0_0"]
    #[inline(always)]
    pub fn pjsel0_0(&self) -> PJSEL0_0_R {
        PJSEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJSEL0_1"]
    #[inline(always)]
    pub fn pjsel0_1(&self) -> PJSEL0_1_R {
        PJSEL0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJSEL0_2"]
    #[inline(always)]
    pub fn pjsel0_2(&self) -> PJSEL0_2_R {
        PJSEL0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJSEL0_3"]
    #[inline(always)]
    pub fn pjsel0_3(&self) -> PJSEL0_3_R {
        PJSEL0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJSEL0_4"]
    #[inline(always)]
    pub fn pjsel0_4(&self) -> PJSEL0_4_R {
        PJSEL0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJSEL0_5"]
    #[inline(always)]
    pub fn pjsel0_5(&self) -> PJSEL0_5_R {
        PJSEL0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJSEL0_6"]
    #[inline(always)]
    pub fn pjsel0_6(&self) -> PJSEL0_6_R {
        PJSEL0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJSEL0_7"]
    #[inline(always)]
    pub fn pjsel0_7(&self) -> PJSEL0_7_R {
        PJSEL0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJSEL0_0"]
    #[inline(always)]
    pub fn pjsel0_0(&mut self) -> PJSEL0_0_W {
        PJSEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - PJSEL0_1"]
    #[inline(always)]
    pub fn pjsel0_1(&mut self) -> PJSEL0_1_W {
        PJSEL0_1_W { w: self }
    }
    #[doc = "Bit 2 - PJSEL0_2"]
    #[inline(always)]
    pub fn pjsel0_2(&mut self) -> PJSEL0_2_W {
        PJSEL0_2_W { w: self }
    }
    #[doc = "Bit 3 - PJSEL0_3"]
    #[inline(always)]
    pub fn pjsel0_3(&mut self) -> PJSEL0_3_W {
        PJSEL0_3_W { w: self }
    }
    #[doc = "Bit 4 - PJSEL0_4"]
    #[inline(always)]
    pub fn pjsel0_4(&mut self) -> PJSEL0_4_W {
        PJSEL0_4_W { w: self }
    }
    #[doc = "Bit 5 - PJSEL0_5"]
    #[inline(always)]
    pub fn pjsel0_5(&mut self) -> PJSEL0_5_W {
        PJSEL0_5_W { w: self }
    }
    #[doc = "Bit 6 - PJSEL0_6"]
    #[inline(always)]
    pub fn pjsel0_6(&mut self) -> PJSEL0_6_W {
        PJSEL0_6_W { w: self }
    }
    #[doc = "Bit 7 - PJSEL0_7"]
    #[inline(always)]
    pub fn pjsel0_7(&mut self) -> PJSEL0_7_W {
        PJSEL0_7_W { w: self }
    }
}
