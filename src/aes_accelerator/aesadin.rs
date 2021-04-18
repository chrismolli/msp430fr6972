#[doc = "Register `AESADIN` reader"]
pub struct R(crate::R<AESADIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESADIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AESADIN_SPEC>> for R {
    fn from(reader: crate::R<AESADIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESADIN` writer"]
pub struct W(crate::W<AESADIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESADIN_SPEC>;
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
impl core::convert::From<crate::W<AESADIN_SPEC>> for W {
    fn from(writer: crate::W<AESADIN_SPEC>) -> Self {
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
#[doc = "AES accelerator data in register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadin](index.html) module"]
pub struct AESADIN_SPEC;
impl crate::RegisterSpec for AESADIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesadin::R](R) reader structure"]
impl crate::Readable for AESADIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesadin::W](W) writer structure"]
impl crate::Writable for AESADIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESADIN to value 0"]
impl crate::Resettable for AESADIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
