#[doc = "Register `MPUSEGB2` reader"]
pub struct R(crate::R<MPUSEGB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUSEGB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUSEGB2_SPEC>> for R {
    fn from(reader: crate::R<MPUSEGB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUSEGB2` writer"]
pub struct W(crate::W<MPUSEGB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUSEGB2_SPEC>;
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
impl core::convert::From<crate::W<MPUSEGB2_SPEC>> for W {
    fn from(writer: crate::W<MPUSEGB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSEGB20` reader - MPU Segment Border 2 Bit: 0"]
pub struct MPUSEGB20_R(crate::FieldReader<bool, bool>);
impl MPUSEGB20_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB20` writer - MPU Segment Border 2 Bit: 0"]
pub struct MPUSEGB20_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB20_W<'a> {
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
#[doc = "Field `MPUSEGB21` reader - MPU Segment Border 2 Bit: 1"]
pub struct MPUSEGB21_R(crate::FieldReader<bool, bool>);
impl MPUSEGB21_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB21` writer - MPU Segment Border 2 Bit: 1"]
pub struct MPUSEGB21_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB21_W<'a> {
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
#[doc = "Field `MPUSEGB22` reader - MPU Segment Border 2 Bit: 2"]
pub struct MPUSEGB22_R(crate::FieldReader<bool, bool>);
impl MPUSEGB22_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB22` writer - MPU Segment Border 2 Bit: 2"]
pub struct MPUSEGB22_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB22_W<'a> {
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
#[doc = "Field `MPUSEGB23` reader - MPU Segment Border 2 Bit: 3"]
pub struct MPUSEGB23_R(crate::FieldReader<bool, bool>);
impl MPUSEGB23_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB23` writer - MPU Segment Border 2 Bit: 3"]
pub struct MPUSEGB23_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB23_W<'a> {
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
#[doc = "Field `MPUSEGB24` reader - MPU Segment Border 2 Bit: 4"]
pub struct MPUSEGB24_R(crate::FieldReader<bool, bool>);
impl MPUSEGB24_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB24` writer - MPU Segment Border 2 Bit: 4"]
pub struct MPUSEGB24_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB24_W<'a> {
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
#[doc = "Field `MPUSEGB25` reader - MPU Segment Border 2 Bit: 5"]
pub struct MPUSEGB25_R(crate::FieldReader<bool, bool>);
impl MPUSEGB25_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB25` writer - MPU Segment Border 2 Bit: 5"]
pub struct MPUSEGB25_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB25_W<'a> {
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
#[doc = "Field `MPUSEGB26` reader - MPU Segment Border 2 Bit: 6"]
pub struct MPUSEGB26_R(crate::FieldReader<bool, bool>);
impl MPUSEGB26_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB26` writer - MPU Segment Border 2 Bit: 6"]
pub struct MPUSEGB26_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB26_W<'a> {
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
#[doc = "Field `MPUSEGB27` reader - MPU Segment Border 2 Bit: 7"]
pub struct MPUSEGB27_R(crate::FieldReader<bool, bool>);
impl MPUSEGB27_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB27` writer - MPU Segment Border 2 Bit: 7"]
pub struct MPUSEGB27_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB27_W<'a> {
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
#[doc = "Field `MPUSEGB28` reader - MPU Segment Border 2 Bit: 8"]
pub struct MPUSEGB28_R(crate::FieldReader<bool, bool>);
impl MPUSEGB28_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB28` writer - MPU Segment Border 2 Bit: 8"]
pub struct MPUSEGB28_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB28_W<'a> {
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
#[doc = "Field `MPUSEGB29` reader - MPU Segment Border 2 Bit: 9"]
pub struct MPUSEGB29_R(crate::FieldReader<bool, bool>);
impl MPUSEGB29_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB29` writer - MPU Segment Border 2 Bit: 9"]
pub struct MPUSEGB29_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB29_W<'a> {
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
#[doc = "Field `MPUSEGB210` reader - MPU Segment Border 2 Bit: 10"]
pub struct MPUSEGB210_R(crate::FieldReader<bool, bool>);
impl MPUSEGB210_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB210_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB210_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB210` writer - MPU Segment Border 2 Bit: 10"]
pub struct MPUSEGB210_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB210_W<'a> {
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
#[doc = "Field `MPUSEGB211` reader - MPU Segment Border 2 Bit: 11"]
pub struct MPUSEGB211_R(crate::FieldReader<bool, bool>);
impl MPUSEGB211_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB211_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB211_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB211` writer - MPU Segment Border 2 Bit: 11"]
pub struct MPUSEGB211_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB211_W<'a> {
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
#[doc = "Field `MPUSEGB212` reader - MPU Segment Border 2 Bit: 12"]
pub struct MPUSEGB212_R(crate::FieldReader<bool, bool>);
impl MPUSEGB212_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB212_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB212_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB212` writer - MPU Segment Border 2 Bit: 12"]
pub struct MPUSEGB212_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB212_W<'a> {
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
#[doc = "Field `MPUSEGB213` reader - MPU Segment Border 2 Bit: 13"]
pub struct MPUSEGB213_R(crate::FieldReader<bool, bool>);
impl MPUSEGB213_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB213_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB213_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB213` writer - MPU Segment Border 2 Bit: 13"]
pub struct MPUSEGB213_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB213_W<'a> {
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
#[doc = "Field `MPUSEGB214` reader - MPU Segment Border 2 Bit: 14"]
pub struct MPUSEGB214_R(crate::FieldReader<bool, bool>);
impl MPUSEGB214_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB214_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB214_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB214` writer - MPU Segment Border 2 Bit: 14"]
pub struct MPUSEGB214_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB214_W<'a> {
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
#[doc = "Field `MPUSEGB215` reader - MPU Segment Border 2 Bit: 15"]
pub struct MPUSEGB215_R(crate::FieldReader<bool, bool>);
impl MPUSEGB215_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGB215_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGB215_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGB215` writer - MPU Segment Border 2 Bit: 15"]
pub struct MPUSEGB215_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGB215_W<'a> {
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
    #[doc = "Bit 0 - MPU Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb20(&self) -> MPUSEGB20_R {
        MPUSEGB20_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb21(&self) -> MPUSEGB21_R {
        MPUSEGB21_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb22(&self) -> MPUSEGB22_R {
        MPUSEGB22_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb23(&self) -> MPUSEGB23_R {
        MPUSEGB23_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb24(&self) -> MPUSEGB24_R {
        MPUSEGB24_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb25(&self) -> MPUSEGB25_R {
        MPUSEGB25_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb26(&self) -> MPUSEGB26_R {
        MPUSEGB26_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb27(&self) -> MPUSEGB27_R {
        MPUSEGB27_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb28(&self) -> MPUSEGB28_R {
        MPUSEGB28_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb29(&self) -> MPUSEGB29_R {
        MPUSEGB29_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb210(&self) -> MPUSEGB210_R {
        MPUSEGB210_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb211(&self) -> MPUSEGB211_R {
        MPUSEGB211_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb212(&self) -> MPUSEGB212_R {
        MPUSEGB212_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb213(&self) -> MPUSEGB213_R {
        MPUSEGB213_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb214(&self) -> MPUSEGB214_R {
        MPUSEGB214_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb215(&self) -> MPUSEGB215_R {
        MPUSEGB215_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpusegb20(&mut self) -> MPUSEGB20_W {
        MPUSEGB20_W { w: self }
    }
    #[doc = "Bit 1 - MPU Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpusegb21(&mut self) -> MPUSEGB21_W {
        MPUSEGB21_W { w: self }
    }
    #[doc = "Bit 2 - MPU Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpusegb22(&mut self) -> MPUSEGB22_W {
        MPUSEGB22_W { w: self }
    }
    #[doc = "Bit 3 - MPU Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpusegb23(&mut self) -> MPUSEGB23_W {
        MPUSEGB23_W { w: self }
    }
    #[doc = "Bit 4 - MPU Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpusegb24(&mut self) -> MPUSEGB24_W {
        MPUSEGB24_W { w: self }
    }
    #[doc = "Bit 5 - MPU Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpusegb25(&mut self) -> MPUSEGB25_W {
        MPUSEGB25_W { w: self }
    }
    #[doc = "Bit 6 - MPU Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpusegb26(&mut self) -> MPUSEGB26_W {
        MPUSEGB26_W { w: self }
    }
    #[doc = "Bit 7 - MPU Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpusegb27(&mut self) -> MPUSEGB27_W {
        MPUSEGB27_W { w: self }
    }
    #[doc = "Bit 8 - MPU Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpusegb28(&mut self) -> MPUSEGB28_W {
        MPUSEGB28_W { w: self }
    }
    #[doc = "Bit 9 - MPU Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpusegb29(&mut self) -> MPUSEGB29_W {
        MPUSEGB29_W { w: self }
    }
    #[doc = "Bit 10 - MPU Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpusegb210(&mut self) -> MPUSEGB210_W {
        MPUSEGB210_W { w: self }
    }
    #[doc = "Bit 11 - MPU Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpusegb211(&mut self) -> MPUSEGB211_W {
        MPUSEGB211_W { w: self }
    }
    #[doc = "Bit 12 - MPU Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpusegb212(&mut self) -> MPUSEGB212_W {
        MPUSEGB212_W { w: self }
    }
    #[doc = "Bit 13 - MPU Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpusegb213(&mut self) -> MPUSEGB213_W {
        MPUSEGB213_W { w: self }
    }
    #[doc = "Bit 14 - MPU Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpusegb214(&mut self) -> MPUSEGB214_W {
        MPUSEGB214_W { w: self }
    }
    #[doc = "Bit 15 - MPU Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpusegb215(&mut self) -> MPUSEGB215_W {
        MPUSEGB215_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Segmentation Border 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusegb2](index.html) module"]
pub struct MPUSEGB2_SPEC;
impl crate::RegisterSpec for MPUSEGB2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpusegb2::R](R) reader structure"]
impl crate::Readable for MPUSEGB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpusegb2::W](W) writer structure"]
impl crate::Writable for MPUSEGB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUSEGB2 to value 0"]
impl crate::Resettable for MPUSEGB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
