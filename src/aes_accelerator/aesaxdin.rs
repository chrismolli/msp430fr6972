#[doc = "Register `AESAXDIN` reader"]
pub struct R(crate::R<AESAXDIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESAXDIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AESAXDIN_SPEC>> for R {
    fn from(reader: crate::R<AESAXDIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESAXDIN` writer"]
pub struct W(crate::W<AESAXDIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAXDIN_SPEC>;
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
impl core::convert::From<crate::W<AESAXDIN_SPEC>> for W {
    fn from(writer: crate::W<AESAXDIN_SPEC>) -> Self {
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
#[doc = "AES accelerator XORed data in register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxdin](index.html) module"]
pub struct AESAXDIN_SPEC;
impl crate::RegisterSpec for AESAXDIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesaxdin::R](R) reader structure"]
impl crate::Readable for AESAXDIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesaxdin::W](W) writer structure"]
impl crate::Writable for AESAXDIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAXDIN to value 0"]
impl crate::Resettable for AESAXDIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
