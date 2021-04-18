#[doc = "Register `MPUIPC0` reader"]
pub struct R(crate::R<MPUIPC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUIPC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUIPC0_SPEC>> for R {
    fn from(reader: crate::R<MPUIPC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUIPC0` writer"]
pub struct W(crate::W<MPUIPC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUIPC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<MPUIPC0_SPEC>> for W {
    fn from(writer: crate::W<MPUIPC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUIPVS` reader - MPU MPU IP protection segment Violation Select"]
pub struct MPUIPVS_R(crate::FieldReader<bool, bool>);
impl MPUIPVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPVS` writer - MPU MPU IP protection segment Violation Select"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MPUIPENA` reader - MPU MPU IP Protection Enable"]
pub struct MPUIPENA_R(crate::FieldReader<bool, bool>);
impl MPUIPENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPENA` writer - MPU MPU IP Protection Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MPUIPLOCK` reader - MPU IP Protection Lock"]
pub struct MPUIPLOCK_R(crate::FieldReader<bool, bool>);
impl MPUIPLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPLOCK` writer - MPU IP Protection Lock"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU IP Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuipc0](index.html) module"]
pub struct MPUIPC0_SPEC;
impl crate::RegisterSpec for MPUIPC0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpuipc0::R](R) reader structure"]
impl crate::Readable for MPUIPC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpuipc0::W](W) writer structure"]
impl crate::Writable for MPUIPC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUIPC0 to value 0"]
impl crate::Resettable for MPUIPC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
