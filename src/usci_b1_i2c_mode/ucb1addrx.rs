#[doc = "Register `UCB1ADDRX` reader"]
pub struct R(crate::R<UCB1ADDRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1ADDRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB1ADDRX_SPEC>> for R {
    fn from(reader: crate::R<UCB1ADDRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1ADDRX` writer"]
pub struct W(crate::W<UCB1ADDRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1ADDRX_SPEC>;
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
impl core::convert::From<crate::W<UCB1ADDRX_SPEC>> for W {
    fn from(writer: crate::W<UCB1ADDRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCADDRX0` reader - I2C Receive Address Bit 0"]
pub struct UCADDRX0_R(crate::FieldReader<bool, bool>);
impl UCADDRX0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX0` writer - I2C Receive Address Bit 0"]
pub struct UCADDRX0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX0_W<'a> {
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
#[doc = "Field `UCADDRX1` reader - I2C Receive Address Bit 1"]
pub struct UCADDRX1_R(crate::FieldReader<bool, bool>);
impl UCADDRX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX1` writer - I2C Receive Address Bit 1"]
pub struct UCADDRX1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX1_W<'a> {
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
#[doc = "Field `UCADDRX2` reader - I2C Receive Address Bit 2"]
pub struct UCADDRX2_R(crate::FieldReader<bool, bool>);
impl UCADDRX2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX2` writer - I2C Receive Address Bit 2"]
pub struct UCADDRX2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX2_W<'a> {
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
#[doc = "Field `UCADDRX3` reader - I2C Receive Address Bit 3"]
pub struct UCADDRX3_R(crate::FieldReader<bool, bool>);
impl UCADDRX3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX3` writer - I2C Receive Address Bit 3"]
pub struct UCADDRX3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX3_W<'a> {
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
#[doc = "Field `UCADDRX4` reader - I2C Receive Address Bit 4"]
pub struct UCADDRX4_R(crate::FieldReader<bool, bool>);
impl UCADDRX4_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX4` writer - I2C Receive Address Bit 4"]
pub struct UCADDRX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX4_W<'a> {
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
#[doc = "Field `UCADDRX5` reader - I2C Receive Address Bit 5"]
pub struct UCADDRX5_R(crate::FieldReader<bool, bool>);
impl UCADDRX5_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX5` writer - I2C Receive Address Bit 5"]
pub struct UCADDRX5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX5_W<'a> {
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
#[doc = "Field `UCADDRX6` reader - I2C Receive Address Bit 6"]
pub struct UCADDRX6_R(crate::FieldReader<bool, bool>);
impl UCADDRX6_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX6` writer - I2C Receive Address Bit 6"]
pub struct UCADDRX6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX6_W<'a> {
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
#[doc = "Field `UCADDRX7` reader - I2C Receive Address Bit 7"]
pub struct UCADDRX7_R(crate::FieldReader<bool, bool>);
impl UCADDRX7_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX7` writer - I2C Receive Address Bit 7"]
pub struct UCADDRX7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX7_W<'a> {
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
#[doc = "Field `UCADDRX8` reader - I2C Receive Address Bit 8"]
pub struct UCADDRX8_R(crate::FieldReader<bool, bool>);
impl UCADDRX8_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX8` writer - I2C Receive Address Bit 8"]
pub struct UCADDRX8_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX8_W<'a> {
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
#[doc = "Field `UCADDRX9` reader - I2C Receive Address Bit 9"]
pub struct UCADDRX9_R(crate::FieldReader<bool, bool>);
impl UCADDRX9_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCADDRX9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCADDRX9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCADDRX9` writer - I2C Receive Address Bit 9"]
pub struct UCADDRX9_W<'a> {
    w: &'a mut W,
}
impl<'a> UCADDRX9_W<'a> {
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
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&self) -> UCADDRX0_R {
        UCADDRX0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&self) -> UCADDRX1_R {
        UCADDRX1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&self) -> UCADDRX2_R {
        UCADDRX2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&self) -> UCADDRX3_R {
        UCADDRX3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&self) -> UCADDRX4_R {
        UCADDRX4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&self) -> UCADDRX5_R {
        UCADDRX5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&self) -> UCADDRX6_R {
        UCADDRX6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&self) -> UCADDRX7_R {
        UCADDRX7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&self) -> UCADDRX8_R {
        UCADDRX8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&self) -> UCADDRX9_R {
        UCADDRX9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&mut self) -> UCADDRX0_W {
        UCADDRX0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&mut self) -> UCADDRX1_W {
        UCADDRX1_W { w: self }
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&mut self) -> UCADDRX2_W {
        UCADDRX2_W { w: self }
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&mut self) -> UCADDRX3_W {
        UCADDRX3_W { w: self }
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&mut self) -> UCADDRX4_W {
        UCADDRX4_W { w: self }
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&mut self) -> UCADDRX5_W {
        UCADDRX5_W { w: self }
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&mut self) -> UCADDRX6_W {
        UCADDRX6_W { w: self }
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&mut self) -> UCADDRX7_W {
        UCADDRX7_W { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&mut self) -> UCADDRX8_W {
        UCADDRX8_W { w: self }
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&mut self) -> UCADDRX9_W {
        UCADDRX9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addrx](index.html) module"]
pub struct UCB1ADDRX_SPEC;
impl crate::RegisterSpec for UCB1ADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1addrx::R](R) reader structure"]
impl crate::Readable for UCB1ADDRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1addrx::W](W) writer structure"]
impl crate::Writable for UCB1ADDRX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1ADDRX to value 0"]
impl crate::Resettable for UCB1ADDRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
