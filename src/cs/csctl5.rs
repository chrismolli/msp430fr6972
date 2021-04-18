#[doc = "Register `CSCTL5` reader"]
pub struct R(crate::R<CSCTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSCTL5_SPEC>> for R {
    fn from(reader: crate::R<CSCTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL5` writer"]
pub struct W(crate::W<CSCTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL5_SPEC>;
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
impl core::convert::From<crate::W<CSCTL5_SPEC>> for W {
    fn from(writer: crate::W<CSCTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFXTOFFG` reader - LFXT Low Frequency Oscillator Fault Flag"]
pub struct LFXTOFFG_R(crate::FieldReader<bool, bool>);
impl LFXTOFFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXTOFFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXTOFFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXTOFFG` writer - LFXT Low Frequency Oscillator Fault Flag"]
pub struct LFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `HFXTOFFG` reader - HFXT High Frequency Oscillator Fault Flag"]
pub struct HFXTOFFG_R(crate::FieldReader<bool, bool>);
impl HFXTOFFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXTOFFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXTOFFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXTOFFG` writer - HFXT High Frequency Oscillator Fault Flag"]
pub struct HFXTOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for XT1"]
pub struct ENSTFCNT1_R(crate::FieldReader<bool, bool>);
impl ENSTFCNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSTFCNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSTFCNT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for XT1"]
pub struct ENSTFCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT1_W<'a> {
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
#[doc = "Field `ENSTFCNT2` reader - Enable start counter for XT2"]
pub struct ENSTFCNT2_R(crate::FieldReader<bool, bool>);
impl ENSTFCNT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENSTFCNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSTFCNT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSTFCNT2` writer - Enable start counter for XT2"]
pub struct ENSTFCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSTFCNT2_W<'a> {
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
    #[doc = "Bit 0 - LFXT Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxtoffg(&self) -> LFXTOFFG_R {
        LFXTOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXT High Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn hfxtoffg(&self) -> HFXTOFFG_R {
        HFXTOFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable start counter for XT2"]
    #[inline(always)]
    pub fn enstfcnt2(&self) -> ENSTFCNT2_R {
        ENSTFCNT2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn lfxtoffg(&mut self) -> LFXTOFFG_W {
        LFXTOFFG_W { w: self }
    }
    #[doc = "Bit 1 - HFXT High Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn hfxtoffg(&mut self) -> HFXTOFFG_W {
        HFXTOFFG_W { w: self }
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W {
        ENSTFCNT1_W { w: self }
    }
    #[doc = "Bit 7 - Enable start counter for XT2"]
    #[inline(always)]
    pub fn enstfcnt2(&mut self) -> ENSTFCNT2_W {
        ENSTFCNT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl5](index.html) module"]
pub struct CSCTL5_SPEC;
impl crate::RegisterSpec for CSCTL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl5::R](R) reader structure"]
impl crate::Readable for CSCTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl5::W](W) writer structure"]
impl crate::Writable for CSCTL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for CSCTL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
