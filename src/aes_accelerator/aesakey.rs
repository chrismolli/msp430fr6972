#[doc = "Register `AESAKEY` reader"]
pub struct R(crate::R<AESAKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESAKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AESAKEY_SPEC>> for R {
    fn from(reader: crate::R<AESAKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESAKEY` writer"]
pub struct W(crate::W<AESAKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAKEY_SPEC>;
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
impl core::convert::From<crate::W<AESAKEY_SPEC>> for W {
    fn from(writer: crate::W<AESAKEY_SPEC>) -> Self {
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
#[doc = "AES accelerator key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesakey](index.html) module"]
pub struct AESAKEY_SPEC;
impl crate::RegisterSpec for AESAKEY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesakey::R](R) reader structure"]
impl crate::Readable for AESAKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesakey::W](W) writer structure"]
impl crate::Writable for AESAKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAKEY to value 0"]
impl crate::Resettable for AESAKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
