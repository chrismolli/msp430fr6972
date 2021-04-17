#[doc = "Reader of register MPUCTL1"]
pub type R = crate::R<u16, super::MPUCTL1>;
#[doc = "Writer for register MPUCTL1"]
pub type W = crate::W<u16, super::MPUCTL1>;
#[doc = "Register MPUCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUSEG1IFG`"]
pub type MPUSEG1IFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG1IFG`"]
pub struct MPUSEG1IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1IFG_W<'a> {
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
#[doc = "Reader of field `MPUSEG2IFG`"]
pub type MPUSEG2IFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG2IFG`"]
pub struct MPUSEG2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2IFG_W<'a> {
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
#[doc = "Reader of field `MPUSEG3IFG`"]
pub type MPUSEG3IFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG3IFG`"]
pub struct MPUSEG3IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3IFG_W<'a> {
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
#[doc = "Reader of field `MPUSEGIIFG`"]
pub type MPUSEGIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIIFG`"]
pub struct MPUSEGIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIIFG_W<'a> {
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
#[doc = "Reader of field `MPUSEGIPIFG`"]
pub type MPUSEGIPIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIPIFG`"]
pub struct MPUSEGIPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIPIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&self) -> MPUSEG1IFG_R {
        MPUSEG1IFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 2 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&self) -> MPUSEG2IFG_R {
        MPUSEG2IFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 3 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&self) -> MPUSEG3IFG_R {
        MPUSEG3IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Info Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegiifg(&self) -> MPUSEGIIFG_R {
        MPUSEGIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU IP Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegipifg(&self) -> MPUSEGIPIFG_R {
        MPUSEGIPIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&mut self) -> MPUSEG1IFG_W {
        MPUSEG1IFG_W { w: self }
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 2 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&mut self) -> MPUSEG2IFG_W {
        MPUSEG2IFG_W { w: self }
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 3 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&mut self) -> MPUSEG3IFG_W {
        MPUSEG3IFG_W { w: self }
    }
    #[doc = "Bit 3 - MPU Info Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegiifg(&mut self) -> MPUSEGIIFG_W {
        MPUSEGIIFG_W { w: self }
    }
    #[doc = "Bit 4 - MPU IP Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegipifg(&mut self) -> MPUSEGIPIFG_W {
        MPUSEGIPIFG_W { w: self }
    }
}
