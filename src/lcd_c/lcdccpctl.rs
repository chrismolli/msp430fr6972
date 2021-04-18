#[doc = "Register `LCDCCPCTL` reader"]
pub struct R(crate::R<LCDCCPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCCPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCCPCTL_SPEC>> for R {
    fn from(reader: crate::R<LCDCCPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCCPCTL` writer"]
pub struct W(crate::W<LCDCCPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCCPCTL_SPEC>;
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
impl core::convert::From<crate::W<LCDCCPCTL_SPEC>> for W {
    fn from(writer: crate::W<LCDCCPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCPDIS0` reader - LCD charge pump disable"]
pub struct LCDCPDIS0_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS0` writer - LCD charge pump disable"]
pub struct LCDCPDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS0_W<'a> {
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
#[doc = "Field `LCDCPDIS1` reader - LCD charge pump disable"]
pub struct LCDCPDIS1_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS1` writer - LCD charge pump disable"]
pub struct LCDCPDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS1_W<'a> {
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
#[doc = "Field `LCDCPDIS2` reader - LCD charge pump disable"]
pub struct LCDCPDIS2_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS2` writer - LCD charge pump disable"]
pub struct LCDCPDIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS2_W<'a> {
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
#[doc = "Field `LCDCPDIS3` reader - LCD charge pump disable"]
pub struct LCDCPDIS3_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS3` writer - LCD charge pump disable"]
pub struct LCDCPDIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS3_W<'a> {
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
#[doc = "Field `LCDCPDIS4` reader - LCD charge pump disable"]
pub struct LCDCPDIS4_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS4` writer - LCD charge pump disable"]
pub struct LCDCPDIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS4_W<'a> {
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
#[doc = "Field `LCDCPDIS5` reader - LCD charge pump disable"]
pub struct LCDCPDIS5_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS5` writer - LCD charge pump disable"]
pub struct LCDCPDIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS5_W<'a> {
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
#[doc = "Field `LCDCPDIS6` reader - LCD charge pump disable"]
pub struct LCDCPDIS6_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS6` writer - LCD charge pump disable"]
pub struct LCDCPDIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS6_W<'a> {
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
#[doc = "Field `LCDCPDIS7` reader - LCD charge pump disable"]
pub struct LCDCPDIS7_R(crate::FieldReader<bool, bool>);
impl LCDCPDIS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPDIS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPDIS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPDIS7` writer - LCD charge pump disable"]
pub struct LCDCPDIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPDIS7_W<'a> {
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
#[doc = "Field `LCDCPCLKSYNC` reader - LCD charge pump clock synchronization"]
pub struct LCDCPCLKSYNC_R(crate::FieldReader<bool, bool>);
impl LCDCPCLKSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPCLKSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPCLKSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPCLKSYNC` writer - LCD charge pump clock synchronization"]
pub struct LCDCPCLKSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPCLKSYNC_W<'a> {
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
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&self) -> LCDCPDIS0_R {
        LCDCPDIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&self) -> LCDCPDIS1_R {
        LCDCPDIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&self) -> LCDCPDIS2_R {
        LCDCPDIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&self) -> LCDCPDIS3_R {
        LCDCPDIS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&self) -> LCDCPDIS4_R {
        LCDCPDIS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&self) -> LCDCPDIS5_R {
        LCDCPDIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&self) -> LCDCPDIS6_R {
        LCDCPDIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&self) -> LCDCPDIS7_R {
        LCDCPDIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&self) -> LCDCPCLKSYNC_R {
        LCDCPCLKSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis0(&mut self) -> LCDCPDIS0_W {
        LCDCPDIS0_W { w: self }
    }
    #[doc = "Bit 1 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis1(&mut self) -> LCDCPDIS1_W {
        LCDCPDIS1_W { w: self }
    }
    #[doc = "Bit 2 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis2(&mut self) -> LCDCPDIS2_W {
        LCDCPDIS2_W { w: self }
    }
    #[doc = "Bit 3 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis3(&mut self) -> LCDCPDIS3_W {
        LCDCPDIS3_W { w: self }
    }
    #[doc = "Bit 4 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis4(&mut self) -> LCDCPDIS4_W {
        LCDCPDIS4_W { w: self }
    }
    #[doc = "Bit 5 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis5(&mut self) -> LCDCPDIS5_W {
        LCDCPDIS5_W { w: self }
    }
    #[doc = "Bit 6 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis6(&mut self) -> LCDCPDIS6_W {
        LCDCPDIS6_W { w: self }
    }
    #[doc = "Bit 7 - LCD charge pump disable"]
    #[inline(always)]
    pub fn lcdcpdis7(&mut self) -> LCDCPDIS7_W {
        LCDCPDIS7_W { w: self }
    }
    #[doc = "Bit 15 - LCD charge pump clock synchronization"]
    #[inline(always)]
    pub fn lcdcpclksync(&mut self) -> LCDCPCLKSYNC_W {
        LCDCPCLKSYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Charge Pump Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdccpctl](index.html) module"]
pub struct LCDCCPCTL_SPEC;
impl crate::RegisterSpec for LCDCCPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdccpctl::R](R) reader structure"]
impl crate::Readable for LCDCCPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdccpctl::W](W) writer structure"]
impl crate::Writable for LCDCCPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCCPCTL to value 0"]
impl crate::Resettable for LCDCCPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
