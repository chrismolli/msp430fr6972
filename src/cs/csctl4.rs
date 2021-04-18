#[doc = "Register `CSCTL4` reader"]
pub struct R(crate::R<CSCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSCTL4_SPEC>> for R {
    fn from(reader: crate::R<CSCTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL4` writer"]
pub struct W(crate::W<CSCTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL4_SPEC>;
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
impl core::convert::From<crate::W<CSCTL4_SPEC>> for W {
    fn from(writer: crate::W<CSCTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFXTOFF` reader - Low Frequency Oscillator (LFXT) disable"]
pub struct LFXTOFF_R(crate::FieldReader<bool, bool>);
impl LFXTOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXTOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXTOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXTOFF` writer - Low Frequency Oscillator (LFXT) disable"]
pub struct LFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTOFF_W<'a> {
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
#[doc = "Field `SMCLKOFF` reader - SMCLK Off"]
pub struct SMCLKOFF_R(crate::FieldReader<bool, bool>);
impl SMCLKOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMCLKOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMCLKOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMCLKOFF` writer - SMCLK Off"]
pub struct SMCLKOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLKOFF_W<'a> {
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
#[doc = "Field `VLOOFF` reader - VLO Off"]
pub struct VLOOFF_R(crate::FieldReader<bool, bool>);
impl VLOOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLOOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLOOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLOOFF` writer - VLO Off"]
pub struct VLOOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VLOOFF_W<'a> {
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
#[doc = "Field `LFXTBYPASS` reader - LFXT bypass mode : 0: internal 1:sourced from external pin"]
pub struct LFXTBYPASS_R(crate::FieldReader<bool, bool>);
impl LFXTBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXTBYPASS` writer - LFXT bypass mode : 0: internal 1:sourced from external pin"]
pub struct LFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTBYPASS_W<'a> {
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
#[doc = "LFXT Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFXTDRIVE_A {
    #[doc = "0: LFXT Drive Level mode: 0"]
    LFXTDRIVE_0 = 0,
    #[doc = "1: LFXT Drive Level mode: 1"]
    LFXTDRIVE_1 = 1,
    #[doc = "2: LFXT Drive Level mode: 2"]
    LFXTDRIVE_2 = 2,
    #[doc = "3: LFXT Drive Level mode: 3"]
    LFXTDRIVE_3 = 3,
}
impl From<LFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFXTDRIVE` reader - LFXT Drive Level mode Bit 0"]
pub struct LFXTDRIVE_R(crate::FieldReader<u8, LFXTDRIVE_A>);
impl LFXTDRIVE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LFXTDRIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTDRIVE_A {
        match self.bits {
            0 => LFXTDRIVE_A::LFXTDRIVE_0,
            1 => LFXTDRIVE_A::LFXTDRIVE_1,
            2 => LFXTDRIVE_A::LFXTDRIVE_2,
            3 => LFXTDRIVE_A::LFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        **self == LFXTDRIVE_A::LFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        **self == LFXTDRIVE_A::LFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        **self == LFXTDRIVE_A::LFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        **self == LFXTDRIVE_A::LFXTDRIVE_3
    }
}
impl core::ops::Deref for LFXTDRIVE_R {
    type Target = crate::FieldReader<u8, LFXTDRIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXTDRIVE` writer - LFXT Drive Level mode Bit 0"]
pub struct LFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFXTDRIVE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "LFXT Drive Level mode: 0"]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_0)
    }
    #[doc = "LFXT Drive Level mode: 1"]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_1)
    }
    #[doc = "LFXT Drive Level mode: 2"]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_2)
    }
    #[doc = "LFXT Drive Level mode: 3"]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `HFXTOFF` reader - High Frequency Oscillator disable"]
pub struct HFXTOFF_R(crate::FieldReader<bool, bool>);
impl HFXTOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXTOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXTOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXTOFF` writer - High Frequency Oscillator disable"]
pub struct HFXTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTOFF_W<'a> {
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
#[doc = "HFXT frequency selection Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFFREQ_A {
    #[doc = "0: HFXT frequency selection: 0"]
    HFFREQ_0 = 0,
    #[doc = "1: HFXT frequency selection: 1"]
    HFFREQ_1 = 1,
    #[doc = "2: HFXT frequency selection: 2"]
    HFFREQ_2 = 2,
    #[doc = "3: HFXT frequency selection: 3"]
    HFFREQ_3 = 3,
}
impl From<HFFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: HFFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFFREQ` reader - HFXT frequency selection Bit 1"]
pub struct HFFREQ_R(crate::FieldReader<u8, HFFREQ_A>);
impl HFFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFFREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFFREQ_A {
        match self.bits {
            0 => HFFREQ_A::HFFREQ_0,
            1 => HFFREQ_A::HFFREQ_1,
            2 => HFFREQ_A::HFFREQ_2,
            3 => HFFREQ_A::HFFREQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFFREQ_0`"]
    #[inline(always)]
    pub fn is_hffreq_0(&self) -> bool {
        **self == HFFREQ_A::HFFREQ_0
    }
    #[doc = "Checks if the value of the field is `HFFREQ_1`"]
    #[inline(always)]
    pub fn is_hffreq_1(&self) -> bool {
        **self == HFFREQ_A::HFFREQ_1
    }
    #[doc = "Checks if the value of the field is `HFFREQ_2`"]
    #[inline(always)]
    pub fn is_hffreq_2(&self) -> bool {
        **self == HFFREQ_A::HFFREQ_2
    }
    #[doc = "Checks if the value of the field is `HFFREQ_3`"]
    #[inline(always)]
    pub fn is_hffreq_3(&self) -> bool {
        **self == HFFREQ_A::HFFREQ_3
    }
}
impl core::ops::Deref for HFFREQ_R {
    type Target = crate::FieldReader<u8, HFFREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFFREQ` writer - HFXT frequency selection Bit 1"]
pub struct HFFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFFREQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HFXT frequency selection: 0"]
    #[inline(always)]
    pub fn hffreq_0(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_0)
    }
    #[doc = "HFXT frequency selection: 1"]
    #[inline(always)]
    pub fn hffreq_1(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_1)
    }
    #[doc = "HFXT frequency selection: 2"]
    #[inline(always)]
    pub fn hffreq_2(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_2)
    }
    #[doc = "HFXT frequency selection: 3"]
    #[inline(always)]
    pub fn hffreq_3(self) -> &'a mut W {
        self.variant(HFFREQ_A::HFFREQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u16 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `HFXTBYPASS` reader - HFXT bypass mode : 0: internal 1:sourced from external pin"]
pub struct HFXTBYPASS_R(crate::FieldReader<bool, bool>);
impl HFXTBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXTBYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXTBYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXTBYPASS` writer - HFXT bypass mode : 0: internal 1:sourced from external pin"]
pub struct HFXTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTBYPASS_W<'a> {
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
#[doc = "HFXT Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXTDRIVE_A {
    #[doc = "0: HFXT Drive Level mode: 0"]
    HFXTDRIVE_0 = 0,
    #[doc = "1: HFXT Drive Level mode: 1"]
    HFXTDRIVE_1 = 1,
    #[doc = "2: HFXT Drive Level mode: 2"]
    HFXTDRIVE_2 = 2,
    #[doc = "3: HFXT Drive Level mode: 3"]
    HFXTDRIVE_3 = 3,
}
impl From<HFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXTDRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFXTDRIVE` reader - HFXT Drive Level mode Bit 0"]
pub struct HFXTDRIVE_R(crate::FieldReader<u8, HFXTDRIVE_A>);
impl HFXTDRIVE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFXTDRIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTDRIVE_A {
        match self.bits {
            0 => HFXTDRIVE_A::HFXTDRIVE_0,
            1 => HFXTDRIVE_A::HFXTDRIVE_1,
            2 => HFXTDRIVE_A::HFXTDRIVE_2,
            3 => HFXTDRIVE_A::HFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        **self == HFXTDRIVE_A::HFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        **self == HFXTDRIVE_A::HFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_hfxtdrive_2(&self) -> bool {
        **self == HFXTDRIVE_A::HFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_hfxtdrive_3(&self) -> bool {
        **self == HFXTDRIVE_A::HFXTDRIVE_3
    }
}
impl core::ops::Deref for HFXTDRIVE_R {
    type Target = crate::FieldReader<u8, HFXTDRIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXTDRIVE` writer - HFXT Drive Level mode Bit 0"]
pub struct HFXTDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTDRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXTDRIVE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HFXT Drive Level mode: 0"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_0)
    }
    #[doc = "HFXT Drive Level mode: 1"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_1)
    }
    #[doc = "HFXT Drive Level mode: 2"]
    #[inline(always)]
    pub fn hfxtdrive_2(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_2)
    }
    #[doc = "HFXT Drive Level mode: 3"]
    #[inline(always)]
    pub fn hfxtdrive_3(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u16 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Frequency Oscillator (LFXT) disable"]
    #[inline(always)]
    pub fn lfxtoff(&self) -> LFXTOFF_R {
        LFXTOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VLO Off"]
    #[inline(always)]
    pub fn vlooff(&self) -> VLOOFF_R {
        VLOOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LFXTBYPASS_R {
        LFXTBYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - LFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LFXTDRIVE_R {
        LFXTDRIVE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - High Frequency Oscillator disable"]
    #[inline(always)]
    pub fn hfxtoff(&self) -> HFXTOFF_R {
        HFXTOFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - HFXT frequency selection Bit 1"]
    #[inline(always)]
    pub fn hffreq(&self) -> HFFREQ_R {
        HFFREQ_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - HFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HFXTBYPASS_R {
        HFXTBYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - HFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HFXTDRIVE_R {
        HFXTDRIVE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Frequency Oscillator (LFXT) disable"]
    #[inline(always)]
    pub fn lfxtoff(&mut self) -> LFXTOFF_W {
        LFXTOFF_W { w: self }
    }
    #[doc = "Bit 1 - SMCLK Off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W {
        SMCLKOFF_W { w: self }
    }
    #[doc = "Bit 3 - VLO Off"]
    #[inline(always)]
    pub fn vlooff(&mut self) -> VLOOFF_W {
        VLOOFF_W { w: self }
    }
    #[doc = "Bit 4 - LFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LFXTBYPASS_W {
        LFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 6:7 - LFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LFXTDRIVE_W {
        LFXTDRIVE_W { w: self }
    }
    #[doc = "Bit 8 - High Frequency Oscillator disable"]
    #[inline(always)]
    pub fn hfxtoff(&mut self) -> HFXTOFF_W {
        HFXTOFF_W { w: self }
    }
    #[doc = "Bits 10:11 - HFXT frequency selection Bit 1"]
    #[inline(always)]
    pub fn hffreq(&mut self) -> HFFREQ_W {
        HFFREQ_W { w: self }
    }
    #[doc = "Bit 12 - HFXT bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HFXTBYPASS_W {
        HFXTBYPASS_W { w: self }
    }
    #[doc = "Bits 14:15 - HFXT Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HFXTDRIVE_W {
        HFXTDRIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl4](index.html) module"]
pub struct CSCTL4_SPEC;
impl crate::RegisterSpec for CSCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl4::R](R) reader structure"]
impl crate::Readable for CSCTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl4::W](W) writer structure"]
impl crate::Writable for CSCTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for CSCTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
