#[doc = "Register `ADC12IER1` reader"]
pub struct R(crate::R<ADC12IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC12IER1_SPEC>> for R {
    fn from(reader: crate::R<ADC12IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IER1` writer"]
pub struct W(crate::W<ADC12IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IER1_SPEC>;
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
impl core::convert::From<crate::W<ADC12IER1_SPEC>> for W {
    fn from(writer: crate::W<ADC12IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IE16` reader - ADC12 Memory 16 Interrupt Enable"]
pub struct ADC12IE16_R(crate::FieldReader<bool, bool>);
impl ADC12IE16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE16` writer - ADC12 Memory 16 Interrupt Enable"]
pub struct ADC12IE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE16_W<'a> {
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
#[doc = "Field `ADC12IE17` reader - ADC12 Memory 17 Interrupt Enable"]
pub struct ADC12IE17_R(crate::FieldReader<bool, bool>);
impl ADC12IE17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE17` writer - ADC12 Memory 17 Interrupt Enable"]
pub struct ADC12IE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE17_W<'a> {
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
#[doc = "Field `ADC12IE18` reader - ADC12 Memory 18 Interrupt Enable"]
pub struct ADC12IE18_R(crate::FieldReader<bool, bool>);
impl ADC12IE18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE18` writer - ADC12 Memory 18 Interrupt Enable"]
pub struct ADC12IE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE18_W<'a> {
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
#[doc = "Field `ADC12IE19` reader - ADC12 Memory 19 Interrupt Enable"]
pub struct ADC12IE19_R(crate::FieldReader<bool, bool>);
impl ADC12IE19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE19` writer - ADC12 Memory 19 Interrupt Enable"]
pub struct ADC12IE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE19_W<'a> {
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
#[doc = "Field `ADC12IE20` reader - ADC12 Memory 20 Interrupt Enable"]
pub struct ADC12IE20_R(crate::FieldReader<bool, bool>);
impl ADC12IE20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE20` writer - ADC12 Memory 20 Interrupt Enable"]
pub struct ADC12IE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE20_W<'a> {
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
#[doc = "Field `ADC12IE21` reader - ADC12 Memory 21 Interrupt Enable"]
pub struct ADC12IE21_R(crate::FieldReader<bool, bool>);
impl ADC12IE21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE21` writer - ADC12 Memory 21 Interrupt Enable"]
pub struct ADC12IE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE21_W<'a> {
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
#[doc = "Field `ADC12IE22` reader - ADC12 Memory 22 Interrupt Enable"]
pub struct ADC12IE22_R(crate::FieldReader<bool, bool>);
impl ADC12IE22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE22` writer - ADC12 Memory 22 Interrupt Enable"]
pub struct ADC12IE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE22_W<'a> {
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
#[doc = "Field `ADC12IE23` reader - ADC12 Memory 23 Interrupt Enable"]
pub struct ADC12IE23_R(crate::FieldReader<bool, bool>);
impl ADC12IE23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE23` writer - ADC12 Memory 23 Interrupt Enable"]
pub struct ADC12IE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE23_W<'a> {
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
#[doc = "Field `ADC12IE24` reader - ADC12 Memory 24 Interrupt Enable"]
pub struct ADC12IE24_R(crate::FieldReader<bool, bool>);
impl ADC12IE24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE24` writer - ADC12 Memory 24 Interrupt Enable"]
pub struct ADC12IE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE24_W<'a> {
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
#[doc = "Field `ADC12IE25` reader - ADC12 Memory 25 Interrupt Enable"]
pub struct ADC12IE25_R(crate::FieldReader<bool, bool>);
impl ADC12IE25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE25` writer - ADC12 Memory 25 Interrupt Enable"]
pub struct ADC12IE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE25_W<'a> {
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
#[doc = "Field `ADC12IE26` reader - ADC12 Memory 26 Interrupt Enable"]
pub struct ADC12IE26_R(crate::FieldReader<bool, bool>);
impl ADC12IE26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE26` writer - ADC12 Memory 26 Interrupt Enable"]
pub struct ADC12IE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE26_W<'a> {
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
#[doc = "Field `ADC12IE27` reader - ADC12 Memory 27 Interrupt Enable"]
pub struct ADC12IE27_R(crate::FieldReader<bool, bool>);
impl ADC12IE27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE27` writer - ADC12 Memory 27 Interrupt Enable"]
pub struct ADC12IE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE27_W<'a> {
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
#[doc = "Field `ADC12IE28` reader - ADC12 Memory 28 Interrupt Enable"]
pub struct ADC12IE28_R(crate::FieldReader<bool, bool>);
impl ADC12IE28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE28` writer - ADC12 Memory 28 Interrupt Enable"]
pub struct ADC12IE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE28_W<'a> {
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
#[doc = "Field `ADC12IE29` reader - ADC12 Memory 29 Interrupt Enable"]
pub struct ADC12IE29_R(crate::FieldReader<bool, bool>);
impl ADC12IE29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE29` writer - ADC12 Memory 29 Interrupt Enable"]
pub struct ADC12IE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE29_W<'a> {
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
#[doc = "Field `ADC12IE30` reader - ADC12 Memory 30 Interrupt Enable"]
pub struct ADC12IE30_R(crate::FieldReader<bool, bool>);
impl ADC12IE30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE30` writer - ADC12 Memory 30 Interrupt Enable"]
pub struct ADC12IE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE30_W<'a> {
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
#[doc = "Field `ADC12IE31` reader - ADC12 Memory 31 Interrupt Enable"]
pub struct ADC12IE31_R(crate::FieldReader<bool, bool>);
impl ADC12IE31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IE31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IE31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IE31` writer - ADC12 Memory 31 Interrupt Enable"]
pub struct ADC12IE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE31_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie16(&self) -> ADC12IE16_R {
        ADC12IE16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie17(&self) -> ADC12IE17_R {
        ADC12IE17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie18(&self) -> ADC12IE18_R {
        ADC12IE18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie19(&self) -> ADC12IE19_R {
        ADC12IE19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie20(&self) -> ADC12IE20_R {
        ADC12IE20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie21(&self) -> ADC12IE21_R {
        ADC12IE21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie22(&self) -> ADC12IE22_R {
        ADC12IE22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie23(&self) -> ADC12IE23_R {
        ADC12IE23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie24(&self) -> ADC12IE24_R {
        ADC12IE24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie25(&self) -> ADC12IE25_R {
        ADC12IE25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie26(&self) -> ADC12IE26_R {
        ADC12IE26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie27(&self) -> ADC12IE27_R {
        ADC12IE27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie28(&self) -> ADC12IE28_R {
        ADC12IE28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie29(&self) -> ADC12IE29_R {
        ADC12IE29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie30(&self) -> ADC12IE30_R {
        ADC12IE30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie31(&self) -> ADC12IE31_R {
        ADC12IE31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie16(&mut self) -> ADC12IE16_W {
        ADC12IE16_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie17(&mut self) -> ADC12IE17_W {
        ADC12IE17_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie18(&mut self) -> ADC12IE18_W {
        ADC12IE18_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie19(&mut self) -> ADC12IE19_W {
        ADC12IE19_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie20(&mut self) -> ADC12IE20_W {
        ADC12IE20_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie21(&mut self) -> ADC12IE21_W {
        ADC12IE21_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie22(&mut self) -> ADC12IE22_W {
        ADC12IE22_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie23(&mut self) -> ADC12IE23_W {
        ADC12IE23_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie24(&mut self) -> ADC12IE24_W {
        ADC12IE24_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie25(&mut self) -> ADC12IE25_W {
        ADC12IE25_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie26(&mut self) -> ADC12IE26_W {
        ADC12IE26_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie27(&mut self) -> ADC12IE27_W {
        ADC12IE27_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie28(&mut self) -> ADC12IE28_W {
        ADC12IE28_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie29(&mut self) -> ADC12IE29_W {
        ADC12IE29_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie30(&mut self) -> ADC12IE30_W {
        ADC12IE30_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie31(&mut self) -> ADC12IE31_W {
        ADC12IE31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 B Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ier1](index.html) module"]
pub struct ADC12IER1_SPEC;
impl crate::RegisterSpec for ADC12IER1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ier1::R](R) reader structure"]
impl crate::Readable for ADC12IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ier1::W](W) writer structure"]
impl crate::Writable for ADC12IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IER1 to value 0"]
impl crate::Resettable for ADC12IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
