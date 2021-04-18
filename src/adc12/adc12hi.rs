#[doc = "Register `ADC12HI` reader"]
pub struct R(crate::R<ADC12HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC12HI_SPEC>> for R {
    fn from(reader: crate::R<ADC12HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12HI` writer"]
pub struct W(crate::W<ADC12HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12HI_SPEC>;
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
impl core::convert::From<crate::W<ADC12HI_SPEC>> for W {
    fn from(writer: crate::W<ADC12HI_SPEC>) -> Self {
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
#[doc = "ADC12 B Window Comparator High Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12hi](index.html) module"]
pub struct ADC12HI_SPEC;
impl crate::RegisterSpec for ADC12HI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12hi::R](R) reader structure"]
impl crate::Readable for ADC12HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12hi::W](W) writer structure"]
impl crate::Writable for ADC12HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12HI to value 0"]
impl crate::Resettable for ADC12HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
