#[doc = "Register `LCDCPCTL2` reader"]
pub struct R(crate::R<LCDCPCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCPCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCPCTL2_SPEC>> for R {
    fn from(reader: crate::R<LCDCPCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCPCTL2` writer"]
pub struct W(crate::W<LCDCPCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCPCTL2_SPEC>;
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
impl core::convert::From<crate::W<LCDCPCTL2_SPEC>> for W {
    fn from(writer: crate::W<LCDCPCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS32` reader - LCD Segment 32 enable."]
pub struct LCDS32_R(crate::FieldReader<bool, bool>);
impl LCDS32_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS32` writer - LCD Segment 32 enable."]
pub struct LCDS32_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS32_W<'a> {
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
#[doc = "Field `LCDS33` reader - LCD Segment 33 enable."]
pub struct LCDS33_R(crate::FieldReader<bool, bool>);
impl LCDS33_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS33` writer - LCD Segment 33 enable."]
pub struct LCDS33_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS33_W<'a> {
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
#[doc = "Field `LCDS34` reader - LCD Segment 34 enable."]
pub struct LCDS34_R(crate::FieldReader<bool, bool>);
impl LCDS34_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS34` writer - LCD Segment 34 enable."]
pub struct LCDS34_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS34_W<'a> {
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
#[doc = "Field `LCDS35` reader - LCD Segment 35 enable."]
pub struct LCDS35_R(crate::FieldReader<bool, bool>);
impl LCDS35_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS35` writer - LCD Segment 35 enable."]
pub struct LCDS35_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS35_W<'a> {
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
#[doc = "Field `LCDS36` reader - LCD Segment 36 enable."]
pub struct LCDS36_R(crate::FieldReader<bool, bool>);
impl LCDS36_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS36` writer - LCD Segment 36 enable."]
pub struct LCDS36_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS36_W<'a> {
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
#[doc = "Field `LCDS37` reader - LCD Segment 37 enable."]
pub struct LCDS37_R(crate::FieldReader<bool, bool>);
impl LCDS37_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS37` writer - LCD Segment 37 enable."]
pub struct LCDS37_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS37_W<'a> {
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
#[doc = "Field `LCDS38` reader - LCD Segment 38 enable."]
pub struct LCDS38_R(crate::FieldReader<bool, bool>);
impl LCDS38_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS38` writer - LCD Segment 38 enable."]
pub struct LCDS38_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS38_W<'a> {
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
#[doc = "Field `LCDS39` reader - LCD Segment 39 enable."]
pub struct LCDS39_R(crate::FieldReader<bool, bool>);
impl LCDS39_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS39` writer - LCD Segment 39 enable."]
pub struct LCDS39_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS39_W<'a> {
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
#[doc = "Field `LCDS40` reader - LCD Segment 40 enable."]
pub struct LCDS40_R(crate::FieldReader<bool, bool>);
impl LCDS40_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS40` writer - LCD Segment 40 enable."]
pub struct LCDS40_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS40_W<'a> {
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
#[doc = "Field `LCDS41` reader - LCD Segment 41 enable."]
pub struct LCDS41_R(crate::FieldReader<bool, bool>);
impl LCDS41_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS41` writer - LCD Segment 41 enable."]
pub struct LCDS41_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS41_W<'a> {
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
#[doc = "Field `LCDS42` reader - LCD Segment 42 enable."]
pub struct LCDS42_R(crate::FieldReader<bool, bool>);
impl LCDS42_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS42` writer - LCD Segment 42 enable."]
pub struct LCDS42_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS42_W<'a> {
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
#[doc = "Field `LCDS43` reader - LCD Segment 43 enable."]
pub struct LCDS43_R(crate::FieldReader<bool, bool>);
impl LCDS43_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS43` writer - LCD Segment 43 enable."]
pub struct LCDS43_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS43_W<'a> {
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
#[doc = "Field `LCDS44` reader - LCD Segment 44 enable."]
pub struct LCDS44_R(crate::FieldReader<bool, bool>);
impl LCDS44_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS44` writer - LCD Segment 44 enable."]
pub struct LCDS44_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS44_W<'a> {
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
#[doc = "Field `LCDS45` reader - LCD Segment 45 enable."]
pub struct LCDS45_R(crate::FieldReader<bool, bool>);
impl LCDS45_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS45` writer - LCD Segment 45 enable."]
pub struct LCDS45_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS45_W<'a> {
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
#[doc = "Field `LCDS46` reader - LCD Segment 46 enable."]
pub struct LCDS46_R(crate::FieldReader<bool, bool>);
impl LCDS46_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS46` writer - LCD Segment 46 enable."]
pub struct LCDS46_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS46_W<'a> {
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
#[doc = "Field `LCDS47` reader - LCD Segment 47 enable."]
pub struct LCDS47_R(crate::FieldReader<bool, bool>);
impl LCDS47_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDS47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS47` writer - LCD Segment 47 enable."]
pub struct LCDS47_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS47_W<'a> {
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
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&self) -> LCDS32_R {
        LCDS32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&self) -> LCDS33_R {
        LCDS33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&self) -> LCDS34_R {
        LCDS34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&self) -> LCDS35_R {
        LCDS35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&self) -> LCDS36_R {
        LCDS36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&self) -> LCDS37_R {
        LCDS37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&self) -> LCDS38_R {
        LCDS38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&self) -> LCDS39_R {
        LCDS39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&self) -> LCDS40_R {
        LCDS40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&self) -> LCDS41_R {
        LCDS41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&self) -> LCDS42_R {
        LCDS42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&self) -> LCDS43_R {
        LCDS43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&self) -> LCDS44_R {
        LCDS44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&self) -> LCDS45_R {
        LCDS45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&self) -> LCDS46_R {
        LCDS46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&self) -> LCDS47_R {
        LCDS47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Segment 32 enable."]
    #[inline(always)]
    pub fn lcds32(&mut self) -> LCDS32_W {
        LCDS32_W { w: self }
    }
    #[doc = "Bit 1 - LCD Segment 33 enable."]
    #[inline(always)]
    pub fn lcds33(&mut self) -> LCDS33_W {
        LCDS33_W { w: self }
    }
    #[doc = "Bit 2 - LCD Segment 34 enable."]
    #[inline(always)]
    pub fn lcds34(&mut self) -> LCDS34_W {
        LCDS34_W { w: self }
    }
    #[doc = "Bit 3 - LCD Segment 35 enable."]
    #[inline(always)]
    pub fn lcds35(&mut self) -> LCDS35_W {
        LCDS35_W { w: self }
    }
    #[doc = "Bit 4 - LCD Segment 36 enable."]
    #[inline(always)]
    pub fn lcds36(&mut self) -> LCDS36_W {
        LCDS36_W { w: self }
    }
    #[doc = "Bit 5 - LCD Segment 37 enable."]
    #[inline(always)]
    pub fn lcds37(&mut self) -> LCDS37_W {
        LCDS37_W { w: self }
    }
    #[doc = "Bit 6 - LCD Segment 38 enable."]
    #[inline(always)]
    pub fn lcds38(&mut self) -> LCDS38_W {
        LCDS38_W { w: self }
    }
    #[doc = "Bit 7 - LCD Segment 39 enable."]
    #[inline(always)]
    pub fn lcds39(&mut self) -> LCDS39_W {
        LCDS39_W { w: self }
    }
    #[doc = "Bit 8 - LCD Segment 40 enable."]
    #[inline(always)]
    pub fn lcds40(&mut self) -> LCDS40_W {
        LCDS40_W { w: self }
    }
    #[doc = "Bit 9 - LCD Segment 41 enable."]
    #[inline(always)]
    pub fn lcds41(&mut self) -> LCDS41_W {
        LCDS41_W { w: self }
    }
    #[doc = "Bit 10 - LCD Segment 42 enable."]
    #[inline(always)]
    pub fn lcds42(&mut self) -> LCDS42_W {
        LCDS42_W { w: self }
    }
    #[doc = "Bit 11 - LCD Segment 43 enable."]
    #[inline(always)]
    pub fn lcds43(&mut self) -> LCDS43_W {
        LCDS43_W { w: self }
    }
    #[doc = "Bit 12 - LCD Segment 44 enable."]
    #[inline(always)]
    pub fn lcds44(&mut self) -> LCDS44_W {
        LCDS44_W { w: self }
    }
    #[doc = "Bit 13 - LCD Segment 45 enable."]
    #[inline(always)]
    pub fn lcds45(&mut self) -> LCDS45_W {
        LCDS45_W { w: self }
    }
    #[doc = "Bit 14 - LCD Segment 46 enable."]
    #[inline(always)]
    pub fn lcds46(&mut self) -> LCDS46_W {
        LCDS46_W { w: self }
    }
    #[doc = "Bit 15 - LCD Segment 47 enable."]
    #[inline(always)]
    pub fn lcds47(&mut self) -> LCDS47_W {
        LCDS47_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl2](index.html) module"]
pub struct LCDCPCTL2_SPEC;
impl crate::RegisterSpec for LCDCPCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcpctl2::R](R) reader structure"]
impl crate::Readable for LCDCPCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcpctl2::W](W) writer structure"]
impl crate::Writable for LCDCPCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCPCTL2 to value 0"]
impl crate::Resettable for LCDCPCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
