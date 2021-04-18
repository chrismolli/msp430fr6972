#[doc = "Register `MACS32H` reader"]
pub struct R(crate::R<MACS32H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACS32H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MACS32H_SPEC>> for R {
    fn from(reader: crate::R<MACS32H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACS32H` writer"]
pub struct W(crate::W<MACS32H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACS32H_SPEC>;
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
impl core::convert::From<crate::W<MACS32H_SPEC>> for W {
    fn from(writer: crate::W<MACS32H_SPEC>) -> Self {
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
#[doc = "32-bit operand 1 - signed multiply accumulate - high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs32h](index.html) module"]
pub struct MACS32H_SPEC;
impl crate::RegisterSpec for MACS32H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [macs32h::R](R) reader structure"]
impl crate::Readable for MACS32H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macs32h::W](W) writer structure"]
impl crate::Writable for MACS32H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACS32H to value 0"]
impl crate::Resettable for MACS32H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
