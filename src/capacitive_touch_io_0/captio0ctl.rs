#[doc = "Register `CAPTIO0CTL` reader"]
pub struct R(crate::R<CAPTIO0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTIO0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAPTIO0CTL_SPEC>> for R {
    fn from(reader: crate::R<CAPTIO0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPTIO0CTL` writer"]
pub struct W(crate::W<CAPTIO0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTIO0CTL_SPEC>;
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
impl core::convert::From<crate::W<CAPTIO0CTL_SPEC>> for W {
    fn from(writer: crate::W<CAPTIO0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTIOPISEL0` reader - CapTouchIO Pin Select Bit: 0"]
pub struct CAPTIOPISEL0_R(crate::FieldReader<bool, bool>);
impl CAPTIOPISEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPISEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPISEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPISEL0` writer - CapTouchIO Pin Select Bit: 0"]
pub struct CAPTIOPISEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL0_W<'a> {
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
#[doc = "Field `CAPTIOPISEL1` reader - CapTouchIO Pin Select Bit: 1"]
pub struct CAPTIOPISEL1_R(crate::FieldReader<bool, bool>);
impl CAPTIOPISEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPISEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPISEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPISEL1` writer - CapTouchIO Pin Select Bit: 1"]
pub struct CAPTIOPISEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL1_W<'a> {
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
#[doc = "Field `CAPTIOPISEL2` reader - CapTouchIO Pin Select Bit: 2"]
pub struct CAPTIOPISEL2_R(crate::FieldReader<bool, bool>);
impl CAPTIOPISEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPISEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPISEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPISEL2` writer - CapTouchIO Pin Select Bit: 2"]
pub struct CAPTIOPISEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPISEL2_W<'a> {
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
#[doc = "Field `CAPTIOPOSEL0` reader - CapTouchIO Port Select Bit: 0"]
pub struct CAPTIOPOSEL0_R(crate::FieldReader<bool, bool>);
impl CAPTIOPOSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPOSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPOSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPOSEL0` writer - CapTouchIO Port Select Bit: 0"]
pub struct CAPTIOPOSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL0_W<'a> {
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
#[doc = "Field `CAPTIOPOSEL1` reader - CapTouchIO Port Select Bit: 1"]
pub struct CAPTIOPOSEL1_R(crate::FieldReader<bool, bool>);
impl CAPTIOPOSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPOSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPOSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPOSEL1` writer - CapTouchIO Port Select Bit: 1"]
pub struct CAPTIOPOSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL1_W<'a> {
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
#[doc = "Field `CAPTIOPOSEL2` reader - CapTouchIO Port Select Bit: 2"]
pub struct CAPTIOPOSEL2_R(crate::FieldReader<bool, bool>);
impl CAPTIOPOSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPOSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPOSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPOSEL2` writer - CapTouchIO Port Select Bit: 2"]
pub struct CAPTIOPOSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL2_W<'a> {
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
#[doc = "Field `CAPTIOPOSEL3` reader - CapTouchIO Port Select Bit: 3"]
pub struct CAPTIOPOSEL3_R(crate::FieldReader<bool, bool>);
impl CAPTIOPOSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOPOSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOPOSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOPOSEL3` writer - CapTouchIO Port Select Bit: 3"]
pub struct CAPTIOPOSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOPOSEL3_W<'a> {
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
#[doc = "Field `CAPTIOEN` reader - CapTouchIO Enable"]
pub struct CAPTIOEN_R(crate::FieldReader<bool, bool>);
impl CAPTIOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIOEN` writer - CapTouchIO Enable"]
pub struct CAPTIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIOEN_W<'a> {
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
#[doc = "Field `CAPTIO` reader - CapTouchIO state"]
pub struct CAPTIO_R(crate::FieldReader<bool, bool>);
impl CAPTIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTIO` writer - CapTouchIO state"]
pub struct CAPTIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTIO_W<'a> {
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
impl R {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&self) -> CAPTIOPISEL0_R {
        CAPTIOPISEL0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&self) -> CAPTIOPISEL1_R {
        CAPTIOPISEL1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&self) -> CAPTIOPISEL2_R {
        CAPTIOPISEL2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&self) -> CAPTIOPOSEL0_R {
        CAPTIOPOSEL0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&self) -> CAPTIOPOSEL1_R {
        CAPTIOPOSEL1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&self) -> CAPTIOPOSEL2_R {
        CAPTIOPOSEL2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&self) -> CAPTIOPOSEL3_R {
        CAPTIOPOSEL3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CAPTIOEN_R {
        CAPTIOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&self) -> CAPTIO_R {
        CAPTIO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CapTouchIO Pin Select Bit: 0"]
    #[inline(always)]
    pub fn captiopisel0(&mut self) -> CAPTIOPISEL0_W {
        CAPTIOPISEL0_W { w: self }
    }
    #[doc = "Bit 2 - CapTouchIO Pin Select Bit: 1"]
    #[inline(always)]
    pub fn captiopisel1(&mut self) -> CAPTIOPISEL1_W {
        CAPTIOPISEL1_W { w: self }
    }
    #[doc = "Bit 3 - CapTouchIO Pin Select Bit: 2"]
    #[inline(always)]
    pub fn captiopisel2(&mut self) -> CAPTIOPISEL2_W {
        CAPTIOPISEL2_W { w: self }
    }
    #[doc = "Bit 4 - CapTouchIO Port Select Bit: 0"]
    #[inline(always)]
    pub fn captioposel0(&mut self) -> CAPTIOPOSEL0_W {
        CAPTIOPOSEL0_W { w: self }
    }
    #[doc = "Bit 5 - CapTouchIO Port Select Bit: 1"]
    #[inline(always)]
    pub fn captioposel1(&mut self) -> CAPTIOPOSEL1_W {
        CAPTIOPOSEL1_W { w: self }
    }
    #[doc = "Bit 6 - CapTouchIO Port Select Bit: 2"]
    #[inline(always)]
    pub fn captioposel2(&mut self) -> CAPTIOPOSEL2_W {
        CAPTIOPOSEL2_W { w: self }
    }
    #[doc = "Bit 7 - CapTouchIO Port Select Bit: 3"]
    #[inline(always)]
    pub fn captioposel3(&mut self) -> CAPTIOPOSEL3_W {
        CAPTIOPOSEL3_W { w: self }
    }
    #[doc = "Bit 8 - CapTouchIO Enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CAPTIOEN_W {
        CAPTIOEN_W { w: self }
    }
    #[doc = "Bit 9 - CapTouchIO state"]
    #[inline(always)]
    pub fn captio(&mut self) -> CAPTIO_W {
        CAPTIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capacitive_Touch_IO 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captio0ctl](index.html) module"]
pub struct CAPTIO0CTL_SPEC;
impl crate::RegisterSpec for CAPTIO0CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [captio0ctl::R](R) reader structure"]
impl crate::Readable for CAPTIO0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [captio0ctl::W](W) writer structure"]
impl crate::Writable for CAPTIO0CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPTIO0CTL to value 0"]
impl crate::Resettable for CAPTIO0CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
