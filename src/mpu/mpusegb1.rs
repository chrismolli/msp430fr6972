#[doc = "Register `MPUSEGB1` reader"]
pub struct R(crate::R<MPUSEGB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUSEGB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUSEGB1_SPEC>> for R {
    fn from(reader: crate::R<MPUSEGB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUSEGB1` writer"]
pub struct W(crate::W<MPUSEGB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUSEGB1_SPEC>;
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
impl core::convert::From<crate::W<MPUSEGB1_SPEC>> for W {
    fn from(writer: crate::W<MPUSEGB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSEGB10` reader - MPU Segment Border 1 Bit: 0"]
pub struct MPUSEGB10_R(crate::FieldReader<bool, bool>);
impl MPUSEGB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB10` writer - MPU Segment Border 1 Bit: 0"]
pub struct MPUSEGB10_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB10_W<'a> {
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
#[doc = "Field `MPUSEGB11` reader - MPU Segment Border 1 Bit: 1"]
pub struct MPUSEGB11_R(crate::FieldReader<bool, bool>);
impl MPUSEGB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB11` writer - MPU Segment Border 1 Bit: 1"]
pub struct MPUSEGB11_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB11_W<'a> {
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
#[doc = "Field `MPUSEGB12` reader - MPU Segment Border 1 Bit: 2"]
pub struct MPUSEGB12_R(crate::FieldReader<bool, bool>);
impl MPUSEGB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB12` writer - MPU Segment Border 1 Bit: 2"]
pub struct MPUSEGB12_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB12_W<'a> {
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
#[doc = "Field `MPUSEGB13` reader - MPU Segment Border 1 Bit: 3"]
pub struct MPUSEGB13_R(crate::FieldReader<bool, bool>);
impl MPUSEGB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB13` writer - MPU Segment Border 1 Bit: 3"]
pub struct MPUSEGB13_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB13_W<'a> {
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
#[doc = "Field `MPUSEGB14` reader - MPU Segment Border 1 Bit: 4"]
pub struct MPUSEGB14_R(crate::FieldReader<bool, bool>);
impl MPUSEGB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB14` writer - MPU Segment Border 1 Bit: 4"]
pub struct MPUSEGB14_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB14_W<'a> {
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
#[doc = "Field `MPUSEGB15` reader - MPU Segment Border 1 Bit: 5"]
pub struct MPUSEGB15_R(crate::FieldReader<bool, bool>);
impl MPUSEGB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB15` writer - MPU Segment Border 1 Bit: 5"]
pub struct MPUSEGB15_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB15_W<'a> {
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
#[doc = "Field `MPUSEGB16` reader - MPU Segment Border 1 Bit: 6"]
pub struct MPUSEGB16_R(crate::FieldReader<bool, bool>);
impl MPUSEGB16_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB16` writer - MPU Segment Border 1 Bit: 6"]
pub struct MPUSEGB16_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB16_W<'a> {
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
#[doc = "Field `MPUSEGB17` reader - MPU Segment Border 1 Bit: 7"]
pub struct MPUSEGB17_R(crate::FieldReader<bool, bool>);
impl MPUSEGB17_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB17` writer - MPU Segment Border 1 Bit: 7"]
pub struct MPUSEGB17_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB17_W<'a> {
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
#[doc = "Field `MPUSEGB18` reader - MPU Segment Border 1 Bit: 8"]
pub struct MPUSEGB18_R(crate::FieldReader<bool, bool>);
impl MPUSEGB18_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB18` writer - MPU Segment Border 1 Bit: 8"]
pub struct MPUSEGB18_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB18_W<'a> {
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
#[doc = "Field `MPUSEGB19` reader - MPU Segment Border 1 Bit: 9"]
pub struct MPUSEGB19_R(crate::FieldReader<bool, bool>);
impl MPUSEGB19_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB19` writer - MPU Segment Border 1 Bit: 9"]
pub struct MPUSEGB19_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB19_W<'a> {
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
#[doc = "Field `MPUSEGB110` reader - MPU Segment Border 1 Bit: 10"]
pub struct MPUSEGB110_R(crate::FieldReader<bool, bool>);
impl MPUSEGB110_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB110` writer - MPU Segment Border 1 Bit: 10"]
pub struct MPUSEGB110_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB110_W<'a> {
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
#[doc = "Field `MPUSEGB111` reader - MPU Segment Border 1 Bit: 11"]
pub struct MPUSEGB111_R(crate::FieldReader<bool, bool>);
impl MPUSEGB111_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB111` writer - MPU Segment Border 1 Bit: 11"]
pub struct MPUSEGB111_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB111_W<'a> {
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
#[doc = "Field `MPUSEGB112` reader - MPU Segment Border 1 Bit: 12"]
pub struct MPUSEGB112_R(crate::FieldReader<bool, bool>);
impl MPUSEGB112_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB112` writer - MPU Segment Border 1 Bit: 12"]
pub struct MPUSEGB112_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB112_W<'a> {
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
#[doc = "Field `MPUSEGB113` reader - MPU Segment Border 1 Bit: 13"]
pub struct MPUSEGB113_R(crate::FieldReader<bool, bool>);
impl MPUSEGB113_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB113` writer - MPU Segment Border 1 Bit: 13"]
pub struct MPUSEGB113_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB113_W<'a> {
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
#[doc = "Field `MPUSEGB114` reader - MPU Segment Border 1 Bit: 14"]
pub struct MPUSEGB114_R(crate::FieldReader<bool, bool>);
impl MPUSEGB114_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB114` writer - MPU Segment Border 1 Bit: 14"]
pub struct MPUSEGB114_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB114_W<'a> {
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
#[doc = "Field `MPUSEGB115` reader - MPU Segment Border 1 Bit: 15"]
pub struct MPUSEGB115_R(crate::FieldReader<bool, bool>);
impl MPUSEGB115_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB115` writer - MPU Segment Border 1 Bit: 15"]
pub struct MPUSEGB115_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB115_W<'a> {
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
    #[doc = "Bit 0 - MPU Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb10(&self) -> MPUSEGB10_R {
        MPUSEGB10_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb11(&self) -> MPUSEGB11_R {
        MPUSEGB11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb12(&self) -> MPUSEGB12_R {
        MPUSEGB12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb13(&self) -> MPUSEGB13_R {
        MPUSEGB13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb14(&self) -> MPUSEGB14_R {
        MPUSEGB14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb15(&self) -> MPUSEGB15_R {
        MPUSEGB15_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb16(&self) -> MPUSEGB16_R {
        MPUSEGB16_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb17(&self) -> MPUSEGB17_R {
        MPUSEGB17_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb18(&self) -> MPUSEGB18_R {
        MPUSEGB18_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb19(&self) -> MPUSEGB19_R {
        MPUSEGB19_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb110(&self) -> MPUSEGB110_R {
        MPUSEGB110_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb111(&self) -> MPUSEGB111_R {
        MPUSEGB111_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb112(&self) -> MPUSEGB112_R {
        MPUSEGB112_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb113(&self) -> MPUSEGB113_R {
        MPUSEGB113_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb114(&self) -> MPUSEGB114_R {
        MPUSEGB114_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb115(&self) -> MPUSEGB115_R {
        MPUSEGB115_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Segment Border 1 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb10(&mut self) -> MPUSEGB10_W {
        MPUSEGB10_W { w: self }
    }
    #[doc = "Bit 1 - MPU Segment Border 1 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb11(&mut self) -> MPUSEGB11_W {
        MPUSEGB11_W { w: self }
    }
    #[doc = "Bit 2 - MPU Segment Border 1 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb12(&mut self) -> MPUSEGB12_W {
        MPUSEGB12_W { w: self }
    }
    #[doc = "Bit 3 - MPU Segment Border 1 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb13(&mut self) -> MPUSEGB13_W {
        MPUSEGB13_W { w: self }
    }
    #[doc = "Bit 4 - MPU Segment Border 1 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb14(&mut self) -> MPUSEGB14_W {
        MPUSEGB14_W { w: self }
    }
    #[doc = "Bit 5 - MPU Segment Border 1 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb15(&mut self) -> MPUSEGB15_W {
        MPUSEGB15_W { w: self }
    }
    #[doc = "Bit 6 - MPU Segment Border 1 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb16(&mut self) -> MPUSEGB16_W {
        MPUSEGB16_W { w: self }
    }
    #[doc = "Bit 7 - MPU Segment Border 1 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb17(&mut self) -> MPUSEGB17_W {
        MPUSEGB17_W { w: self }
    }
    #[doc = "Bit 8 - MPU Segment Border 1 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb18(&mut self) -> MPUSEGB18_W {
        MPUSEGB18_W { w: self }
    }
    #[doc = "Bit 9 - MPU Segment Border 1 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb19(&mut self) -> MPUSEGB19_W {
        MPUSEGB19_W { w: self }
    }
    #[doc = "Bit 10 - MPU Segment Border 1 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb110(&mut self) -> MPUSEGB110_W {
        MPUSEGB110_W { w: self }
    }
    #[doc = "Bit 11 - MPU Segment Border 1 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb111(&mut self) -> MPUSEGB111_W {
        MPUSEGB111_W { w: self }
    }
    #[doc = "Bit 12 - MPU Segment Border 1 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb112(&mut self) -> MPUSEGB112_W {
        MPUSEGB112_W { w: self }
    }
    #[doc = "Bit 13 - MPU Segment Border 1 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb113(&mut self) -> MPUSEGB113_W {
        MPUSEGB113_W { w: self }
    }
    #[doc = "Bit 14 - MPU Segment Border 1 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb114(&mut self) -> MPUSEGB114_W {
        MPUSEGB114_W { w: self }
    }
    #[doc = "Bit 15 - MPU Segment Border 1 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb115(&mut self) -> MPUSEGB115_W {
        MPUSEGB115_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Segmentation Border 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusegb1](index.html) module"]
pub struct MPUSEGB1_SPEC;
impl crate::RegisterSpec for MPUSEGB1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpusegb1::R](R) reader structure"]
impl crate::Readable for MPUSEGB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpusegb1::W](W) writer structure"]
impl crate::Writable for MPUSEGB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUSEGB1 to value 0"]
impl crate::Resettable for MPUSEGB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
