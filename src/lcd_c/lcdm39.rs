#[doc = "Register `LCDM39` reader"]
pub struct R(crate::R<LCDM39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDM39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDM39_SPEC>> for R {
    fn from(reader: crate::R<LCDM39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDM39` writer"]
pub struct W(crate::W<LCDM39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDM39_SPEC>;
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
impl core::convert::From<crate::W<LCDM39_SPEC>> for W {
    fn from(writer: crate::W<LCDM39_SPEC>) -> Self {
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
#[doc = "LCD Memory 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm39](index.html) module"]
pub struct LCDM39_SPEC;
impl crate::RegisterSpec for LCDM39_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdm39::R](R) reader structure"]
impl crate::Readable for LCDM39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdm39::W](W) writer structure"]
impl crate::Writable for LCDM39_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDM39 to value 0"]
impl crate::Resettable for LCDM39_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
