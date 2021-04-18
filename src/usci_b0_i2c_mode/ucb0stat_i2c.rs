#[doc = "Register `UCB0STAT_I2C` reader"]
pub struct R(crate::R<UCB0STAT_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0STAT_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB0STAT_I2C_SPEC>> for R {
    fn from(reader: crate::R<UCB0STAT_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0STAT_I2C` writer"]
pub struct W(crate::W<UCB0STAT_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0STAT_I2C_SPEC>;
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
impl core::convert::From<crate::W<UCB0STAT_I2C_SPEC>> for W {
    fn from(writer: crate::W<UCB0STAT_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub struct UCBBUSY_R(crate::FieldReader<bool, bool>);
impl UCBBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub struct UCBBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBBUSY_W<'a> {
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
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub struct UCGC_R(crate::FieldReader<bool, bool>);
impl UCGC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCGC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCGC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub struct UCGC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGC_W<'a> {
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
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub struct UCSCLLOW_R(crate::FieldReader<bool, bool>);
impl UCSCLLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSCLLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSCLLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub struct UCSCLLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSCLLOW_W<'a> {
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
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W {
        UCBBUSY_W { w: self }
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&mut self) -> UCGC_W {
        UCGC_W { w: self }
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W {
        UCSCLLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0stat_i2c](index.html) module"]
pub struct UCB0STAT_I2C_SPEC;
impl crate::RegisterSpec for UCB0STAT_I2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0stat_i2c::R](R) reader structure"]
impl crate::Readable for UCB0STAT_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0stat_i2c::W](W) writer structure"]
impl crate::Writable for UCB0STAT_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0STAT_I2C to value 0"]
impl crate::Resettable for UCB0STAT_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
