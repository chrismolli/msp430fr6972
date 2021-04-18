#[doc = "Register `DMA2SA` reader"]
pub struct R(crate::R<DMA2SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA2SA_SPEC>> for R {
    fn from(reader: crate::R<DMA2SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2SA` writer"]
pub struct W(crate::W<DMA2SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2SA_SPEC>;
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
impl core::convert::From<crate::W<DMA2SA_SPEC>> for W {
    fn from(writer: crate::W<DMA2SA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 2 Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2sa](index.html) module"]
pub struct DMA2SA_SPEC;
impl crate::RegisterSpec for DMA2SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2sa::R](R) reader structure"]
impl crate::Readable for DMA2SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2sa::W](W) writer structure"]
impl crate::Writable for DMA2SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2SA to value 0"]
impl crate::Resettable for DMA2SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
