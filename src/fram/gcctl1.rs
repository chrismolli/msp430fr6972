#[doc = "Register `GCCTL1` reader"]
pub struct R(crate::R<GCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GCCTL1_SPEC>> for R {
    fn from(reader: crate::R<GCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL1` writer"]
pub struct W(crate::W<GCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL1_SPEC>;
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
impl core::convert::From<crate::W<GCCTL1_SPEC>> for W {
    fn from(writer: crate::W<GCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error flag"]
pub struct CBDIFG_R(crate::FieldReader<bool, bool>);
impl CBDIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBDIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBDIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error flag"]
pub struct CBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDIFG_W<'a> {
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
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error flag"]
pub struct UBDIFG_R(crate::FieldReader<bool, bool>);
impl UBDIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UBDIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBDIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error flag"]
pub struct UBDIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDIFG_W<'a> {
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
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub struct ACCTEIFG_R(crate::FieldReader<bool, bool>);
impl ACCTEIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCTEIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCTEIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub struct ACCTEIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCTEIFG_W<'a> {
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
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CBDIFG_W {
        CBDIFG_W { w: self }
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UBDIFG_W {
        UBDIFG_W { w: self }
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> ACCTEIFG_W {
        ACCTEIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl1](index.html) module"]
pub struct GCCTL1_SPEC;
impl crate::RegisterSpec for GCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl1::R](R) reader structure"]
impl crate::Readable for GCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl1::W](W) writer structure"]
impl crate::Writable for GCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for GCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
