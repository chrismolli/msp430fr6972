#[doc = "Register `RTCCTL0` reader"]
pub struct R(crate::R<RTCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCCTL0_SPEC>> for R {
    fn from(reader: crate::R<RTCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL0` writer"]
pub struct W(crate::W<RTCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL0_SPEC>;
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
impl core::convert::From<crate::W<RTCCTL0_SPEC>> for W {
    fn from(writer: crate::W<RTCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCRDYIFG` reader - RTC Ready Interrupt Flag"]
pub struct RTCRDYIFG_R(crate::FieldReader<bool, bool>);
impl RTCRDYIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCRDYIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCRDYIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCRDYIFG` writer - RTC Ready Interrupt Flag"]
pub struct RTCRDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIFG_W<'a> {
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
#[doc = "Field `RTCAIFG` reader - RTC Alarm Interrupt Flag"]
pub struct RTCAIFG_R(crate::FieldReader<bool, bool>);
impl RTCAIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAIFG` writer - RTC Alarm Interrupt Flag"]
pub struct RTCAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIFG_W<'a> {
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
#[doc = "Field `RTCTEVIFG` reader - RTC Time Event Interrupt Flag"]
pub struct RTCTEVIFG_R(crate::FieldReader<bool, bool>);
impl RTCTEVIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTEVIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTEVIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTEVIFG` writer - RTC Time Event Interrupt Flag"]
pub struct RTCTEVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIFG_W<'a> {
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
#[doc = "Field `RTCOFIFG` reader - RTC 32kHz cyrstal oscillator fault interrupt flag"]
pub struct RTCOFIFG_R(crate::FieldReader<bool, bool>);
impl RTCOFIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOFIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOFIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOFIFG` writer - RTC 32kHz cyrstal oscillator fault interrupt flag"]
pub struct RTCOFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIFG_W<'a> {
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
#[doc = "Field `RTCRDYIE` reader - RTC Ready Interrupt Enable Flag"]
pub struct RTCRDYIE_R(crate::FieldReader<bool, bool>);
impl RTCRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCRDYIE` writer - RTC Ready Interrupt Enable Flag"]
pub struct RTCRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDYIE_W<'a> {
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
#[doc = "Field `RTCAIE` reader - RTC Alarm Interrupt Enable Flag"]
pub struct RTCAIE_R(crate::FieldReader<bool, bool>);
impl RTCAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAIE` writer - RTC Alarm Interrupt Enable Flag"]
pub struct RTCAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTCTEVIE` reader - RTC Time Event Interrupt Enable Flag"]
pub struct RTCTEVIE_R(crate::FieldReader<bool, bool>);
impl RTCTEVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTEVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTEVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTEVIE` writer - RTC Time Event Interrupt Enable Flag"]
pub struct RTCTEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEVIE_W<'a> {
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
#[doc = "Field `RTCOFIE` reader - RTC 32kHz cyrstal oscillator fault interrupt enable"]
pub struct RTCOFIE_R(crate::FieldReader<bool, bool>);
impl RTCOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOFIE` writer - RTC 32kHz cyrstal oscillator fault interrupt enable"]
pub struct RTCOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOFIE_W<'a> {
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
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC 32kHz cyrstal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RTCOFIFG_R {
        RTCOFIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC 32kHz cyrstal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RTCOFIE_R {
        RTCOFIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W {
        RTCRDYIFG_W { w: self }
    }
    #[doc = "Bit 1 - RTC Alarm Interrupt Flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W {
        RTCAIFG_W { w: self }
    }
    #[doc = "Bit 2 - RTC Time Event Interrupt Flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W {
        RTCTEVIFG_W { w: self }
    }
    #[doc = "Bit 3 - RTC 32kHz cyrstal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RTCOFIFG_W {
        RTCOFIFG_W { w: self }
    }
    #[doc = "Bit 4 - RTC Ready Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W {
        RTCRDYIE_W { w: self }
    }
    #[doc = "Bit 5 - RTC Alarm Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RTCAIE_W {
        RTCAIE_W { w: self }
    }
    #[doc = "Bit 6 - RTC Time Event Interrupt Enable Flag"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W {
        RTCTEVIE_W { w: self }
    }
    #[doc = "Bit 7 - RTC 32kHz cyrstal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RTCOFIE_W {
        RTCOFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Clock Control 0/Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl0](index.html) module"]
pub struct RTCCTL0_SPEC;
impl crate::RegisterSpec for RTCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl0::R](R) reader structure"]
impl crate::Readable for RTCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl0::W](W) writer structure"]
impl crate::Writable for RTCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL0 to value 0"]
impl crate::Resettable for RTCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
