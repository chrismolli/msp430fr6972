#[doc = "Register `MPUIPSEGB2` reader"]
pub struct R(crate::R<MPUIPSEGB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUIPSEGB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUIPSEGB2_SPEC>> for R {
    fn from(reader: crate::R<MPUIPSEGB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUIPSEGB2` writer"]
pub struct W(crate::W<MPUIPSEGB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUIPSEGB2_SPEC>;
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
impl core::convert::From<crate::W<MPUIPSEGB2_SPEC>> for W {
    fn from(writer: crate::W<MPUIPSEGB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUIPSEGB20` reader - MPU IP Segment Border 2 Bit: 0"]
pub struct MPUIPSEGB20_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB20_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB20` writer - MPU IP Segment Border 2 Bit: 0"]
pub struct MPUIPSEGB20_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB20_W<'a> {
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
#[doc = "Field `MPUIPSEGB21` reader - MPU IP Segment Border 2 Bit: 1"]
pub struct MPUIPSEGB21_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB21_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB21` writer - MPU IP Segment Border 2 Bit: 1"]
pub struct MPUIPSEGB21_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB21_W<'a> {
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
#[doc = "Field `MPUIPSEGB22` reader - MPU IP Segment Border 2 Bit: 2"]
pub struct MPUIPSEGB22_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB22_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB22` writer - MPU IP Segment Border 2 Bit: 2"]
pub struct MPUIPSEGB22_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB22_W<'a> {
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
#[doc = "Field `MPUIPSEGB23` reader - MPU IP Segment Border 2 Bit: 3"]
pub struct MPUIPSEGB23_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB23_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB23` writer - MPU IP Segment Border 2 Bit: 3"]
pub struct MPUIPSEGB23_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB23_W<'a> {
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
#[doc = "Field `MPUIPSEGB24` reader - MPU IP Segment Border 2 Bit: 4"]
pub struct MPUIPSEGB24_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB24_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB24` writer - MPU IP Segment Border 2 Bit: 4"]
pub struct MPUIPSEGB24_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB24_W<'a> {
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
#[doc = "Field `MPUIPSEGB25` reader - MPU IP Segment Border 2 Bit: 5"]
pub struct MPUIPSEGB25_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB25_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB25` writer - MPU IP Segment Border 2 Bit: 5"]
pub struct MPUIPSEGB25_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB25_W<'a> {
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
#[doc = "Field `MPUIPSEGB26` reader - MPU IP Segment Border 2 Bit: 6"]
pub struct MPUIPSEGB26_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB26_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB26` writer - MPU IP Segment Border 2 Bit: 6"]
pub struct MPUIPSEGB26_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB26_W<'a> {
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
#[doc = "Field `MPUIPSEGB27` reader - MPU IP Segment Border 2 Bit: 7"]
pub struct MPUIPSEGB27_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB27_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB27` writer - MPU IP Segment Border 2 Bit: 7"]
pub struct MPUIPSEGB27_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB27_W<'a> {
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
#[doc = "Field `MPUIPSEGB28` reader - MPU IP Segment Border 2 Bit: 8"]
pub struct MPUIPSEGB28_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB28_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB28` writer - MPU IP Segment Border 2 Bit: 8"]
pub struct MPUIPSEGB28_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB28_W<'a> {
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
#[doc = "Field `MPUIPSEGB29` reader - MPU IP Segment Border 2 Bit: 9"]
pub struct MPUIPSEGB29_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB29_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB29` writer - MPU IP Segment Border 2 Bit: 9"]
pub struct MPUIPSEGB29_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB29_W<'a> {
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
#[doc = "Field `MPUIPSEGB210` reader - MPU IP Segment Border 2 Bit: 10"]
pub struct MPUIPSEGB210_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB210_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB210_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB210_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB210` writer - MPU IP Segment Border 2 Bit: 10"]
pub struct MPUIPSEGB210_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB210_W<'a> {
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
#[doc = "Field `MPUIPSEGB211` reader - MPU IP Segment Border 2 Bit: 11"]
pub struct MPUIPSEGB211_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB211_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB211_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB211_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB211` writer - MPU IP Segment Border 2 Bit: 11"]
pub struct MPUIPSEGB211_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB211_W<'a> {
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
#[doc = "Field `MPUIPSEGB212` reader - MPU IP Segment Border 2 Bit: 12"]
pub struct MPUIPSEGB212_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB212_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB212_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB212_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB212` writer - MPU IP Segment Border 2 Bit: 12"]
pub struct MPUIPSEGB212_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB212_W<'a> {
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
#[doc = "Field `MPUIPSEGB213` reader - MPU IP Segment Border 2 Bit: 13"]
pub struct MPUIPSEGB213_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB213_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB213_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB213_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB213` writer - MPU IP Segment Border 2 Bit: 13"]
pub struct MPUIPSEGB213_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB213_W<'a> {
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
#[doc = "Field `MPUIPSEGB214` reader - MPU IP Segment Border 2 Bit: 14"]
pub struct MPUIPSEGB214_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB214_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB214_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB214_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB214` writer - MPU IP Segment Border 2 Bit: 14"]
pub struct MPUIPSEGB214_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB214_W<'a> {
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
#[doc = "Field `MPUIPSEGB215` reader - MPU IP Segment Border 2 Bit: 15"]
pub struct MPUIPSEGB215_R(crate::FieldReader<bool, bool>);
impl MPUIPSEGB215_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUIPSEGB215_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUIPSEGB215_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUIPSEGB215` writer - MPU IP Segment Border 2 Bit: 15"]
pub struct MPUIPSEGB215_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUIPSEGB215_W<'a> {
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
    #[doc = "Bit 0 - MPU IP Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb20(&self) -> MPUIPSEGB20_R {
        MPUIPSEGB20_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU IP Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb21(&self) -> MPUIPSEGB21_R {
        MPUIPSEGB21_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU IP Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb22(&self) -> MPUIPSEGB22_R {
        MPUIPSEGB22_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU IP Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb23(&self) -> MPUIPSEGB23_R {
        MPUIPSEGB23_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU IP Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb24(&self) -> MPUIPSEGB24_R {
        MPUIPSEGB24_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU IP Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb25(&self) -> MPUIPSEGB25_R {
        MPUIPSEGB25_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU IP Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb26(&self) -> MPUIPSEGB26_R {
        MPUIPSEGB26_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU IP Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb27(&self) -> MPUIPSEGB27_R {
        MPUIPSEGB27_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU IP Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb28(&self) -> MPUIPSEGB28_R {
        MPUIPSEGB28_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU IP Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb29(&self) -> MPUIPSEGB29_R {
        MPUIPSEGB29_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU IP Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb210(&self) -> MPUIPSEGB210_R {
        MPUIPSEGB210_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU IP Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb211(&self) -> MPUIPSEGB211_R {
        MPUIPSEGB211_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU IP Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb212(&self) -> MPUIPSEGB212_R {
        MPUIPSEGB212_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU IP Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb213(&self) -> MPUIPSEGB213_R {
        MPUIPSEGB213_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU IP Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb214(&self) -> MPUIPSEGB214_R {
        MPUIPSEGB214_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU IP Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb215(&self) -> MPUIPSEGB215_R {
        MPUIPSEGB215_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU IP Segment Border 2 Bit: 0"]
    #[inline(always)]
    pub fn mpuipsegb20(&mut self) -> MPUIPSEGB20_W {
        MPUIPSEGB20_W { w: self }
    }
    #[doc = "Bit 1 - MPU IP Segment Border 2 Bit: 1"]
    #[inline(always)]
    pub fn mpuipsegb21(&mut self) -> MPUIPSEGB21_W {
        MPUIPSEGB21_W { w: self }
    }
    #[doc = "Bit 2 - MPU IP Segment Border 2 Bit: 2"]
    #[inline(always)]
    pub fn mpuipsegb22(&mut self) -> MPUIPSEGB22_W {
        MPUIPSEGB22_W { w: self }
    }
    #[doc = "Bit 3 - MPU IP Segment Border 2 Bit: 3"]
    #[inline(always)]
    pub fn mpuipsegb23(&mut self) -> MPUIPSEGB23_W {
        MPUIPSEGB23_W { w: self }
    }
    #[doc = "Bit 4 - MPU IP Segment Border 2 Bit: 4"]
    #[inline(always)]
    pub fn mpuipsegb24(&mut self) -> MPUIPSEGB24_W {
        MPUIPSEGB24_W { w: self }
    }
    #[doc = "Bit 5 - MPU IP Segment Border 2 Bit: 5"]
    #[inline(always)]
    pub fn mpuipsegb25(&mut self) -> MPUIPSEGB25_W {
        MPUIPSEGB25_W { w: self }
    }
    #[doc = "Bit 6 - MPU IP Segment Border 2 Bit: 6"]
    #[inline(always)]
    pub fn mpuipsegb26(&mut self) -> MPUIPSEGB26_W {
        MPUIPSEGB26_W { w: self }
    }
    #[doc = "Bit 7 - MPU IP Segment Border 2 Bit: 7"]
    #[inline(always)]
    pub fn mpuipsegb27(&mut self) -> MPUIPSEGB27_W {
        MPUIPSEGB27_W { w: self }
    }
    #[doc = "Bit 8 - MPU IP Segment Border 2 Bit: 8"]
    #[inline(always)]
    pub fn mpuipsegb28(&mut self) -> MPUIPSEGB28_W {
        MPUIPSEGB28_W { w: self }
    }
    #[doc = "Bit 9 - MPU IP Segment Border 2 Bit: 9"]
    #[inline(always)]
    pub fn mpuipsegb29(&mut self) -> MPUIPSEGB29_W {
        MPUIPSEGB29_W { w: self }
    }
    #[doc = "Bit 10 - MPU IP Segment Border 2 Bit: 10"]
    #[inline(always)]
    pub fn mpuipsegb210(&mut self) -> MPUIPSEGB210_W {
        MPUIPSEGB210_W { w: self }
    }
    #[doc = "Bit 11 - MPU IP Segment Border 2 Bit: 11"]
    #[inline(always)]
    pub fn mpuipsegb211(&mut self) -> MPUIPSEGB211_W {
        MPUIPSEGB211_W { w: self }
    }
    #[doc = "Bit 12 - MPU IP Segment Border 2 Bit: 12"]
    #[inline(always)]
    pub fn mpuipsegb212(&mut self) -> MPUIPSEGB212_W {
        MPUIPSEGB212_W { w: self }
    }
    #[doc = "Bit 13 - MPU IP Segment Border 2 Bit: 13"]
    #[inline(always)]
    pub fn mpuipsegb213(&mut self) -> MPUIPSEGB213_W {
        MPUIPSEGB213_W { w: self }
    }
    #[doc = "Bit 14 - MPU IP Segment Border 2 Bit: 14"]
    #[inline(always)]
    pub fn mpuipsegb214(&mut self) -> MPUIPSEGB214_W {
        MPUIPSEGB214_W { w: self }
    }
    #[doc = "Bit 15 - MPU IP Segment Border 2 Bit: 15"]
    #[inline(always)]
    pub fn mpuipsegb215(&mut self) -> MPUIPSEGB215_W {
        MPUIPSEGB215_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU IP Segment Border 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuipsegb2](index.html) module"]
pub struct MPUIPSEGB2_SPEC;
impl crate::RegisterSpec for MPUIPSEGB2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpuipsegb2::R](R) reader structure"]
impl crate::Readable for MPUIPSEGB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpuipsegb2::W](W) writer structure"]
impl crate::Writable for MPUIPSEGB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUIPSEGB2 to value 0"]
impl crate::Resettable for MPUIPSEGB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
