#[doc = "Reader of register MPUCTL0"]
pub type R = crate::R<u16, super::MPUCTL0>;
#[doc = "Writer for register MPUCTL0"]
pub type W = crate::W<u16, super::MPUCTL0>;
#[doc = "Register MPUCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUENA`"]
pub type MPUENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUENA`"]
pub struct MPUENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUENA_W<'a> {
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
#[doc = "Reader of field `MPULOCK`"]
pub type MPULOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPULOCK`"]
pub struct MPULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MPULOCK_W<'a> {
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
#[doc = "Reader of field `MPUSEGIE`"]
pub type MPUSEGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIE`"]
pub struct MPUSEGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIE_W<'a> {
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
    #[doc = "Bit 0 - MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&self) -> MPUENA_R {
        MPUENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&self) -> MPULOCK_R {
        MPULOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Enable NMI on Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&self) -> MPUSEGIE_R {
        MPUSEGIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Enable"]
    #[inline(always)]
    pub fn mpuena(&mut self) -> MPUENA_W {
        MPUENA_W { w: self }
    }
    #[doc = "Bit 1 - MPU Lock"]
    #[inline(always)]
    pub fn mpulock(&mut self) -> MPULOCK_W {
        MPULOCK_W { w: self }
    }
    #[doc = "Bit 4 - MPU Enable NMI on Segment violation"]
    #[inline(always)]
    pub fn mpusegie(&mut self) -> MPUSEGIE_W {
        MPUSEGIE_W { w: self }
    }
}
