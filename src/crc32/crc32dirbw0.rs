#[doc = "Register `CRC32DIRBW0` reader"]
pub struct R(crate::R<CRC32DIRBW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32DIRBW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC32DIRBW0_SPEC>> for R {
    fn from(reader: crate::R<CRC32DIRBW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32DIRBW0` writer"]
pub struct W(crate::W<CRC32DIRBW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32DIRBW0_SPEC>;
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
impl core::convert::From<crate::W<CRC32DIRBW0_SPEC>> for W {
    fn from(writer: crate::W<CRC32DIRBW0_SPEC>) -> Self {
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
#[doc = "CRC32 Data In Reversed Bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dirbw0](index.html) module"]
pub struct CRC32DIRBW0_SPEC;
impl crate::RegisterSpec for CRC32DIRBW0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc32dirbw0::R](R) reader structure"]
impl crate::Readable for CRC32DIRBW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32dirbw0::W](W) writer structure"]
impl crate::Writable for CRC32DIRBW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32DIRBW0 to value 0"]
impl crate::Resettable for CRC32DIRBW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
