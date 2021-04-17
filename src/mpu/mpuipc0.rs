#[doc = "Reader of register MPUIPC0"]
pub type R = crate::R<u16, super::MPUIPC0>;
#[doc = "Writer for register MPUIPC0"]
pub type W = crate::W<u16, super::MPUIPC0>;
#[doc = "Register MPUIPC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUIPC0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUIPVS`"]
pub type MPUIPVS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPVS`"]
pub struct MPUIPVS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPVS_W<'a> {
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
#[doc = "Reader of field `MPUIPENA`"]
pub type MPUIPENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPENA`"]
pub struct MPUIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPENA_W<'a> {
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
#[doc = "Reader of field `MPUIPLOCK`"]
pub type MPUIPLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUIPLOCK`"]
pub struct MPUIPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPLOCK_W<'a> {
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
    #[doc = "Bit 5 - MPU MPU IP protection segment Violation Select"]
    #[inline(always)]
    pub fn mpuipvs(&self) -> MPUIPVS_R {
        MPUIPVS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU MPU IP Protection Enable"]
    #[inline(always)]
    pub fn mpuipena(&self) -> MPUIPENA_R {
        MPUIPENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU IP Protection Lock"]
    #[inline(always)]
    pub fn mpuiplock(&self) -> MPUIPLOCK_R {
        MPUIPLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - MPU MPU IP protection segment Violation Select"]
    #[inline(always)]
    pub fn mpuipvs(&mut self) -> MPUIPVS_W {
        MPUIPVS_W { w: self }
    }
    #[doc = "Bit 6 - MPU MPU IP Protection Enable"]
    #[inline(always)]
    pub fn mpuipena(&mut self) -> MPUIPENA_W {
        MPUIPENA_W { w: self }
    }
    #[doc = "Bit 7 - MPU IP Protection Lock"]
    #[inline(always)]
    pub fn mpuiplock(&mut self) -> MPUIPLOCK_W {
        MPUIPLOCK_W { w: self }
    }
}
