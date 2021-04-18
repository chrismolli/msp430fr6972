#[doc = "Register `P4IV` reader"]
pub struct R(crate::R<P4IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P4IV_SPEC>> for R {
    fn from(reader: crate::R<P4IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4IV` writer"]
pub struct W(crate::W<P4IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4IV_SPEC>;
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
impl core::convert::From<crate::W<P4IV_SPEC>> for W {
    fn from(writer: crate::W<P4IV_SPEC>) -> Self {
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
#[doc = "Port 4 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4iv](index.html) module"]
pub struct P4IV_SPEC;
impl crate::RegisterSpec for P4IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p4iv::R](R) reader structure"]
impl crate::Readable for P4IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4iv::W](W) writer structure"]
impl crate::Writable for P4IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4IV to value 0"]
impl crate::Resettable for P4IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
