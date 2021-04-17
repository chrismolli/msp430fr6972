#[doc = "Reader of register LCDCCPCTL"]
pub type R = crate::R<u16, super::LCDCCPCTL>;
#[doc = "Writer for register LCDCCPCTL"]
pub type W = crate::W<u16, super::LCDCCPCTL>;
#[doc = "Register LCDCCPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCCPCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDCPDIS0`"]
pub type LCDCPDIS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS0`"]
pub struct LCDCPDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS0_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS1`"]
pub type LCDCPDIS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS1`"]
pub struct LCDCPDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS1_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS2`"]
pub type LCDCPDIS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS2`"]
pub struct LCDCPDIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS2_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS3`"]
pub type LCDCPDIS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS3`"]
pub struct LCDCPDIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS3_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS4`"]
pub type LCDCPDIS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS4`"]
pub struct LCDCPDIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS4_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS5`"]
pub type LCDCPDIS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS5`"]
pub struct LCDCPDIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS5_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS6`"]
pub type LCDCPDIS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS6`"]
pub struct LCDCPDIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS6_W<'a> {
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
#[doc = "Reader of field `LCDCPDIS7`"]
pub type LCDCPDIS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPDIS7`"]
pub struct LCDCPDIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS7_W<'a> {
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
#[doc = "Reader of field `LCDCPCLKSYNC`"]
pub type LCDCPCLKSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDCPCLKSYNC`"]
pub struct LCDCPCLKSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPCLKSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&self) -> LCDCPDIS0_R {
        LCDCPDIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&self) -> LCDCPDIS1_R {
        LCDCPDIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&self) -> LCDCPDIS2_R {
        LCDCPDIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&self) -> LCDCPDIS3_R {
        LCDCPDIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&self) -> LCDCPDIS4_R {
        LCDCPDIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&self) -> LCDCPDIS5_R {
        LCDCPDIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&self) -> LCDCPDIS6_R {
        LCDCPDIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&self) -> LCDCPDIS7_R {
        LCDCPDIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&self) -> LCDCPCLKSYNC_R {
        LCDCPCLKSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&mut self) -> LCDCPDIS0_W {
        LCDCPDIS0_W { w: self }
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&mut self) -> LCDCPDIS1_W {
        LCDCPDIS1_W { w: self }
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&mut self) -> LCDCPDIS2_W {
        LCDCPDIS2_W { w: self }
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&mut self) -> LCDCPDIS3_W {
        LCDCPDIS3_W { w: self }
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&mut self) -> LCDCPDIS4_W {
        LCDCPDIS4_W { w: self }
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&mut self) -> LCDCPDIS5_W {
        LCDCPDIS5_W { w: self }
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&mut self) -> LCDCPDIS6_W {
        LCDCPDIS6_W { w: self }
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&mut self) -> LCDCPDIS7_W {
        LCDCPDIS7_W { w: self }
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&mut self) -> LCDCPCLKSYNC_W {
        LCDCPCLKSYNC_W { w: self }
    }
}
