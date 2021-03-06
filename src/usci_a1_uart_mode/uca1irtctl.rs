#[doc = "Register `UCA1IRTCTL` reader"]
pub struct R(crate::R<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCA1IRTCTL_SPEC>> for R {
    fn from(reader: crate::R<UCA1IRTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRTCTL` writer"]
pub struct W(crate::W<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRTCTL_SPEC>;
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
impl core::convert::From<crate::W<UCA1IRTCTL_SPEC>> for W {
    fn from(writer: crate::W<UCA1IRTCTL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irtctl](index.html) module"]
pub struct UCA1IRTCTL_SPEC;
impl crate::RegisterSpec for UCA1IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1irtctl::R](R) reader structure"]
impl crate::Readable for UCA1IRTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irtctl::W](W) writer structure"]
impl crate::Writable for UCA1IRTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1IRTCTL to value 0"]
impl crate::Resettable for UCA1IRTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
