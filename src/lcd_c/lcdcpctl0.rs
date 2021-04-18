#[doc = "Register `LCDCPCTL0` reader"]
pub struct R(crate::R<LCDCPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCPCTL0_SPEC>> for R {
    fn from(reader: crate::R<LCDCPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCPCTL0` writer"]
pub struct W(crate::W<LCDCPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCPCTL0_SPEC>;
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
impl core::convert::From<crate::W<LCDCPCTL0_SPEC>> for W {
    fn from(writer: crate::W<LCDCPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS0` reader - LCD Segment 0 enable."]
pub struct LCDS0_R(crate::FieldReader<bool, bool>);
impl LCDS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS0` writer - LCD Segment 0 enable."]
pub struct LCDS0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS0_W<'a> {
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
#[doc = "Field `LCDS1` reader - LCD Segment 1 enable."]
pub struct LCDS1_R(crate::FieldReader<bool, bool>);
impl LCDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS1` writer - LCD Segment 1 enable."]
pub struct LCDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS1_W<'a> {
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
#[doc = "Field `LCDS2` reader - LCD Segment 2 enable."]
pub struct LCDS2_R(crate::FieldReader<bool, bool>);
impl LCDS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS2` writer - LCD Segment 2 enable."]
pub struct LCDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS2_W<'a> {
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
#[doc = "Field `LCDS3` reader - LCD Segment 3 enable."]
pub struct LCDS3_R(crate::FieldReader<bool, bool>);
impl LCDS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS3` writer - LCD Segment 3 enable."]
pub struct LCDS3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS3_W<'a> {
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
#[doc = "Field `LCDS4` reader - LCD Segment 4 enable."]
pub struct LCDS4_R(crate::FieldReader<bool, bool>);
impl LCDS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS4` writer - LCD Segment 4 enable."]
pub struct LCDS4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS4_W<'a> {
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
#[doc = "Field `LCDS5` reader - LCD Segment 5 enable."]
pub struct LCDS5_R(crate::FieldReader<bool, bool>);
impl LCDS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS5` writer - LCD Segment 5 enable."]
pub struct LCDS5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS5_W<'a> {
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
#[doc = "Field `LCDS6` reader - LCD Segment 6 enable."]
pub struct LCDS6_R(crate::FieldReader<bool, bool>);
impl LCDS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS6` writer - LCD Segment 6 enable."]
pub struct LCDS6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS6_W<'a> {
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
#[doc = "Field `LCDS7` reader - LCD Segment 7 enable."]
pub struct LCDS7_R(crate::FieldReader<bool, bool>);
impl LCDS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS7` writer - LCD Segment 7 enable."]
pub struct LCDS7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS7_W<'a> {
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
#[doc = "Field `LCDS8` reader - LCD Segment 8 enable."]
pub struct LCDS8_R(crate::FieldReader<bool, bool>);
impl LCDS8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS8` writer - LCD Segment 8 enable."]
pub struct LCDS8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS8_W<'a> {
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
#[doc = "Field `LCDS9` reader - LCD Segment 9 enable."]
pub struct LCDS9_R(crate::FieldReader<bool, bool>);
impl LCDS9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS9` writer - LCD Segment 9 enable."]
pub struct LCDS9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS9_W<'a> {
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
#[doc = "Field `LCDS10` reader - LCD Segment 10 enable."]
pub struct LCDS10_R(crate::FieldReader<bool, bool>);
impl LCDS10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS10` writer - LCD Segment 10 enable."]
pub struct LCDS10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS10_W<'a> {
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
#[doc = "Field `LCDS11` reader - LCD Segment 11 enable."]
pub struct LCDS11_R(crate::FieldReader<bool, bool>);
impl LCDS11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS11` writer - LCD Segment 11 enable."]
pub struct LCDS11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS11_W<'a> {
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
#[doc = "Field `LCDS12` reader - LCD Segment 12 enable."]
pub struct LCDS12_R(crate::FieldReader<bool, bool>);
impl LCDS12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS12` writer - LCD Segment 12 enable."]
pub struct LCDS12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS12_W<'a> {
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
#[doc = "Field `LCDS13` reader - LCD Segment 13 enable."]
pub struct LCDS13_R(crate::FieldReader<bool, bool>);
impl LCDS13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS13` writer - LCD Segment 13 enable."]
pub struct LCDS13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS13_W<'a> {
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
#[doc = "Field `LCDS14` reader - LCD Segment 14 enable."]
pub struct LCDS14_R(crate::FieldReader<bool, bool>);
impl LCDS14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS14` writer - LCD Segment 14 enable."]
pub struct LCDS14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS14_W<'a> {
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
#[doc = "Field `LCDS15` reader - LCD Segment 15 enable."]
pub struct LCDS15_R(crate::FieldReader<bool, bool>);
impl LCDS15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS15` writer - LCD Segment 15 enable."]
pub struct LCDS15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS15_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&self) -> LCDS0_R {
        LCDS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&self) -> LCDS1_R {
        LCDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&self) -> LCDS2_R {
        LCDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&self) -> LCDS3_R {
        LCDS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&self) -> LCDS4_R {
        LCDS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&self) -> LCDS5_R {
        LCDS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&self) -> LCDS6_R {
        LCDS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&self) -> LCDS7_R {
        LCDS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&self) -> LCDS8_R {
        LCDS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&self) -> LCDS9_R {
        LCDS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&self) -> LCDS10_R {
        LCDS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&self) -> LCDS11_R {
        LCDS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&self) -> LCDS12_R {
        LCDS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&self) -> LCDS13_R {
        LCDS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&self) -> LCDS14_R {
        LCDS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&self) -> LCDS15_R {
        LCDS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 0 enable."]
    #[inline(always)]
    pub fn lcds0(&mut self) -> LCDS0_W {
        LCDS0_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 1 enable."]
    #[inline(always)]
    pub fn lcds1(&mut self) -> LCDS1_W {
        LCDS1_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 2 enable."]
    #[inline(always)]
    pub fn lcds2(&mut self) -> LCDS2_W {
        LCDS2_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 3 enable."]
    #[inline(always)]
    pub fn lcds3(&mut self) -> LCDS3_W {
        LCDS3_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 4 enable."]
    #[inline(always)]
    pub fn lcds4(&mut self) -> LCDS4_W {
        LCDS4_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 5 enable."]
    #[inline(always)]
    pub fn lcds5(&mut self) -> LCDS5_W {
        LCDS5_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 6 enable."]
    #[inline(always)]
    pub fn lcds6(&mut self) -> LCDS6_W {
        LCDS6_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 7 enable."]
    #[inline(always)]
    pub fn lcds7(&mut self) -> LCDS7_W {
        LCDS7_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 8 enable."]
    #[inline(always)]
    pub fn lcds8(&mut self) -> LCDS8_W {
        LCDS8_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 9 enable."]
    #[inline(always)]
    pub fn lcds9(&mut self) -> LCDS9_W {
        LCDS9_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 10 enable."]
    #[inline(always)]
    pub fn lcds10(&mut self) -> LCDS10_W {
        LCDS10_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 11 enable."]
    #[inline(always)]
    pub fn lcds11(&mut self) -> LCDS11_W {
        LCDS11_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 12 enable."]
    #[inline(always)]
    pub fn lcds12(&mut self) -> LCDS12_W {
        LCDS12_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 13 enable."]
    #[inline(always)]
    pub fn lcds13(&mut self) -> LCDS13_W {
        LCDS13_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 14 enable."]
    #[inline(always)]
    pub fn lcds14(&mut self) -> LCDS14_W {
        LCDS14_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 15 enable."]
    #[inline(always)]
    pub fn lcds15(&mut self) -> LCDS15_W {
        LCDS15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Port Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl0](index.html) module"]
pub struct LCDCPCTL0_SPEC;
impl crate::RegisterSpec for LCDCPCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcpctl0::R](R) reader structure"]
impl crate::Readable for LCDCPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcpctl0::W](W) writer structure"]
impl crate::Writable for LCDCPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCPCTL0 to value 0"]
impl crate::Resettable for LCDCPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
