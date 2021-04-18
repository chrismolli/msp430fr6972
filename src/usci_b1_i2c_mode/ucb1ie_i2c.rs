#[doc = "Register `UCB1IE_I2C` reader"]
pub struct R(crate::R<UCB1IE_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1IE_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB1IE_I2C_SPEC>> for R {
    fn from(reader: crate::R<UCB1IE_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1IE_I2C` writer"]
pub struct W(crate::W<UCB1IE_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1IE_I2C_SPEC>;
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
impl core::convert::From<crate::W<UCB1IE_I2C_SPEC>> for W {
    fn from(writer: crate::W<UCB1IE_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIE0` reader - I2C Receive Interrupt Enable 0"]
pub struct UCRXIE0_R(crate::FieldReader<bool, bool>);
impl UCRXIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE0` writer - I2C Receive Interrupt Enable 0"]
pub struct UCRXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE0_W<'a> {
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
#[doc = "Field `UCTXIE0` reader - I2C Transmit Interrupt Enable 0"]
pub struct UCTXIE0_R(crate::FieldReader<bool, bool>);
impl UCTXIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE0` writer - I2C Transmit Interrupt Enable 0"]
pub struct UCTXIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE0_W<'a> {
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
#[doc = "Field `UCSTTIE` reader - I2C START Condition interrupt enable"]
pub struct UCSTTIE_R(crate::FieldReader<bool, bool>);
impl UCSTTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIE` writer - I2C START Condition interrupt enable"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
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
#[doc = "Field `UCSTPIE` reader - I2C STOP Condition interrupt enable"]
pub struct UCSTPIE_R(crate::FieldReader<bool, bool>);
impl UCSTPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTPIE` writer - I2C STOP Condition interrupt enable"]
pub struct UCSTPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIE_W<'a> {
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
#[doc = "Field `UCALIE` reader - I2C Arbitration Lost interrupt enable"]
pub struct UCALIE_R(crate::FieldReader<bool, bool>);
impl UCALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCALIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCALIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCALIE` writer - I2C Arbitration Lost interrupt enable"]
pub struct UCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIE_W<'a> {
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
#[doc = "Field `UCNACKIE` reader - I2C NACK Condition interrupt enable"]
pub struct UCNACKIE_R(crate::FieldReader<bool, bool>);
impl UCNACKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCNACKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCNACKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCNACKIE` writer - I2C NACK Condition interrupt enable"]
pub struct UCNACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIE_W<'a> {
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
#[doc = "Field `UCBCNTIE` reader - I2C Automatic stop assertion interrupt enable"]
pub struct UCBCNTIE_R(crate::FieldReader<bool, bool>);
impl UCBCNTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNTIE` writer - I2C Automatic stop assertion interrupt enable"]
pub struct UCBCNTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNTIE_W<'a> {
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
#[doc = "Field `UCCLTOIE` reader - I2C Clock Low Timeout interrupt enable"]
pub struct UCCLTOIE_R(crate::FieldReader<bool, bool>);
impl UCCLTOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCCLTOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCCLTOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCCLTOIE` writer - I2C Clock Low Timeout interrupt enable"]
pub struct UCCLTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTOIE_W<'a> {
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
#[doc = "Field `UCRXIE1` reader - I2C Receive Interrupt Enable 1"]
pub struct UCRXIE1_R(crate::FieldReader<bool, bool>);
impl UCRXIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE1` writer - I2C Receive Interrupt Enable 1"]
pub struct UCRXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE1_W<'a> {
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
#[doc = "Field `UCTXIE1` reader - I2C Transmit Interrupt Enable 1"]
pub struct UCTXIE1_R(crate::FieldReader<bool, bool>);
impl UCTXIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE1` writer - I2C Transmit Interrupt Enable 1"]
pub struct UCTXIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE1_W<'a> {
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
#[doc = "Field `UCRXIE2` reader - I2C Receive Interrupt Enable 2"]
pub struct UCRXIE2_R(crate::FieldReader<bool, bool>);
impl UCRXIE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE2` writer - I2C Receive Interrupt Enable 2"]
pub struct UCRXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE2_W<'a> {
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
#[doc = "Field `UCTXIE2` reader - I2C Transmit Interrupt Enable 2"]
pub struct UCTXIE2_R(crate::FieldReader<bool, bool>);
impl UCTXIE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE2` writer - I2C Transmit Interrupt Enable 2"]
pub struct UCTXIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `UCRXIE3` reader - I2C Receive Interrupt Enable 3"]
pub struct UCRXIE3_R(crate::FieldReader<bool, bool>);
impl UCRXIE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE3` writer - I2C Receive Interrupt Enable 3"]
pub struct UCRXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `UCTXIE3` reader - I2C Transmit Interrupt Enable 3"]
pub struct UCTXIE3_R(crate::FieldReader<bool, bool>);
impl UCTXIE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE3` writer - I2C Transmit Interrupt Enable 3"]
pub struct UCTXIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE3_W<'a> {
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
#[doc = "Field `UCBIT9IE` reader - I2C Bit 9 Position Interrupt Enable 3"]
pub struct UCBIT9IE_R(crate::FieldReader<bool, bool>);
impl UCBIT9IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBIT9IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBIT9IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBIT9IE` writer - I2C Bit 9 Position Interrupt Enable 3"]
pub struct UCBIT9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBIT9IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W {
        UCRXIE0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&mut self) -> UCTXIE0_W {
        UCTXIE0_W { w: self }
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W {
        UCSTPIE_W { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W {
        UCALIE_W { w: self }
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W {
        UCNACKIE_W { w: self }
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W {
        UCBCNTIE_W { w: self }
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W {
        UCCLTOIE_W { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W {
        UCRXIE1_W { w: self }
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&mut self) -> UCTXIE1_W {
        UCTXIE1_W { w: self }
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W {
        UCRXIE2_W { w: self }
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&mut self) -> UCTXIE2_W {
        UCTXIE2_W { w: self }
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W {
        UCRXIE3_W { w: self }
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&mut self) -> UCTXIE3_W {
        UCTXIE3_W { w: self }
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W {
        UCBIT9IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie_i2c](index.html) module"]
pub struct UCB1IE_I2C_SPEC;
impl crate::RegisterSpec for UCB1IE_I2C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1ie_i2c::R](R) reader structure"]
impl crate::Readable for UCB1IE_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1ie_i2c::W](W) writer structure"]
impl crate::Writable for UCB1IE_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1IE_I2C to value 0"]
impl crate::Resettable for UCB1IE_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
