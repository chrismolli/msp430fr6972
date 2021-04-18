#[doc = "Register `CECTL3` reader"]
pub struct R(crate::R<CECTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CECTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CECTL3_SPEC>> for R {
    fn from(reader: crate::R<CECTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CECTL3` writer"]
pub struct W(crate::W<CECTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CECTL3_SPEC>;
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
impl core::convert::From<crate::W<CECTL3_SPEC>> for W {
    fn from(writer: crate::W<CECTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEPD0` reader - Comp. E Disable Input Buffer of Port Register .0"]
pub struct CEPD0_R(crate::FieldReader<bool, bool>);
impl CEPD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD0` writer - Comp. E Disable Input Buffer of Port Register .0"]
pub struct CEPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD0_W<'a> {
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
#[doc = "Field `CEPD1` reader - Comp. E Disable Input Buffer of Port Register .1"]
pub struct CEPD1_R(crate::FieldReader<bool, bool>);
impl CEPD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD1` writer - Comp. E Disable Input Buffer of Port Register .1"]
pub struct CEPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD1_W<'a> {
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
#[doc = "Field `CEPD2` reader - Comp. E Disable Input Buffer of Port Register .2"]
pub struct CEPD2_R(crate::FieldReader<bool, bool>);
impl CEPD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD2` writer - Comp. E Disable Input Buffer of Port Register .2"]
pub struct CEPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD2_W<'a> {
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
#[doc = "Field `CEPD3` reader - Comp. E Disable Input Buffer of Port Register .3"]
pub struct CEPD3_R(crate::FieldReader<bool, bool>);
impl CEPD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD3` writer - Comp. E Disable Input Buffer of Port Register .3"]
pub struct CEPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD3_W<'a> {
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
#[doc = "Field `CEPD4` reader - Comp. E Disable Input Buffer of Port Register .4"]
pub struct CEPD4_R(crate::FieldReader<bool, bool>);
impl CEPD4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD4` writer - Comp. E Disable Input Buffer of Port Register .4"]
pub struct CEPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD4_W<'a> {
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
#[doc = "Field `CEPD5` reader - Comp. E Disable Input Buffer of Port Register .5"]
pub struct CEPD5_R(crate::FieldReader<bool, bool>);
impl CEPD5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD5` writer - Comp. E Disable Input Buffer of Port Register .5"]
pub struct CEPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD5_W<'a> {
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
#[doc = "Field `CEPD6` reader - Comp. E Disable Input Buffer of Port Register .6"]
pub struct CEPD6_R(crate::FieldReader<bool, bool>);
impl CEPD6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD6` writer - Comp. E Disable Input Buffer of Port Register .6"]
pub struct CEPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD6_W<'a> {
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
#[doc = "Field `CEPD7` reader - Comp. E Disable Input Buffer of Port Register .7"]
pub struct CEPD7_R(crate::FieldReader<bool, bool>);
impl CEPD7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD7` writer - Comp. E Disable Input Buffer of Port Register .7"]
pub struct CEPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD7_W<'a> {
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
#[doc = "Field `CEPD8` reader - Comp. E Disable Input Buffer of Port Register .8"]
pub struct CEPD8_R(crate::FieldReader<bool, bool>);
impl CEPD8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD8` writer - Comp. E Disable Input Buffer of Port Register .8"]
pub struct CEPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD8_W<'a> {
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
#[doc = "Field `CEPD9` reader - Comp. E Disable Input Buffer of Port Register .9"]
pub struct CEPD9_R(crate::FieldReader<bool, bool>);
impl CEPD9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD9` writer - Comp. E Disable Input Buffer of Port Register .9"]
pub struct CEPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD9_W<'a> {
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
#[doc = "Field `CEPD10` reader - Comp. E Disable Input Buffer of Port Register .10"]
pub struct CEPD10_R(crate::FieldReader<bool, bool>);
impl CEPD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD10` writer - Comp. E Disable Input Buffer of Port Register .10"]
pub struct CEPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD10_W<'a> {
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
#[doc = "Field `CEPD11` reader - Comp. E Disable Input Buffer of Port Register .11"]
pub struct CEPD11_R(crate::FieldReader<bool, bool>);
impl CEPD11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD11` writer - Comp. E Disable Input Buffer of Port Register .11"]
pub struct CEPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD11_W<'a> {
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
#[doc = "Field `CEPD12` reader - Comp. E Disable Input Buffer of Port Register .12"]
pub struct CEPD12_R(crate::FieldReader<bool, bool>);
impl CEPD12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD12` writer - Comp. E Disable Input Buffer of Port Register .12"]
pub struct CEPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD12_W<'a> {
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
#[doc = "Field `CEPD13` reader - Comp. E Disable Input Buffer of Port Register .13"]
pub struct CEPD13_R(crate::FieldReader<bool, bool>);
impl CEPD13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD13` writer - Comp. E Disable Input Buffer of Port Register .13"]
pub struct CEPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD13_W<'a> {
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
#[doc = "Field `CEPD14` reader - Comp. E Disable Input Buffer of Port Register .14"]
pub struct CEPD14_R(crate::FieldReader<bool, bool>);
impl CEPD14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD14` writer - Comp. E Disable Input Buffer of Port Register .14"]
pub struct CEPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD14_W<'a> {
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
#[doc = "Field `CEPD15` reader - Comp. E Disable Input Buffer of Port Register .15"]
pub struct CEPD15_R(crate::FieldReader<bool, bool>);
impl CEPD15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEPD15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD15` writer - Comp. E Disable Input Buffer of Port Register .15"]
pub struct CEPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD15_W<'a> {
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
    #[doc = "Bit 0 - Comp. E Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cepd0(&self) -> CEPD0_R {
        CEPD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. E Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cepd1(&self) -> CEPD1_R {
        CEPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. E Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cepd2(&self) -> CEPD2_R {
        CEPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. E Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cepd3(&self) -> CEPD3_R {
        CEPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. E Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cepd4(&self) -> CEPD4_R {
        CEPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. E Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cepd5(&self) -> CEPD5_R {
        CEPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comp. E Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cepd6(&self) -> CEPD6_R {
        CEPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. E Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cepd7(&self) -> CEPD7_R {
        CEPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. E Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cepd8(&self) -> CEPD8_R {
        CEPD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. E Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cepd9(&self) -> CEPD9_R {
        CEPD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comp. E Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cepd10(&self) -> CEPD10_R {
        CEPD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comp. E Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cepd11(&self) -> CEPD11_R {
        CEPD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. E Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cepd12(&self) -> CEPD12_R {
        CEPD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comp. E Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cepd13(&self) -> CEPD13_R {
        CEPD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comp. E Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cepd14(&self) -> CEPD14_R {
        CEPD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comp. E Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cepd15(&self) -> CEPD15_R {
        CEPD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. E Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cepd0(&mut self) -> CEPD0_W {
        CEPD0_W { w: self }
    }
    #[doc = "Bit 1 - Comp. E Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cepd1(&mut self) -> CEPD1_W {
        CEPD1_W { w: self }
    }
    #[doc = "Bit 2 - Comp. E Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cepd2(&mut self) -> CEPD2_W {
        CEPD2_W { w: self }
    }
    #[doc = "Bit 3 - Comp. E Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cepd3(&mut self) -> CEPD3_W {
        CEPD3_W { w: self }
    }
    #[doc = "Bit 4 - Comp. E Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cepd4(&mut self) -> CEPD4_W {
        CEPD4_W { w: self }
    }
    #[doc = "Bit 5 - Comp. E Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cepd5(&mut self) -> CEPD5_W {
        CEPD5_W { w: self }
    }
    #[doc = "Bit 6 - Comp. E Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cepd6(&mut self) -> CEPD6_W {
        CEPD6_W { w: self }
    }
    #[doc = "Bit 7 - Comp. E Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cepd7(&mut self) -> CEPD7_W {
        CEPD7_W { w: self }
    }
    #[doc = "Bit 8 - Comp. E Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cepd8(&mut self) -> CEPD8_W {
        CEPD8_W { w: self }
    }
    #[doc = "Bit 9 - Comp. E Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cepd9(&mut self) -> CEPD9_W {
        CEPD9_W { w: self }
    }
    #[doc = "Bit 10 - Comp. E Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cepd10(&mut self) -> CEPD10_W {
        CEPD10_W { w: self }
    }
    #[doc = "Bit 11 - Comp. E Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cepd11(&mut self) -> CEPD11_W {
        CEPD11_W { w: self }
    }
    #[doc = "Bit 12 - Comp. E Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cepd12(&mut self) -> CEPD12_W {
        CEPD12_W { w: self }
    }
    #[doc = "Bit 13 - Comp. E Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cepd13(&mut self) -> CEPD13_W {
        CEPD13_W { w: self }
    }
    #[doc = "Bit 14 - Comp. E Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cepd14(&mut self) -> CEPD14_W {
        CEPD14_W { w: self }
    }
    #[doc = "Bit 15 - Comp. E Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cepd15(&mut self) -> CEPD15_W {
        CEPD15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator E Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl3](index.html) module"]
pub struct CECTL3_SPEC;
impl crate::RegisterSpec for CECTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cectl3::R](R) reader structure"]
impl crate::Readable for CECTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cectl3::W](W) writer structure"]
impl crate::Writable for CECTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CECTL3 to value 0"]
impl crate::Resettable for CECTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
