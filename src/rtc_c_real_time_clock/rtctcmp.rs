#[doc = "Register `RTCTCMP` reader"]
pub struct R(crate::R<RTCTCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCTCMP_SPEC>> for R {
    fn from(reader: crate::R<RTCTCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTCMP` writer"]
pub struct W(crate::W<RTCTCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTCMP_SPEC>;
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
impl core::convert::From<crate::W<RTCTCMP_SPEC>> for W {
    fn from(writer: crate::W<RTCTCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCTCMP0` reader - RTC Temperature Compensation Bit 0"]
pub struct RTCTCMP0_R(crate::FieldReader<bool, bool>);
impl RTCTCMP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP0` writer - RTC Temperature Compensation Bit 0"]
pub struct RTCTCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP0_W<'a> {
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
#[doc = "Field `RTCTCMP1` reader - RTC Temperature Compensation Bit 1"]
pub struct RTCTCMP1_R(crate::FieldReader<bool, bool>);
impl RTCTCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP1` writer - RTC Temperature Compensation Bit 1"]
pub struct RTCTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP1_W<'a> {
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
#[doc = "Field `RTCTCMP2` reader - RTC Temperature Compensation Bit 2"]
pub struct RTCTCMP2_R(crate::FieldReader<bool, bool>);
impl RTCTCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP2` writer - RTC Temperature Compensation Bit 2"]
pub struct RTCTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP2_W<'a> {
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
#[doc = "Field `RTCTCMP3` reader - RTC Temperature Compensation Bit 3"]
pub struct RTCTCMP3_R(crate::FieldReader<bool, bool>);
impl RTCTCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP3` writer - RTC Temperature Compensation Bit 3"]
pub struct RTCTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP3_W<'a> {
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
#[doc = "Field `RTCTCMP4` reader - RTC Temperature Compensation Bit 4"]
pub struct RTCTCMP4_R(crate::FieldReader<bool, bool>);
impl RTCTCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP4` writer - RTC Temperature Compensation Bit 4"]
pub struct RTCTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP4_W<'a> {
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
#[doc = "Field `RTCTCMP5` reader - RTC Temperature Compensation Bit 5"]
pub struct RTCTCMP5_R(crate::FieldReader<bool, bool>);
impl RTCTCMP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP5` writer - RTC Temperature Compensation Bit 5"]
pub struct RTCTCMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP5_W<'a> {
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
#[doc = "Field `RTCTCMP6` reader - RTC Temperature Compensation Bit 6"]
pub struct RTCTCMP6_R(crate::FieldReader<bool, bool>);
impl RTCTCMP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP6` writer - RTC Temperature Compensation Bit 6"]
pub struct RTCTCMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP6_W<'a> {
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
#[doc = "Field `RTCTCMP7` reader - RTC Temperature Compensation Bit 7"]
pub struct RTCTCMP7_R(crate::FieldReader<bool, bool>);
impl RTCTCMP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMP7` writer - RTC Temperature Compensation Bit 7"]
pub struct RTCTCMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMP7_W<'a> {
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
#[doc = "Field `RTCTCOK` reader - RTC Temperature compensation write OK"]
pub struct RTCTCOK_R(crate::FieldReader<bool, bool>);
impl RTCTCOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCOK` writer - RTC Temperature compensation write OK"]
pub struct RTCTCOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCOK_W<'a> {
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
#[doc = "Field `RTCTCRDY` reader - RTC Temperature compensation ready"]
pub struct RTCTCRDY_R(crate::FieldReader<bool, bool>);
impl RTCTCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCRDY` writer - RTC Temperature compensation ready"]
pub struct RTCTCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RTCTCMPS` reader - RTC Temperature Compensation Sign"]
pub struct RTCTCMPS_R(crate::FieldReader<bool, bool>);
impl RTCTCMPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCTCMPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCTCMPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTCMPS` writer - RTC Temperature Compensation Sign"]
pub struct RTCTCMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTCMPS_W<'a> {
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
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&self) -> RTCTCMP0_R {
        RTCTCMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&self) -> RTCTCMP1_R {
        RTCTCMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&self) -> RTCTCMP2_R {
        RTCTCMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&self) -> RTCTCMP3_R {
        RTCTCMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&self) -> RTCTCMP4_R {
        RTCTCMP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&self) -> RTCTCMP5_R {
        RTCTCMP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&self) -> RTCTCMP6_R {
        RTCTCMP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&self) -> RTCTCMP7_R {
        RTCTCMP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&self) -> RTCTCOK_R {
        RTCTCOK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&self) -> RTCTCRDY_R {
        RTCTCRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&self) -> RTCTCMPS_R {
        RTCTCMPS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Temperature Compensation Bit 0"]
    #[inline(always)]
    pub fn rtctcmp0(&mut self) -> RTCTCMP0_W {
        RTCTCMP0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Temperature Compensation Bit 1"]
    #[inline(always)]
    pub fn rtctcmp1(&mut self) -> RTCTCMP1_W {
        RTCTCMP1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Temperature Compensation Bit 2"]
    #[inline(always)]
    pub fn rtctcmp2(&mut self) -> RTCTCMP2_W {
        RTCTCMP2_W { w: self }
    }
    #[doc = "Bit 3 - RTC Temperature Compensation Bit 3"]
    #[inline(always)]
    pub fn rtctcmp3(&mut self) -> RTCTCMP3_W {
        RTCTCMP3_W { w: self }
    }
    #[doc = "Bit 4 - RTC Temperature Compensation Bit 4"]
    #[inline(always)]
    pub fn rtctcmp4(&mut self) -> RTCTCMP4_W {
        RTCTCMP4_W { w: self }
    }
    #[doc = "Bit 5 - RTC Temperature Compensation Bit 5"]
    #[inline(always)]
    pub fn rtctcmp5(&mut self) -> RTCTCMP5_W {
        RTCTCMP5_W { w: self }
    }
    #[doc = "Bit 6 - RTC Temperature Compensation Bit 6"]
    #[inline(always)]
    pub fn rtctcmp6(&mut self) -> RTCTCMP6_W {
        RTCTCMP6_W { w: self }
    }
    #[doc = "Bit 7 - RTC Temperature Compensation Bit 7"]
    #[inline(always)]
    pub fn rtctcmp7(&mut self) -> RTCTCMP7_W {
        RTCTCMP7_W { w: self }
    }
    #[doc = "Bit 13 - RTC Temperature compensation write OK"]
    #[inline(always)]
    pub fn rtctcok(&mut self) -> RTCTCOK_W {
        RTCTCOK_W { w: self }
    }
    #[doc = "Bit 14 - RTC Temperature compensation ready"]
    #[inline(always)]
    pub fn rtctcrdy(&mut self) -> RTCTCRDY_W {
        RTCTCRDY_W { w: self }
    }
    #[doc = "Bit 15 - RTC Temperature Compensation Sign"]
    #[inline(always)]
    pub fn rtctcmps(&mut self) -> RTCTCMPS_W {
        RTCTCMPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Timer Temperature Compensation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctcmp](index.html) module"]
pub struct RTCTCMP_SPEC;
impl crate::RegisterSpec for RTCTCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctcmp::R](R) reader structure"]
impl crate::Readable for RTCTCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctcmp::W](W) writer structure"]
impl crate::Writable for RTCTCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTCMP to value 0"]
impl crate::Resettable for RTCTCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
