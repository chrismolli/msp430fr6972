#[doc = "Register `CSCTL0` reader"]
pub struct R(crate::R<CSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSCTL0_SPEC>> for R {
    fn from(reader: crate::R<CSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL0` writer"]
pub struct W(crate::W<CSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL0_SPEC>;
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
impl core::convert::From<crate::W<CSCTL0_SPEC>> for W {
    fn from(writer: crate::W<CSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl0](index.html) module"]
pub struct CSCTL0_SPEC;
impl crate::RegisterSpec for CSCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl0::R](R) reader structure"]
impl crate::Readable for CSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl0::W](W) writer structure"]
impl crate::Writable for CSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for CSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
