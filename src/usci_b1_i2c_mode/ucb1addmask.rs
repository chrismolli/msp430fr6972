#[doc = "Register `UCB1ADDMASK` reader"]
pub struct R(crate::R<UCB1ADDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1ADDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB1ADDMASK_SPEC>> for R {
    fn from(reader: crate::R<UCB1ADDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1ADDMASK` writer"]
pub struct W(crate::W<UCB1ADDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1ADDMASK_SPEC>;
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
impl core::convert::From<crate::W<UCB1ADDMASK_SPEC>> for W {
    fn from(writer: crate::W<UCB1ADDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCADDMASK0` reader - I2C Address Mask Bit 0"]
pub struct UCADDMASK0_R(crate::FieldReader<bool, bool>);
impl UCADDMASK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK0` writer - I2C Address Mask Bit 0"]
pub struct UCADDMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK0_W<'a> {
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
#[doc = "Field `UCADDMASK1` reader - I2C Address Mask Bit 1"]
pub struct UCADDMASK1_R(crate::FieldReader<bool, bool>);
impl UCADDMASK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK1` writer - I2C Address Mask Bit 1"]
pub struct UCADDMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK1_W<'a> {
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
#[doc = "Field `UCADDMASK2` reader - I2C Address Mask Bit 2"]
pub struct UCADDMASK2_R(crate::FieldReader<bool, bool>);
impl UCADDMASK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK2` writer - I2C Address Mask Bit 2"]
pub struct UCADDMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK2_W<'a> {
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
#[doc = "Field `UCADDMASK3` reader - I2C Address Mask Bit 3"]
pub struct UCADDMASK3_R(crate::FieldReader<bool, bool>);
impl UCADDMASK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK3` writer - I2C Address Mask Bit 3"]
pub struct UCADDMASK3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK3_W<'a> {
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
#[doc = "Field `UCADDMASK4` reader - I2C Address Mask Bit 4"]
pub struct UCADDMASK4_R(crate::FieldReader<bool, bool>);
impl UCADDMASK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK4` writer - I2C Address Mask Bit 4"]
pub struct UCADDMASK4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK4_W<'a> {
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
#[doc = "Field `UCADDMASK5` reader - I2C Address Mask Bit 5"]
pub struct UCADDMASK5_R(crate::FieldReader<bool, bool>);
impl UCADDMASK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK5` writer - I2C Address Mask Bit 5"]
pub struct UCADDMASK5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK5_W<'a> {
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
#[doc = "Field `UCADDMASK6` reader - I2C Address Mask Bit 6"]
pub struct UCADDMASK6_R(crate::FieldReader<bool, bool>);
impl UCADDMASK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK6` writer - I2C Address Mask Bit 6"]
pub struct UCADDMASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK6_W<'a> {
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
#[doc = "Field `UCADDMASK7` reader - I2C Address Mask Bit 7"]
pub struct UCADDMASK7_R(crate::FieldReader<bool, bool>);
impl UCADDMASK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK7` writer - I2C Address Mask Bit 7"]
pub struct UCADDMASK7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK7_W<'a> {
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
#[doc = "Field `UCADDMASK8` reader - I2C Address Mask Bit 8"]
pub struct UCADDMASK8_R(crate::FieldReader<bool, bool>);
impl UCADDMASK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK8` writer - I2C Address Mask Bit 8"]
pub struct UCADDMASK8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK8_W<'a> {
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
#[doc = "Field `UCADDMASK9` reader - I2C Address Mask Bit 9"]
pub struct UCADDMASK9_R(crate::FieldReader<bool, bool>);
impl UCADDMASK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDMASK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDMASK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDMASK9` writer - I2C Address Mask Bit 9"]
pub struct UCADDMASK9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDMASK9_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&self) -> UCADDMASK0_R {
        UCADDMASK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&self) -> UCADDMASK1_R {
        UCADDMASK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&self) -> UCADDMASK2_R {
        UCADDMASK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&self) -> UCADDMASK3_R {
        UCADDMASK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&self) -> UCADDMASK4_R {
        UCADDMASK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&self) -> UCADDMASK5_R {
        UCADDMASK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&self) -> UCADDMASK6_R {
        UCADDMASK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&self) -> UCADDMASK7_R {
        UCADDMASK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&self) -> UCADDMASK8_R {
        UCADDMASK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&self) -> UCADDMASK9_R {
        UCADDMASK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline(always)]
    pub fn ucaddmask0(&mut self) -> UCADDMASK0_W {
        UCADDMASK0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline(always)]
    pub fn ucaddmask1(&mut self) -> UCADDMASK1_W {
        UCADDMASK1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline(always)]
    pub fn ucaddmask2(&mut self) -> UCADDMASK2_W {
        UCADDMASK2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline(always)]
    pub fn ucaddmask3(&mut self) -> UCADDMASK3_W {
        UCADDMASK3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline(always)]
    pub fn ucaddmask4(&mut self) -> UCADDMASK4_W {
        UCADDMASK4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline(always)]
    pub fn ucaddmask5(&mut self) -> UCADDMASK5_W {
        UCADDMASK5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline(always)]
    pub fn ucaddmask6(&mut self) -> UCADDMASK6_W {
        UCADDMASK6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline(always)]
    pub fn ucaddmask7(&mut self) -> UCADDMASK7_W {
        UCADDMASK7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline(always)]
    pub fn ucaddmask8(&mut self) -> UCADDMASK8_W {
        UCADDMASK8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline(always)]
    pub fn ucaddmask9(&mut self) -> UCADDMASK9_W {
        UCADDMASK9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addmask](index.html) module"]
pub struct UCB1ADDMASK_SPEC;
impl crate::RegisterSpec for UCB1ADDMASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1addmask::R](R) reader structure"]
impl crate::Readable for UCB1ADDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1addmask::W](W) writer structure"]
impl crate::Writable for UCB1ADDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1ADDMASK to value 0"]
impl crate::Resettable for UCB1ADDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
