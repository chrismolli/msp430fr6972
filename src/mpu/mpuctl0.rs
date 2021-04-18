#[doc = "Register `MPUCTL0` reader"]
pub struct R(crate::R<MPUCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUCTL0_SPEC>> for R {
    fn from(reader: crate::R<MPUCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUCTL0` writer"]
pub struct W(crate::W<MPUCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUCTL0_SPEC>;
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
impl core::convert::From<crate::W<MPUCTL0_SPEC>> for W {
    fn from(writer: crate::W<MPUCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUENA` reader - MPU Enable"]
pub struct MPUENA_R(crate::FieldReader<bool, bool>);
impl MPUENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUENA` writer - MPU Enable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `MPULOCK` reader - MPU Lock"]
pub struct MPULOCK_R(crate::FieldReader<bool, bool>);
impl MPULOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPULOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPULOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPULOCK` writer - MPU Lock"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MPUSEGIE` reader - MPU Enable NMI on Segment violation"]
pub struct MPUSEGIE_R(crate::FieldReader<bool, bool>);
impl MPUSEGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIE` writer - MPU Enable NMI on Segment violation"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuctl0](index.html) module"]
pub struct MPUCTL0_SPEC;
impl crate::RegisterSpec for MPUCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpuctl0::R](R) reader structure"]
impl crate::Readable for MPUCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpuctl0::W](W) writer structure"]
impl crate::Writable for MPUCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUCTL0 to value 0"]
impl crate::Resettable for MPUCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
