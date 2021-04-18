#[doc = "Register `ADC12IFGR1` reader"]
pub struct R(crate::R<ADC12IFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC12IFGR1_SPEC>> for R {
    fn from(reader: crate::R<ADC12IFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IFGR1` writer"]
pub struct W(crate::W<ADC12IFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IFGR1_SPEC>;
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
impl core::convert::From<crate::W<ADC12IFGR1_SPEC>> for W {
    fn from(writer: crate::W<ADC12IFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12IFG16` reader - ADC12 Memory 16 Interrupt Flag"]
pub struct ADC12IFG16_R(crate::FieldReader<bool, bool>);
impl ADC12IFG16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG16` writer - ADC12 Memory 16 Interrupt Flag"]
pub struct ADC12IFG16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG16_W<'a> {
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
#[doc = "Field `ADC12IFG17` reader - ADC12 Memory 17 Interrupt Flag"]
pub struct ADC12IFG17_R(crate::FieldReader<bool, bool>);
impl ADC12IFG17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG17` writer - ADC12 Memory 17 Interrupt Flag"]
pub struct ADC12IFG17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG17_W<'a> {
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
#[doc = "Field `ADC12IFG18` reader - ADC12 Memory 18 Interrupt Flag"]
pub struct ADC12IFG18_R(crate::FieldReader<bool, bool>);
impl ADC12IFG18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG18` writer - ADC12 Memory 18 Interrupt Flag"]
pub struct ADC12IFG18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG18_W<'a> {
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
#[doc = "Field `ADC12IFG19` reader - ADC12 Memory 19 Interrupt Flag"]
pub struct ADC12IFG19_R(crate::FieldReader<bool, bool>);
impl ADC12IFG19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG19` writer - ADC12 Memory 19 Interrupt Flag"]
pub struct ADC12IFG19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG19_W<'a> {
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
#[doc = "Field `ADC12IFG20` reader - ADC12 Memory 20 Interrupt Flag"]
pub struct ADC12IFG20_R(crate::FieldReader<bool, bool>);
impl ADC12IFG20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG20` writer - ADC12 Memory 20 Interrupt Flag"]
pub struct ADC12IFG20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG20_W<'a> {
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
#[doc = "Field `ADC12IFG21` reader - ADC12 Memory 21 Interrupt Flag"]
pub struct ADC12IFG21_R(crate::FieldReader<bool, bool>);
impl ADC12IFG21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG21` writer - ADC12 Memory 21 Interrupt Flag"]
pub struct ADC12IFG21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG21_W<'a> {
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
#[doc = "Field `ADC12IFG22` reader - ADC12 Memory 22 Interrupt Flag"]
pub struct ADC12IFG22_R(crate::FieldReader<bool, bool>);
impl ADC12IFG22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG22` writer - ADC12 Memory 22 Interrupt Flag"]
pub struct ADC12IFG22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG22_W<'a> {
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
#[doc = "Field `ADC12IFG23` reader - ADC12 Memory 23 Interrupt Flag"]
pub struct ADC12IFG23_R(crate::FieldReader<bool, bool>);
impl ADC12IFG23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG23` writer - ADC12 Memory 23 Interrupt Flag"]
pub struct ADC12IFG23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG23_W<'a> {
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
#[doc = "Field `ADC12IFG24` reader - ADC12 Memory 24 Interrupt Flag"]
pub struct ADC12IFG24_R(crate::FieldReader<bool, bool>);
impl ADC12IFG24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG24` writer - ADC12 Memory 24 Interrupt Flag"]
pub struct ADC12IFG24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG24_W<'a> {
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
#[doc = "Field `ADC12IFG25` reader - ADC12 Memory 25 Interrupt Flag"]
pub struct ADC12IFG25_R(crate::FieldReader<bool, bool>);
impl ADC12IFG25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG25` writer - ADC12 Memory 25 Interrupt Flag"]
pub struct ADC12IFG25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG25_W<'a> {
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
#[doc = "Field `ADC12IFG26` reader - ADC12 Memory 26 Interrupt Flag"]
pub struct ADC12IFG26_R(crate::FieldReader<bool, bool>);
impl ADC12IFG26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG26` writer - ADC12 Memory 26 Interrupt Flag"]
pub struct ADC12IFG26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG26_W<'a> {
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
#[doc = "Field `ADC12IFG27` reader - ADC12 Memory 27 Interrupt Flag"]
pub struct ADC12IFG27_R(crate::FieldReader<bool, bool>);
impl ADC12IFG27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG27` writer - ADC12 Memory 27 Interrupt Flag"]
pub struct ADC12IFG27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG27_W<'a> {
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
#[doc = "Field `ADC12IFG28` reader - ADC12 Memory 28 Interrupt Flag"]
pub struct ADC12IFG28_R(crate::FieldReader<bool, bool>);
impl ADC12IFG28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG28` writer - ADC12 Memory 28 Interrupt Flag"]
pub struct ADC12IFG28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG28_W<'a> {
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
#[doc = "Field `ADC12IFG29` reader - ADC12 Memory 29 Interrupt Flag"]
pub struct ADC12IFG29_R(crate::FieldReader<bool, bool>);
impl ADC12IFG29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG29` writer - ADC12 Memory 29 Interrupt Flag"]
pub struct ADC12IFG29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG29_W<'a> {
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
#[doc = "Field `ADC12IFG30` reader - ADC12 Memory 30 Interrupt Flag"]
pub struct ADC12IFG30_R(crate::FieldReader<bool, bool>);
impl ADC12IFG30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG30` writer - ADC12 Memory 30 Interrupt Flag"]
pub struct ADC12IFG30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG30_W<'a> {
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
#[doc = "Field `ADC12IFG31` reader - ADC12 Memory 31 Interrupt Flag"]
pub struct ADC12IFG31_R(crate::FieldReader<bool, bool>);
impl ADC12IFG31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12IFG31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12IFG31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12IFG31` writer - ADC12 Memory 31 Interrupt Flag"]
pub struct ADC12IFG31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG31_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg16(&self) -> ADC12IFG16_R {
        ADC12IFG16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg17(&self) -> ADC12IFG17_R {
        ADC12IFG17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg18(&self) -> ADC12IFG18_R {
        ADC12IFG18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg19(&self) -> ADC12IFG19_R {
        ADC12IFG19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg20(&self) -> ADC12IFG20_R {
        ADC12IFG20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg21(&self) -> ADC12IFG21_R {
        ADC12IFG21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg22(&self) -> ADC12IFG22_R {
        ADC12IFG22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg23(&self) -> ADC12IFG23_R {
        ADC12IFG23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg24(&self) -> ADC12IFG24_R {
        ADC12IFG24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg25(&self) -> ADC12IFG25_R {
        ADC12IFG25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg26(&self) -> ADC12IFG26_R {
        ADC12IFG26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg27(&self) -> ADC12IFG27_R {
        ADC12IFG27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg28(&self) -> ADC12IFG28_R {
        ADC12IFG28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg29(&self) -> ADC12IFG29_R {
        ADC12IFG29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg30(&self) -> ADC12IFG30_R {
        ADC12IFG30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg31(&self) -> ADC12IFG31_R {
        ADC12IFG31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg16(&mut self) -> ADC12IFG16_W {
        ADC12IFG16_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg17(&mut self) -> ADC12IFG17_W {
        ADC12IFG17_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg18(&mut self) -> ADC12IFG18_W {
        ADC12IFG18_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg19(&mut self) -> ADC12IFG19_W {
        ADC12IFG19_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg20(&mut self) -> ADC12IFG20_W {
        ADC12IFG20_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg21(&mut self) -> ADC12IFG21_W {
        ADC12IFG21_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg22(&mut self) -> ADC12IFG22_W {
        ADC12IFG22_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg23(&mut self) -> ADC12IFG23_W {
        ADC12IFG23_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg24(&mut self) -> ADC12IFG24_W {
        ADC12IFG24_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg25(&mut self) -> ADC12IFG25_W {
        ADC12IFG25_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg26(&mut self) -> ADC12IFG26_W {
        ADC12IFG26_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg27(&mut self) -> ADC12IFG27_W {
        ADC12IFG27_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg28(&mut self) -> ADC12IFG28_W {
        ADC12IFG28_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg29(&mut self) -> ADC12IFG29_W {
        ADC12IFG29_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg30(&mut self) -> ADC12IFG30_W {
        ADC12IFG30_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg31(&mut self) -> ADC12IFG31_W {
        ADC12IFG31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 B Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifgr1](index.html) module"]
pub struct ADC12IFGR1_SPEC;
impl crate::RegisterSpec for ADC12IFGR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ifgr1::R](R) reader structure"]
impl crate::Readable for ADC12IFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ifgr1::W](W) writer structure"]
impl crate::Writable for ADC12IFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IFGR1 to value 0"]
impl crate::Resettable for ADC12IFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
