#[doc = "Register `PMMIFG` reader"]
pub struct R(crate::R<PMMIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMMIFG_SPEC>> for R {
    fn from(reader: crate::R<PMMIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMIFG` writer"]
pub struct W(crate::W<PMMIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMIFG_SPEC>;
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
impl core::convert::From<crate::W<PMMIFG_SPEC>> for W {
    fn from(writer: crate::W<PMMIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMBORIFG` reader - PMM Software BOR interrupt flag"]
pub struct PMMBORIFG_R(crate::FieldReader<bool, bool>);
impl PMMBORIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMBORIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMBORIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMBORIFG` writer - PMM Software BOR interrupt flag"]
pub struct PMMBORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMBORIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PMMRSTIFG` reader - PMM RESET pin interrupt flag"]
pub struct PMMRSTIFG_R(crate::FieldReader<bool, bool>);
impl PMMRSTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMRSTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMRSTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMRSTIFG` writer - PMM RESET pin interrupt flag"]
pub struct PMMRSTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMRSTIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PMMPORIFG` reader - PMM Software POR interrupt flag"]
pub struct PMMPORIFG_R(crate::FieldReader<bool, bool>);
impl PMMPORIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMPORIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMPORIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMPORIFG` writer - PMM Software POR interrupt flag"]
pub struct PMMPORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMPORIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SVSHIFG` reader - SVS low side interrupt flag"]
pub struct SVSHIFG_R(crate::FieldReader<bool, bool>);
impl SVSHIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSHIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSHIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSHIFG` writer - SVS low side interrupt flag"]
pub struct SVSHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PMMLPM5IFG` reader - LPM5 indication Flag"]
pub struct PMMLPM5IFG_R(crate::FieldReader<bool, bool>);
impl PMMLPM5IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMMLPM5IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMLPM5IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMLPM5IFG` writer - LPM5 indication Flag"]
pub struct PMMLPM5IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMLPM5IFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PMMBORIFG_R {
        PMMBORIFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PMMRSTIFG_R {
        PMMRSTIFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PMMPORIFG_R {
        PMMPORIFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&self) -> SVSHIFG_R {
        SVSHIFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> PMMLPM5IFG_R {
        PMMLPM5IFG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PMMBORIFG_W {
        PMMBORIFG_W { w: self }
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PMMRSTIFG_W {
        PMMRSTIFG_W { w: self }
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PMMPORIFG_W {
        PMMPORIFG_W { w: self }
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SVSHIFG_W {
        SVSHIFG_W { w: self }
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> PMMLPM5IFG_W {
        PMMLPM5IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](index.html) module"]
pub struct PMMIFG_SPEC;
impl crate::RegisterSpec for PMMIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmifg::R](R) reader structure"]
impl crate::Readable for PMMIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](W) writer structure"]
impl crate::Writable for PMMIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMIFG to value 0"]
impl crate::Resettable for PMMIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
