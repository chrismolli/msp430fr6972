#[doc = "Register `LCDBM10` reader"]
pub struct R(crate::R<LCDBM10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDBM10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDBM10_SPEC>> for R {
    fn from(reader: crate::R<LCDBM10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDBM10` writer"]
pub struct W(crate::W<LCDBM10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDBM10_SPEC>;
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
impl core::convert::From<crate::W<LCDBM10_SPEC>> for W {
    fn from(writer: crate::W<LCDBM10_SPEC>) -> Self {
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
#[doc = "LCD Blinking Memory 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm10](index.html) module"]
pub struct LCDBM10_SPEC;
impl crate::RegisterSpec for LCDBM10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdbm10::R](R) reader structure"]
impl crate::Readable for LCDBM10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdbm10::W](W) writer structure"]
impl crate::Writable for LCDBM10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDBM10 to value 0"]
impl crate::Resettable for LCDBM10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
