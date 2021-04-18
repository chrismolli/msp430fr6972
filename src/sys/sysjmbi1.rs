#[doc = "Register `SYSJMBI1` reader"]
pub struct R(crate::R<SYSJMBI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSJMBI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYSJMBI1_SPEC>> for R {
    fn from(reader: crate::R<SYSJMBI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSJMBI1` writer"]
pub struct W(crate::W<SYSJMBI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSJMBI1_SPEC>;
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
impl core::convert::From<crate::W<SYSJMBI1_SPEC>> for W {
    fn from(writer: crate::W<SYSJMBI1_SPEC>) -> Self {
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
#[doc = "JTAG mailbox input 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbi1](index.html) module"]
pub struct SYSJMBI1_SPEC;
impl crate::RegisterSpec for SYSJMBI1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysjmbi1::R](R) reader structure"]
impl crate::Readable for SYSJMBI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysjmbi1::W](W) writer structure"]
impl crate::Writable for SYSJMBI1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSJMBI1 to value 0"]
impl crate::Resettable for SYSJMBI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
