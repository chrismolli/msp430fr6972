#[doc = "Register `LCDM27` reader"]
pub struct R(crate::R<LCDM27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDM27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDM27_SPEC>> for R {
    fn from(reader: crate::R<LCDM27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDM27` writer"]
pub struct W(crate::W<LCDM27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDM27_SPEC>;
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
impl core::convert::From<crate::W<LCDM27_SPEC>> for W {
    fn from(writer: crate::W<LCDM27_SPEC>) -> Self {
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
#[doc = "LCD Memory 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm27](index.html) module"]
pub struct LCDM27_SPEC;
impl crate::RegisterSpec for LCDM27_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdm27::R](R) reader structure"]
impl crate::Readable for LCDM27_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdm27::W](W) writer structure"]
impl crate::Writable for LCDM27_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDM27 to value 0"]
impl crate::Resettable for LCDM27_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
