#[doc = "Register `AESAXIN` reader"]
pub struct R(crate::R<AESAXIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESAXIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AESAXIN_SPEC>> for R {
    fn from(reader: crate::R<AESAXIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESAXIN` writer"]
pub struct W(crate::W<AESAXIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAXIN_SPEC>;
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
impl core::convert::From<crate::W<AESAXIN_SPEC>> for W {
    fn from(writer: crate::W<AESAXIN_SPEC>) -> Self {
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
#[doc = "AES accelerator XORed data in register (no trigger)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxin](index.html) module"]
pub struct AESAXIN_SPEC;
impl crate::RegisterSpec for AESAXIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesaxin::R](R) reader structure"]
impl crate::Readable for AESAXIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesaxin::W](W) writer structure"]
impl crate::Writable for AESAXIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAXIN to value 0"]
impl crate::Resettable for AESAXIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
