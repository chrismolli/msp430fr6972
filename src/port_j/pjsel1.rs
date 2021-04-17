#[doc = "Reader of register PJSEL1"]
pub type R = crate::R<u16, super::PJSEL1>;
#[doc = "Writer for register PJSEL1"]
pub type W = crate::W<u16, super::PJSEL1>;
#[doc = "Register PJSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PJSEL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJSEL1_0`"]
pub type PJSEL1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_0`"]
pub struct PJSEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_0_W<'a> {
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
#[doc = "Reader of field `PJSEL1_1`"]
pub type PJSEL1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_1`"]
pub struct PJSEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_1_W<'a> {
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
#[doc = "Reader of field `PJSEL1_2`"]
pub type PJSEL1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_2`"]
pub struct PJSEL1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_2_W<'a> {
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
#[doc = "Reader of field `PJSEL1_3`"]
pub type PJSEL1_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_3`"]
pub struct PJSEL1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_3_W<'a> {
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
#[doc = "Reader of field `PJSEL1_4`"]
pub type PJSEL1_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_4`"]
pub struct PJSEL1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_4_W<'a> {
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
#[doc = "Reader of field `PJSEL1_5`"]
pub type PJSEL1_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_5`"]
pub struct PJSEL1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_5_W<'a> {
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
#[doc = "Reader of field `PJSEL1_6`"]
pub type PJSEL1_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_6`"]
pub struct PJSEL1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_6_W<'a> {
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
#[doc = "Reader of field `PJSEL1_7`"]
pub type PJSEL1_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJSEL1_7`"]
pub struct PJSEL1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_7_W<'a> {
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
    #[doc = "Bit 0 - PJSEL1_0"]
    #[inline(always)]
    pub fn pjsel1_0(&self) -> PJSEL1_0_R {
        PJSEL1_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJSEL1_1"]
    #[inline(always)]
    pub fn pjsel1_1(&self) -> PJSEL1_1_R {
        PJSEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJSEL1_2"]
    #[inline(always)]
    pub fn pjsel1_2(&self) -> PJSEL1_2_R {
        PJSEL1_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJSEL1_3"]
    #[inline(always)]
    pub fn pjsel1_3(&self) -> PJSEL1_3_R {
        PJSEL1_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJSEL1_4"]
    #[inline(always)]
    pub fn pjsel1_4(&self) -> PJSEL1_4_R {
        PJSEL1_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJSEL1_5"]
    #[inline(always)]
    pub fn pjsel1_5(&self) -> PJSEL1_5_R {
        PJSEL1_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJSEL1_6"]
    #[inline(always)]
    pub fn pjsel1_6(&self) -> PJSEL1_6_R {
        PJSEL1_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJSEL1_7"]
    #[inline(always)]
    pub fn pjsel1_7(&self) -> PJSEL1_7_R {
        PJSEL1_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJSEL1_0"]
    #[inline(always)]
    pub fn pjsel1_0(&mut self) -> PJSEL1_0_W {
        PJSEL1_0_W { w: self }
    }
    #[doc = "Bit 1 - PJSEL1_1"]
    #[inline(always)]
    pub fn pjsel1_1(&mut self) -> PJSEL1_1_W {
        PJSEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - PJSEL1_2"]
    #[inline(always)]
    pub fn pjsel1_2(&mut self) -> PJSEL1_2_W {
        PJSEL1_2_W { w: self }
    }
    #[doc = "Bit 3 - PJSEL1_3"]
    #[inline(always)]
    pub fn pjsel1_3(&mut self) -> PJSEL1_3_W {
        PJSEL1_3_W { w: self }
    }
    #[doc = "Bit 4 - PJSEL1_4"]
    #[inline(always)]
    pub fn pjsel1_4(&mut self) -> PJSEL1_4_W {
        PJSEL1_4_W { w: self }
    }
    #[doc = "Bit 5 - PJSEL1_5"]
    #[inline(always)]
    pub fn pjsel1_5(&mut self) -> PJSEL1_5_W {
        PJSEL1_5_W { w: self }
    }
    #[doc = "Bit 6 - PJSEL1_6"]
    #[inline(always)]
    pub fn pjsel1_6(&mut self) -> PJSEL1_6_W {
        PJSEL1_6_W { w: self }
    }
    #[doc = "Bit 7 - PJSEL1_7"]
    #[inline(always)]
    pub fn pjsel1_7(&mut self) -> PJSEL1_7_W {
        PJSEL1_7_W { w: self }
    }
}
