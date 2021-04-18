#[doc = "Register `RTCOCAL` reader"]
pub struct R(crate::R<RTCOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCOCAL_SPEC>> for R {
    fn from(reader: crate::R<RTCOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOCAL` writer"]
pub struct W(crate::W<RTCOCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOCAL_SPEC>;
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
impl core::convert::From<crate::W<RTCOCAL_SPEC>> for W {
    fn from(writer: crate::W<RTCOCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCOCAL0` reader - RTC Offset Calibration Bit 0"]
pub struct RTCOCAL0_R(crate::FieldReader<bool, bool>);
impl RTCOCAL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL0` writer - RTC Offset Calibration Bit 0"]
pub struct RTCOCAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL0_W<'a> {
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
#[doc = "Field `RTCOCAL1` reader - RTC Offset Calibration Bit 1"]
pub struct RTCOCAL1_R(crate::FieldReader<bool, bool>);
impl RTCOCAL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL1` writer - RTC Offset Calibration Bit 1"]
pub struct RTCOCAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL1_W<'a> {
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
#[doc = "Field `RTCOCAL2` reader - RTC Offset Calibration Bit 2"]
pub struct RTCOCAL2_R(crate::FieldReader<bool, bool>);
impl RTCOCAL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL2` writer - RTC Offset Calibration Bit 2"]
pub struct RTCOCAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL2_W<'a> {
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
#[doc = "Field `RTCOCAL3` reader - RTC Offset Calibration Bit 3"]
pub struct RTCOCAL3_R(crate::FieldReader<bool, bool>);
impl RTCOCAL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL3` writer - RTC Offset Calibration Bit 3"]
pub struct RTCOCAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL3_W<'a> {
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
#[doc = "Field `RTCOCAL4` reader - RTC Offset Calibration Bit 4"]
pub struct RTCOCAL4_R(crate::FieldReader<bool, bool>);
impl RTCOCAL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL4` writer - RTC Offset Calibration Bit 4"]
pub struct RTCOCAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL4_W<'a> {
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
#[doc = "Field `RTCOCAL5` reader - RTC Offset Calibration Bit 5"]
pub struct RTCOCAL5_R(crate::FieldReader<bool, bool>);
impl RTCOCAL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL5` writer - RTC Offset Calibration Bit 5"]
pub struct RTCOCAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL5_W<'a> {
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
#[doc = "Field `RTCOCAL6` reader - RTC Offset Calibration Bit 6"]
pub struct RTCOCAL6_R(crate::FieldReader<bool, bool>);
impl RTCOCAL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL6` writer - RTC Offset Calibration Bit 6"]
pub struct RTCOCAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL6_W<'a> {
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
#[doc = "Field `RTCOCAL7` reader - RTC Offset Calibration Bit 7"]
pub struct RTCOCAL7_R(crate::FieldReader<bool, bool>);
impl RTCOCAL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCAL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL7` writer - RTC Offset Calibration Bit 7"]
pub struct RTCOCAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL7_W<'a> {
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
#[doc = "Field `RTCOCALS` reader - RTC Offset Calibration Sign"]
pub struct RTCOCALS_R(crate::FieldReader<bool, bool>);
impl RTCOCALS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCALS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCALS` writer - RTC Offset Calibration Sign"]
pub struct RTCOCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCALS_W<'a> {
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
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&self) -> RTCOCAL0_R {
        RTCOCAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&self) -> RTCOCAL1_R {
        RTCOCAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&self) -> RTCOCAL2_R {
        RTCOCAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&self) -> RTCOCAL3_R {
        RTCOCAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&self) -> RTCOCAL4_R {
        RTCOCAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&self) -> RTCOCAL5_R {
        RTCOCAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&self) -> RTCOCAL6_R {
        RTCOCAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&self) -> RTCOCAL7_R {
        RTCOCAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Offset Calibration Bit 0"]
    #[inline(always)]
    pub fn rtcocal0(&mut self) -> RTCOCAL0_W {
        RTCOCAL0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Offset Calibration Bit 1"]
    #[inline(always)]
    pub fn rtcocal1(&mut self) -> RTCOCAL1_W {
        RTCOCAL1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Offset Calibration Bit 2"]
    #[inline(always)]
    pub fn rtcocal2(&mut self) -> RTCOCAL2_W {
        RTCOCAL2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Offset Calibration Bit 3"]
    #[inline(always)]
    pub fn rtcocal3(&mut self) -> RTCOCAL3_W {
        RTCOCAL3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Offset Calibration Bit 4"]
    #[inline(always)]
    pub fn rtcocal4(&mut self) -> RTCOCAL4_W {
        RTCOCAL4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Offset Calibration Bit 5"]
    #[inline(always)]
    pub fn rtcocal5(&mut self) -> RTCOCAL5_W {
        RTCOCAL5_W { w: self }
    }
    #[doc = "Bit 6 - RTC Offset Calibration Bit 6"]
    #[inline(always)]
    pub fn rtcocal6(&mut self) -> RTCOCAL6_W {
        RTCOCAL6_W { w: self }
    }
    #[doc = "Bit 7 - RTC Offset Calibration Bit 7"]
    #[inline(always)]
    pub fn rtcocal7(&mut self) -> RTCOCAL7_W {
        RTCOCAL7_W { w: self }
    }
    #[doc = "Bit 15 - RTC Offset Calibration Sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W {
        RTCOCALS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Clock Offset Calibartion\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](index.html) module"]
pub struct RTCOCAL_SPEC;
impl crate::RegisterSpec for RTCOCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcocal::R](R) reader structure"]
impl crate::Readable for RTCOCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](W) writer structure"]
impl crate::Writable for RTCOCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOCAL to value 0"]
impl crate::Resettable for RTCOCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
