#[doc = "Register `MPUSAM` reader"]
pub struct R(crate::R<MPUSAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUSAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUSAM_SPEC>> for R {
    fn from(reader: crate::R<MPUSAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUSAM` writer"]
pub struct W(crate::W<MPUSAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUSAM_SPEC>;
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
impl core::convert::From<crate::W<MPUSAM_SPEC>> for W {
    fn from(writer: crate::W<MPUSAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSEG1RE` reader - MPU Main memory Segment 1 Read enable"]
pub struct MPUSEG1RE_R(crate::FieldReader<bool, bool>);
impl MPUSEG1RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG1RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG1RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG1RE` writer - MPU Main memory Segment 1 Read enable"]
pub struct MPUSEG1RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1RE_W<'a> {
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
#[doc = "Field `MPUSEG1WE` reader - MPU Main memory Segment 1 Write enable"]
pub struct MPUSEG1WE_R(crate::FieldReader<bool, bool>);
impl MPUSEG1WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG1WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG1WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG1WE` writer - MPU Main memory Segment 1 Write enable"]
pub struct MPUSEG1WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1WE_W<'a> {
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
#[doc = "Field `MPUSEG1XE` reader - MPU Main memory Segment 1 Execute enable"]
pub struct MPUSEG1XE_R(crate::FieldReader<bool, bool>);
impl MPUSEG1XE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG1XE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG1XE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG1XE` writer - MPU Main memory Segment 1 Execute enable"]
pub struct MPUSEG1XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1XE_W<'a> {
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
#[doc = "Field `MPUSEG1VS` reader - MPU Main memory Segment 1 Violation select"]
pub struct MPUSEG1VS_R(crate::FieldReader<bool, bool>);
impl MPUSEG1VS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG1VS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG1VS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG1VS` writer - MPU Main memory Segment 1 Violation select"]
pub struct MPUSEG1VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1VS_W<'a> {
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
#[doc = "Field `MPUSEG2RE` reader - MPU Main memory Segment 2 Read enable"]
pub struct MPUSEG2RE_R(crate::FieldReader<bool, bool>);
impl MPUSEG2RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG2RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG2RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG2RE` writer - MPU Main memory Segment 2 Read enable"]
pub struct MPUSEG2RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2RE_W<'a> {
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
#[doc = "Field `MPUSEG2WE` reader - MPU Main memory Segment 2 Write enable"]
pub struct MPUSEG2WE_R(crate::FieldReader<bool, bool>);
impl MPUSEG2WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG2WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG2WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG2WE` writer - MPU Main memory Segment 2 Write enable"]
pub struct MPUSEG2WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2WE_W<'a> {
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
#[doc = "Field `MPUSEG2XE` reader - MPU Main memory Segment 2 Execute enable"]
pub struct MPUSEG2XE_R(crate::FieldReader<bool, bool>);
impl MPUSEG2XE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG2XE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG2XE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG2XE` writer - MPU Main memory Segment 2 Execute enable"]
pub struct MPUSEG2XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2XE_W<'a> {
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
#[doc = "Field `MPUSEG2VS` reader - MPU Main memory Segment 2 Violation select"]
pub struct MPUSEG2VS_R(crate::FieldReader<bool, bool>);
impl MPUSEG2VS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG2VS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG2VS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG2VS` writer - MPU Main memory Segment 2 Violation select"]
pub struct MPUSEG2VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2VS_W<'a> {
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
#[doc = "Field `MPUSEG3RE` reader - MPU Main memory Segment 3 Read enable"]
pub struct MPUSEG3RE_R(crate::FieldReader<bool, bool>);
impl MPUSEG3RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG3RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG3RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG3RE` writer - MPU Main memory Segment 3 Read enable"]
pub struct MPUSEG3RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3RE_W<'a> {
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
#[doc = "Field `MPUSEG3WE` reader - MPU Main memory Segment 3 Write enable"]
pub struct MPUSEG3WE_R(crate::FieldReader<bool, bool>);
impl MPUSEG3WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG3WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG3WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG3WE` writer - MPU Main memory Segment 3 Write enable"]
pub struct MPUSEG3WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3WE_W<'a> {
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
#[doc = "Field `MPUSEG3XE` reader - MPU Main memory Segment 3 Execute enable"]
pub struct MPUSEG3XE_R(crate::FieldReader<bool, bool>);
impl MPUSEG3XE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG3XE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG3XE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG3XE` writer - MPU Main memory Segment 3 Execute enable"]
pub struct MPUSEG3XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3XE_W<'a> {
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
#[doc = "Field `MPUSEG3VS` reader - MPU Main memory Segment 3 Violation select"]
pub struct MPUSEG3VS_R(crate::FieldReader<bool, bool>);
impl MPUSEG3VS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG3VS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG3VS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG3VS` writer - MPU Main memory Segment 3 Violation select"]
pub struct MPUSEG3VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3VS_W<'a> {
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
#[doc = "Field `MPUSEGIRE` reader - MPU Info memory Segment Read enable"]
pub struct MPUSEGIRE_R(crate::FieldReader<bool, bool>);
impl MPUSEGIRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIRE` writer - MPU Info memory Segment Read enable"]
pub struct MPUSEGIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIRE_W<'a> {
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
#[doc = "Field `MPUSEGIWE` reader - MPU Info memory Segment Write enable"]
pub struct MPUSEGIWE_R(crate::FieldReader<bool, bool>);
impl MPUSEGIWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIWE` writer - MPU Info memory Segment Write enable"]
pub struct MPUSEGIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIWE_W<'a> {
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
#[doc = "Field `MPUSEGIXE` reader - MPU Info memory Segment Execute enable"]
pub struct MPUSEGIXE_R(crate::FieldReader<bool, bool>);
impl MPUSEGIXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIXE` writer - MPU Info memory Segment Execute enable"]
pub struct MPUSEGIXE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIXE_W<'a> {
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
#[doc = "Field `MPUSEGIVS` reader - MPU Info memory Segment Violation select"]
pub struct MPUSEGIVS_R(crate::FieldReader<bool, bool>);
impl MPUSEGIVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIVS` writer - MPU Info memory Segment Violation select"]
pub struct MPUSEGIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIVS_W<'a> {
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
    #[doc = "Bit 0 - MPU Main memory Segment 1 Read enable"]
    #[inline(always)]
    pub fn mpuseg1re(&self) -> MPUSEG1RE_R {
        MPUSEG1RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Main memory Segment 1 Write enable"]
    #[inline(always)]
    pub fn mpuseg1we(&self) -> MPUSEG1WE_R {
        MPUSEG1WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Main memory Segment 1 Execute enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&self) -> MPUSEG1XE_R {
        MPUSEG1XE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Main memory Segment 1 Violation select"]
    #[inline(always)]
    pub fn mpuseg1vs(&self) -> MPUSEG1VS_R {
        MPUSEG1VS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Main memory Segment 2 Read enable"]
    #[inline(always)]
    pub fn mpuseg2re(&self) -> MPUSEG2RE_R {
        MPUSEG2RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Main memory Segment 2 Write enable"]
    #[inline(always)]
    pub fn mpuseg2we(&self) -> MPUSEG2WE_R {
        MPUSEG2WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Main memory Segment 2 Execute enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&self) -> MPUSEG2XE_R {
        MPUSEG2XE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Main memory Segment 2 Violation select"]
    #[inline(always)]
    pub fn mpuseg2vs(&self) -> MPUSEG2VS_R {
        MPUSEG2VS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Main memory Segment 3 Read enable"]
    #[inline(always)]
    pub fn mpuseg3re(&self) -> MPUSEG3RE_R {
        MPUSEG3RE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Main memory Segment 3 Write enable"]
    #[inline(always)]
    pub fn mpuseg3we(&self) -> MPUSEG3WE_R {
        MPUSEG3WE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Main memory Segment 3 Execute enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&self) -> MPUSEG3XE_R {
        MPUSEG3XE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Main memory Segment 3 Violation select"]
    #[inline(always)]
    pub fn mpuseg3vs(&self) -> MPUSEG3VS_R {
        MPUSEG3VS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Info memory Segment Read enable"]
    #[inline(always)]
    pub fn mpusegire(&self) -> MPUSEGIRE_R {
        MPUSEGIRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Info memory Segment Write enable"]
    #[inline(always)]
    pub fn mpusegiwe(&self) -> MPUSEGIWE_R {
        MPUSEGIWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Info memory Segment Execute enable"]
    #[inline(always)]
    pub fn mpusegixe(&self) -> MPUSEGIXE_R {
        MPUSEGIXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Info memory Segment Violation select"]
    #[inline(always)]
    pub fn mpusegivs(&self) -> MPUSEGIVS_R {
        MPUSEGIVS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Main memory Segment 1 Read enable"]
    #[inline(always)]
    pub fn mpuseg1re(&mut self) -> MPUSEG1RE_W {
        MPUSEG1RE_W { w: self }
    }
    #[doc = "Bit 1 - MPU Main memory Segment 1 Write enable"]
    #[inline(always)]
    pub fn mpuseg1we(&mut self) -> MPUSEG1WE_W {
        MPUSEG1WE_W { w: self }
    }
    #[doc = "Bit 2 - MPU Main memory Segment 1 Execute enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&mut self) -> MPUSEG1XE_W {
        MPUSEG1XE_W { w: self }
    }
    #[doc = "Bit 3 - MPU Main memory Segment 1 Violation select"]
    #[inline(always)]
    pub fn mpuseg1vs(&mut self) -> MPUSEG1VS_W {
        MPUSEG1VS_W { w: self }
    }
    #[doc = "Bit 4 - MPU Main memory Segment 2 Read enable"]
    #[inline(always)]
    pub fn mpuseg2re(&mut self) -> MPUSEG2RE_W {
        MPUSEG2RE_W { w: self }
    }
    #[doc = "Bit 5 - MPU Main memory Segment 2 Write enable"]
    #[inline(always)]
    pub fn mpuseg2we(&mut self) -> MPUSEG2WE_W {
        MPUSEG2WE_W { w: self }
    }
    #[doc = "Bit 6 - MPU Main memory Segment 2 Execute enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&mut self) -> MPUSEG2XE_W {
        MPUSEG2XE_W { w: self }
    }
    #[doc = "Bit 7 - MPU Main memory Segment 2 Violation select"]
    #[inline(always)]
    pub fn mpuseg2vs(&mut self) -> MPUSEG2VS_W {
        MPUSEG2VS_W { w: self }
    }
    #[doc = "Bit 8 - MPU Main memory Segment 3 Read enable"]
    #[inline(always)]
    pub fn mpuseg3re(&mut self) -> MPUSEG3RE_W {
        MPUSEG3RE_W { w: self }
    }
    #[doc = "Bit 9 - MPU Main memory Segment 3 Write enable"]
    #[inline(always)]
    pub fn mpuseg3we(&mut self) -> MPUSEG3WE_W {
        MPUSEG3WE_W { w: self }
    }
    #[doc = "Bit 10 - MPU Main memory Segment 3 Execute enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&mut self) -> MPUSEG3XE_W {
        MPUSEG3XE_W { w: self }
    }
    #[doc = "Bit 11 - MPU Main memory Segment 3 Violation select"]
    #[inline(always)]
    pub fn mpuseg3vs(&mut self) -> MPUSEG3VS_W {
        MPUSEG3VS_W { w: self }
    }
    #[doc = "Bit 12 - MPU Info memory Segment Read enable"]
    #[inline(always)]
    pub fn mpusegire(&mut self) -> MPUSEGIRE_W {
        MPUSEGIRE_W { w: self }
    }
    #[doc = "Bit 13 - MPU Info memory Segment Write enable"]
    #[inline(always)]
    pub fn mpusegiwe(&mut self) -> MPUSEGIWE_W {
        MPUSEGIWE_W { w: self }
    }
    #[doc = "Bit 14 - MPU Info memory Segment Execute enable"]
    #[inline(always)]
    pub fn mpusegixe(&mut self) -> MPUSEGIXE_W {
        MPUSEGIXE_W { w: self }
    }
    #[doc = "Bit 15 - MPU Info memory Segment Violation select"]
    #[inline(always)]
    pub fn mpusegivs(&mut self) -> MPUSEGIVS_W {
        MPUSEGIVS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Access Management Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpusam](index.html) module"]
pub struct MPUSAM_SPEC;
impl crate::RegisterSpec for MPUSAM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpusam::R](R) reader structure"]
impl crate::Readable for MPUSAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpusam::W](W) writer structure"]
impl crate::Writable for MPUSAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUSAM to value 0"]
impl crate::Resettable for MPUSAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
