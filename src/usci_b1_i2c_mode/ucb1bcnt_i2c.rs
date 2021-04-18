#[doc = "Register `UCB1BCNT_I2C` reader"]
pub struct R(crate::R<UCB1BCNT_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1BCNT_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB1BCNT_I2C_SPEC>> for R {
    fn from(reader: crate::R<UCB1BCNT_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1BCNT_I2C` writer"]
pub struct W(crate::W<UCB1BCNT_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1BCNT_I2C_SPEC>;
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
impl core::convert::From<crate::W<UCB1BCNT_I2C_SPEC>> for W {
    fn from(writer: crate::W<UCB1BCNT_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBCNT0` reader - USCI Byte Counter Bit 0"]
pub struct UCBCNT0_R(crate::FieldReader<bool, bool>);
impl UCBCNT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT0` writer - USCI Byte Counter Bit 0"]
pub struct UCBCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `UCBCNT1` reader - USCI Byte Counter Bit 1"]
pub struct UCBCNT1_R(crate::FieldReader<bool, bool>);
impl UCBCNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT1` writer - USCI Byte Counter Bit 1"]
pub struct UCBCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UCBCNT2` reader - USCI Byte Counter Bit 2"]
pub struct UCBCNT2_R(crate::FieldReader<bool, bool>);
impl UCBCNT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT2` writer - USCI Byte Counter Bit 2"]
pub struct UCBCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `UCBCNT3` reader - USCI Byte Counter Bit 3"]
pub struct UCBCNT3_R(crate::FieldReader<bool, bool>);
impl UCBCNT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT3` writer - USCI Byte Counter Bit 3"]
pub struct UCBCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UCBCNT4` reader - USCI Byte Counter Bit 4"]
pub struct UCBCNT4_R(crate::FieldReader<bool, bool>);
impl UCBCNT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT4` writer - USCI Byte Counter Bit 4"]
pub struct UCBCNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UCBCNT5` reader - USCI Byte Counter Bit 5"]
pub struct UCBCNT5_R(crate::FieldReader<bool, bool>);
impl UCBCNT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT5` writer - USCI Byte Counter Bit 5"]
pub struct UCBCNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UCBCNT6` reader - USCI Byte Counter Bit 6"]
pub struct UCBCNT6_R(crate::FieldReader<bool, bool>);
impl UCBCNT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT6` writer - USCI Byte Counter Bit 6"]
pub struct UCBCNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCBCNT7` reader - USCI Byte Counter Bit 7"]
pub struct UCBCNT7_R(crate::FieldReader<bool, bool>);
impl UCBCNT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNT7` writer - USCI Byte Counter Bit 7"]
pub struct UCBCNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&self) -> UCBCNT0_R {
        UCBCNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&self) -> UCBCNT1_R {
        UCBCNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&self) -> UCBCNT2_R {
        UCBCNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&self) -> UCBCNT3_R {
        UCBCNT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&self) -> UCBCNT4_R {
        UCBCNT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&self) -> UCBCNT5_R {
        UCBCNT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&self) -> UCBCNT6_R {
        UCBCNT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&self) -> UCBCNT7_R {
        UCBCNT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline(always)]
    pub fn ucbcnt0(&mut self) -> UCBCNT0_W {
        UCBCNT0_W { w: self }
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline(always)]
    pub fn ucbcnt1(&mut self) -> UCBCNT1_W {
        UCBCNT1_W { w: self }
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline(always)]
    pub fn ucbcnt2(&mut self) -> UCBCNT2_W {
        UCBCNT2_W { w: self }
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline(always)]
    pub fn ucbcnt3(&mut self) -> UCBCNT3_W {
        UCBCNT3_W { w: self }
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline(always)]
    pub fn ucbcnt4(&mut self) -> UCBCNT4_W {
        UCBCNT4_W { w: self }
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline(always)]
    pub fn ucbcnt5(&mut self) -> UCBCNT5_W {
        UCBCNT5_W { w: self }
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline(always)]
    pub fn ucbcnt6(&mut self) -> UCBCNT6_W {
        UCBCNT6_W { w: self }
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline(always)]
    pub fn ucbcnt7(&mut self) -> UCBCNT7_W {
        UCBCNT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Byte Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1bcnt_i2c](index.html) module"]
pub struct UCB1BCNT_I2C_SPEC;
impl crate::RegisterSpec for UCB1BCNT_I2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb1bcnt_i2c::R](R) reader structure"]
impl crate::Readable for UCB1BCNT_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1bcnt_i2c::W](W) writer structure"]
impl crate::Writable for UCB1BCNT_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1BCNT_I2C to value 0"]
impl crate::Resettable for UCB1BCNT_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
