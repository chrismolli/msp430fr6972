#[doc = "Register `LCDCPCTL1` reader"]
pub struct R(crate::R<LCDCPCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCPCTL1_SPEC>> for R {
    fn from(reader: crate::R<LCDCPCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCPCTL1` writer"]
pub struct W(crate::W<LCDCPCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCPCTL1_SPEC>;
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
impl core::convert::From<crate::W<LCDCPCTL1_SPEC>> for W {
    fn from(writer: crate::W<LCDCPCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS16` reader - LCD Segment 16 enable."]
pub struct LCDS16_R(crate::FieldReader<bool, bool>);
impl LCDS16_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS16` writer - LCD Segment 16 enable."]
pub struct LCDS16_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS16_W<'a> {
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
#[doc = "Field `LCDS17` reader - LCD Segment 17 enable."]
pub struct LCDS17_R(crate::FieldReader<bool, bool>);
impl LCDS17_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS17` writer - LCD Segment 17 enable."]
pub struct LCDS17_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS17_W<'a> {
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
#[doc = "Field `LCDS18` reader - LCD Segment 18 enable."]
pub struct LCDS18_R(crate::FieldReader<bool, bool>);
impl LCDS18_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS18` writer - LCD Segment 18 enable."]
pub struct LCDS18_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS18_W<'a> {
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
#[doc = "Field `LCDS19` reader - LCD Segment 19 enable."]
pub struct LCDS19_R(crate::FieldReader<bool, bool>);
impl LCDS19_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS19` writer - LCD Segment 19 enable."]
pub struct LCDS19_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS19_W<'a> {
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
#[doc = "Field `LCDS20` reader - LCD Segment 20 enable."]
pub struct LCDS20_R(crate::FieldReader<bool, bool>);
impl LCDS20_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS20` writer - LCD Segment 20 enable."]
pub struct LCDS20_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS20_W<'a> {
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
#[doc = "Field `LCDS21` reader - LCD Segment 21 enable."]
pub struct LCDS21_R(crate::FieldReader<bool, bool>);
impl LCDS21_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS21` writer - LCD Segment 21 enable."]
pub struct LCDS21_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS21_W<'a> {
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
#[doc = "Field `LCDS22` reader - LCD Segment 22 enable."]
pub struct LCDS22_R(crate::FieldReader<bool, bool>);
impl LCDS22_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS22` writer - LCD Segment 22 enable."]
pub struct LCDS22_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS22_W<'a> {
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
#[doc = "Field `LCDS23` reader - LCD Segment 23 enable."]
pub struct LCDS23_R(crate::FieldReader<bool, bool>);
impl LCDS23_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS23` writer - LCD Segment 23 enable."]
pub struct LCDS23_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS23_W<'a> {
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
#[doc = "Field `LCDS24` reader - LCD Segment 24 enable."]
pub struct LCDS24_R(crate::FieldReader<bool, bool>);
impl LCDS24_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS24` writer - LCD Segment 24 enable."]
pub struct LCDS24_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS24_W<'a> {
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
#[doc = "Field `LCDS25` reader - LCD Segment 25 enable."]
pub struct LCDS25_R(crate::FieldReader<bool, bool>);
impl LCDS25_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS25` writer - LCD Segment 25 enable."]
pub struct LCDS25_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS25_W<'a> {
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
#[doc = "Field `LCDS26` reader - LCD Segment 26 enable."]
pub struct LCDS26_R(crate::FieldReader<bool, bool>);
impl LCDS26_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS26` writer - LCD Segment 26 enable."]
pub struct LCDS26_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS26_W<'a> {
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
#[doc = "Field `LCDS27` reader - LCD Segment 27 enable."]
pub struct LCDS27_R(crate::FieldReader<bool, bool>);
impl LCDS27_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS27` writer - LCD Segment 27 enable."]
pub struct LCDS27_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS27_W<'a> {
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
#[doc = "Field `LCDS28` reader - LCD Segment 28 enable."]
pub struct LCDS28_R(crate::FieldReader<bool, bool>);
impl LCDS28_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS28` writer - LCD Segment 28 enable."]
pub struct LCDS28_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS28_W<'a> {
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
#[doc = "Field `LCDS29` reader - LCD Segment 29 enable."]
pub struct LCDS29_R(crate::FieldReader<bool, bool>);
impl LCDS29_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS29` writer - LCD Segment 29 enable."]
pub struct LCDS29_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS29_W<'a> {
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
#[doc = "Field `LCDS30` reader - LCD Segment 30 enable."]
pub struct LCDS30_R(crate::FieldReader<bool, bool>);
impl LCDS30_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS30` writer - LCD Segment 30 enable."]
pub struct LCDS30_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS30_W<'a> {
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
#[doc = "Field `LCDS31` reader - LCD Segment 31 enable."]
pub struct LCDS31_R(crate::FieldReader<bool, bool>);
impl LCDS31_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS31` writer - LCD Segment 31 enable."]
pub struct LCDS31_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS31_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&self) -> LCDS16_R {
        LCDS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&self) -> LCDS17_R {
        LCDS17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&self) -> LCDS18_R {
        LCDS18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&self) -> LCDS19_R {
        LCDS19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&self) -> LCDS20_R {
        LCDS20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&self) -> LCDS21_R {
        LCDS21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&self) -> LCDS22_R {
        LCDS22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&self) -> LCDS23_R {
        LCDS23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&self) -> LCDS24_R {
        LCDS24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&self) -> LCDS25_R {
        LCDS25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&self) -> LCDS26_R {
        LCDS26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&self) -> LCDS27_R {
        LCDS27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&self) -> LCDS28_R {
        LCDS28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&self) -> LCDS29_R {
        LCDS29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&self) -> LCDS30_R {
        LCDS30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&self) -> LCDS31_R {
        LCDS31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 16 enable."]
    #[inline(always)]
    pub fn lcds16(&mut self) -> LCDS16_W {
        LCDS16_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 17 enable."]
    #[inline(always)]
    pub fn lcds17(&mut self) -> LCDS17_W {
        LCDS17_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 18 enable."]
    #[inline(always)]
    pub fn lcds18(&mut self) -> LCDS18_W {
        LCDS18_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 19 enable."]
    #[inline(always)]
    pub fn lcds19(&mut self) -> LCDS19_W {
        LCDS19_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 20 enable."]
    #[inline(always)]
    pub fn lcds20(&mut self) -> LCDS20_W {
        LCDS20_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 21 enable."]
    #[inline(always)]
    pub fn lcds21(&mut self) -> LCDS21_W {
        LCDS21_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 22 enable."]
    #[inline(always)]
    pub fn lcds22(&mut self) -> LCDS22_W {
        LCDS22_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 23 enable."]
    #[inline(always)]
    pub fn lcds23(&mut self) -> LCDS23_W {
        LCDS23_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 24 enable."]
    #[inline(always)]
    pub fn lcds24(&mut self) -> LCDS24_W {
        LCDS24_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 25 enable."]
    #[inline(always)]
    pub fn lcds25(&mut self) -> LCDS25_W {
        LCDS25_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 26 enable."]
    #[inline(always)]
    pub fn lcds26(&mut self) -> LCDS26_W {
        LCDS26_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 27 enable."]
    #[inline(always)]
    pub fn lcds27(&mut self) -> LCDS27_W {
        LCDS27_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 28 enable."]
    #[inline(always)]
    pub fn lcds28(&mut self) -> LCDS28_W {
        LCDS28_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 29 enable."]
    #[inline(always)]
    pub fn lcds29(&mut self) -> LCDS29_W {
        LCDS29_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 30 enable."]
    #[inline(always)]
    pub fn lcds30(&mut self) -> LCDS30_W {
        LCDS30_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 31 enable."]
    #[inline(always)]
    pub fn lcds31(&mut self) -> LCDS31_W {
        LCDS31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl1](index.html) module"]
pub struct LCDCPCTL1_SPEC;
impl crate::RegisterSpec for LCDCPCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcpctl1::R](R) reader structure"]
impl crate::Readable for LCDCPCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcpctl1::W](W) writer structure"]
impl crate::Writable for LCDCPCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCPCTL1 to value 0"]
impl crate::Resettable for LCDCPCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
