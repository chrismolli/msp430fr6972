#[doc = "Register `PMMCTL0` reader"]
pub struct R(crate::R<PMMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMMCTL0_SPEC>> for R {
    fn from(reader: crate::R<PMMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL0` writer"]
pub struct W(crate::W<PMMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL0_SPEC>;
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
impl core::convert::From<crate::W<PMMCTL0_SPEC>> for W {
    fn from(writer: crate::W<PMMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub struct PMMSWBOR_R(crate::FieldReader<bool, bool>);
impl PMMSWBOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMSWBOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMSWBOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub struct PMMSWBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWBOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub struct PMMSWPOR_R(crate::FieldReader<bool, bool>);
impl PMMSWPOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMSWPOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMSWPOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub struct PMMSWPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWPOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub struct PMMREGOFF_R(crate::FieldReader<bool, bool>);
impl PMMREGOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMREGOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMREGOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub struct PMMREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREGOFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub struct SVSHE_R(crate::FieldReader<bool, bool>);
impl SVSHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub struct SVSHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PMMLPRST` reader - PMM Low-Power Reset Enable"]
pub struct PMMLPRST_R(crate::FieldReader<bool, bool>);
impl PMMLPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMLPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMLPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMLPRST` writer - PMM Low-Power Reset Enable"]
pub struct PMMLPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMLPRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMM Low-Power Reset Enable"]
    #[inline(always)]
    pub fn pmmlprst(&self) -> PMMLPRST_R {
        PMMLPRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W {
        PMMSWBOR_W { w: self }
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W {
        PMMSWPOR_W { w: self }
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W {
        PMMREGOFF_W { w: self }
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W {
        SVSHE_W { w: self }
    }
    #[doc = "Bit 7 - PMM Low-Power Reset Enable"]
    #[inline(always)]
    pub fn pmmlprst(&mut self) -> PMMLPRST_W {
        PMMLPRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](index.html) module"]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl0::R](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
