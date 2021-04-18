#[doc = "Register `MAC` reader"]
pub struct R(crate::R<MAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_SPEC>> for R {
    fn from(reader: crate::R<MAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC` writer"]
pub struct W(crate::W<MAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SPEC>;
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
impl core::convert::From<crate::W<MAC_SPEC>> for W {
    fn from(writer: crate::W<MAC_SPEC>) -> Self {
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
#[doc = "Multiply Unsigned and Accumulate/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac](index.html) module"]
pub struct MAC_SPEC;
impl crate::RegisterSpec for MAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mac::R](R) reader structure"]
impl crate::Readable for MAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac::W](W) writer structure"]
impl crate::Writable for MAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC to value 0"]
impl crate::Resettable for MAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
