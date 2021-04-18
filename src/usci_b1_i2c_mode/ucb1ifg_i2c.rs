#[doc = "Register `UCB1IFG_I2C` reader"]
pub struct R(crate::R<UCB1IFG_I2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1IFG_I2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB1IFG_I2C_SPEC>> for R {
    fn from(reader: crate::R<UCB1IFG_I2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1IFG_I2C` writer"]
pub struct W(crate::W<UCB1IFG_I2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1IFG_I2C_SPEC>;
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
impl core::convert::From<crate::W<UCB1IFG_I2C_SPEC>> for W {
    fn from(writer: crate::W<UCB1IFG_I2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG0` reader - I2C Receive Interrupt Flag 0"]
pub struct UCRXIFG0_R(crate::FieldReader<bool, bool>);
impl UCRXIFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG0` writer - I2C Receive Interrupt Flag 0"]
pub struct UCRXIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG0_W<'a> {
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
#[doc = "Field `UCTXIFG0` reader - I2C Transmit Interrupt Flag 0"]
pub struct UCTXIFG0_R(crate::FieldReader<bool, bool>);
impl UCTXIFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG0` writer - I2C Transmit Interrupt Flag 0"]
pub struct UCTXIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG0_W<'a> {
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
#[doc = "Field `UCSTTIFG` reader - I2C START Condition interrupt Flag"]
pub struct UCSTTIFG_R(crate::FieldReader<bool, bool>);
impl UCSTTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIFG` writer - I2C START Condition interrupt Flag"]
pub struct UCSTTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIFG_W<'a> {
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
#[doc = "Field `UCSTPIFG` reader - I2C STOP Condition interrupt Flag"]
pub struct UCSTPIFG_R(crate::FieldReader<bool, bool>);
impl UCSTPIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTPIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTPIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTPIFG` writer - I2C STOP Condition interrupt Flag"]
pub struct UCSTPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIFG_W<'a> {
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
#[doc = "Field `UCALIFG` reader - I2C Arbitration Lost interrupt Flag"]
pub struct UCALIFG_R(crate::FieldReader<bool, bool>);
impl UCALIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCALIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCALIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCALIFG` writer - I2C Arbitration Lost interrupt Flag"]
pub struct UCALIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIFG_W<'a> {
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
#[doc = "Field `UCNACKIFG` reader - I2C NACK Condition interrupt Flag"]
pub struct UCNACKIFG_R(crate::FieldReader<bool, bool>);
impl UCNACKIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCNACKIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCNACKIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCNACKIFG` writer - I2C NACK Condition interrupt Flag"]
pub struct UCNACKIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIFG_W<'a> {
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
#[doc = "Field `UCBCNTIFG` reader - I2C Byte counter interrupt flag"]
pub struct UCBCNTIFG_R(crate::FieldReader<bool, bool>);
impl UCBCNTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBCNTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNTIFG` writer - I2C Byte counter interrupt flag"]
pub struct UCBCNTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNTIFG_W<'a> {
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
#[doc = "Field `UCCLTOIFG` reader - I2C Clock low Timeout interrupt Flag"]
pub struct UCCLTOIFG_R(crate::FieldReader<bool, bool>);
impl UCCLTOIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCCLTOIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCCLTOIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCCLTOIFG` writer - I2C Clock low Timeout interrupt Flag"]
pub struct UCCLTOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTOIFG_W<'a> {
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
#[doc = "Field `UCRXIFG1` reader - I2C Receive Interrupt Flag 1"]
pub struct UCRXIFG1_R(crate::FieldReader<bool, bool>);
impl UCRXIFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG1` writer - I2C Receive Interrupt Flag 1"]
pub struct UCRXIFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG1_W<'a> {
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
#[doc = "Field `UCTXIFG1` reader - I2C Transmit Interrupt Flag 1"]
pub struct UCTXIFG1_R(crate::FieldReader<bool, bool>);
impl UCTXIFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG1` writer - I2C Transmit Interrupt Flag 1"]
pub struct UCTXIFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG1_W<'a> {
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
#[doc = "Field `UCRXIFG2` reader - I2C Receive Interrupt Flag 2"]
pub struct UCRXIFG2_R(crate::FieldReader<bool, bool>);
impl UCRXIFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG2` writer - I2C Receive Interrupt Flag 2"]
pub struct UCRXIFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG2_W<'a> {
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
#[doc = "Field `UCTXIFG2` reader - I2C Transmit Interrupt Flag 2"]
pub struct UCTXIFG2_R(crate::FieldReader<bool, bool>);
impl UCTXIFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG2` writer - I2C Transmit Interrupt Flag 2"]
pub struct UCTXIFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG2_W<'a> {
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
#[doc = "Field `UCRXIFG3` reader - I2C Receive Interrupt Flag 3"]
pub struct UCRXIFG3_R(crate::FieldReader<bool, bool>);
impl UCRXIFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG3` writer - I2C Receive Interrupt Flag 3"]
pub struct UCRXIFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG3_W<'a> {
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
#[doc = "Field `UCTXIFG3` reader - I2C Transmit Interrupt Flag 3"]
pub struct UCTXIFG3_R(crate::FieldReader<bool, bool>);
impl UCTXIFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG3` writer - I2C Transmit Interrupt Flag 3"]
pub struct UCTXIFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG3_W<'a> {
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
#[doc = "Field `UCBIT9IFG` reader - I2C Bit 9 Possition Interrupt Flag 3"]
pub struct UCBIT9IFG_R(crate::FieldReader<bool, bool>);
impl UCBIT9IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBIT9IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBIT9IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBIT9IFG` writer - I2C Bit 9 Possition Interrupt Flag 3"]
pub struct UCBIT9IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBIT9IFG_W<'a> {
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
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> UCRXIFG0_R {
        UCRXIFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> UCTXIFG0_R {
        UCTXIFG0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UCBCNTIFG_R {
        UCBCNTIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UCCLTOIFG_R {
        UCCLTOIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> UCRXIFG1_R {
        UCRXIFG1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> UCTXIFG1_R {
        UCTXIFG1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> UCRXIFG2_R {
        UCRXIFG2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> UCTXIFG2_R {
        UCTXIFG2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> UCRXIFG3_R {
        UCRXIFG3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> UCTXIFG3_R {
        UCTXIFG3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> UCBIT9IFG_R {
        UCBIT9IFG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> UCRXIFG0_W {
        UCRXIFG0_W { w: self }
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> UCTXIFG0_W {
        UCTXIFG0_W { w: self }
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W {
        UCSTTIFG_W { w: self }
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W {
        UCSTPIFG_W { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W {
        UCALIFG_W { w: self }
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W {
        UCNACKIFG_W { w: self }
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UCBCNTIFG_W {
        UCBCNTIFG_W { w: self }
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UCCLTOIFG_W {
        UCCLTOIFG_W { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> UCRXIFG1_W {
        UCRXIFG1_W { w: self }
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> UCTXIFG1_W {
        UCTXIFG1_W { w: self }
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> UCRXIFG2_W {
        UCRXIFG2_W { w: self }
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> UCTXIFG2_W {
        UCTXIFG2_W { w: self }
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> UCRXIFG3_W {
        UCRXIFG3_W { w: self }
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> UCTXIFG3_W {
        UCTXIFG3_W { w: self }
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> UCBIT9IFG_W {
        UCBIT9IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg_i2c](index.html) module"]
pub struct UCB1IFG_I2C_SPEC;
impl crate::RegisterSpec for UCB1IFG_I2C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1ifg_i2c::R](R) reader structure"]
impl crate::Readable for UCB1IFG_I2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1ifg_i2c::W](W) writer structure"]
impl crate::Writable for UCB1IFG_I2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1IFG_I2C to value 0"]
impl crate::Resettable for UCB1IFG_I2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
