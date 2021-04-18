#[doc = "Register `PJSEL1` reader"]
pub struct R(crate::R<PJSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PJSEL1_SPEC>> for R {
    fn from(reader: crate::R<PJSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL1` writer"]
pub struct W(crate::W<PJSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL1_SPEC>;
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
impl core::convert::From<crate::W<PJSEL1_SPEC>> for W {
    fn from(writer: crate::W<PJSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL1_0` reader - PJSEL1_0"]
pub struct PJSEL1_0_R(crate::FieldReader<bool, bool>);
impl PJSEL1_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_0` writer - PJSEL1_0"]
pub struct PJSEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_0_W<'a> {
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
#[doc = "Field `PJSEL1_1` reader - PJSEL1_1"]
pub struct PJSEL1_1_R(crate::FieldReader<bool, bool>);
impl PJSEL1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_1` writer - PJSEL1_1"]
pub struct PJSEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_1_W<'a> {
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
#[doc = "Field `PJSEL1_2` reader - PJSEL1_2"]
pub struct PJSEL1_2_R(crate::FieldReader<bool, bool>);
impl PJSEL1_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_2` writer - PJSEL1_2"]
pub struct PJSEL1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_2_W<'a> {
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
#[doc = "Field `PJSEL1_3` reader - PJSEL1_3"]
pub struct PJSEL1_3_R(crate::FieldReader<bool, bool>);
impl PJSEL1_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_3` writer - PJSEL1_3"]
pub struct PJSEL1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_3_W<'a> {
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
#[doc = "Field `PJSEL1_4` reader - PJSEL1_4"]
pub struct PJSEL1_4_R(crate::FieldReader<bool, bool>);
impl PJSEL1_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_4` writer - PJSEL1_4"]
pub struct PJSEL1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_4_W<'a> {
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
#[doc = "Field `PJSEL1_5` reader - PJSEL1_5"]
pub struct PJSEL1_5_R(crate::FieldReader<bool, bool>);
impl PJSEL1_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_5` writer - PJSEL1_5"]
pub struct PJSEL1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_5_W<'a> {
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
#[doc = "Field `PJSEL1_6` reader - PJSEL1_6"]
pub struct PJSEL1_6_R(crate::FieldReader<bool, bool>);
impl PJSEL1_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_6` writer - PJSEL1_6"]
pub struct PJSEL1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_6_W<'a> {
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
#[doc = "Field `PJSEL1_7` reader - PJSEL1_7"]
pub struct PJSEL1_7_R(crate::FieldReader<bool, bool>);
impl PJSEL1_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL1_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1_7` writer - PJSEL1_7"]
pub struct PJSEL1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PJSEL1_0"]
    #[inline(always)]
    pub fn pjsel1_0(&self) -> PJSEL1_0_R {
        PJSEL1_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJSEL1_1"]
    #[inline(always)]
    pub fn pjsel1_1(&self) -> PJSEL1_1_R {
        PJSEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJSEL1_2"]
    #[inline(always)]
    pub fn pjsel1_2(&self) -> PJSEL1_2_R {
        PJSEL1_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJSEL1_3"]
    #[inline(always)]
    pub fn pjsel1_3(&self) -> PJSEL1_3_R {
        PJSEL1_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJSEL1_4"]
    #[inline(always)]
    pub fn pjsel1_4(&self) -> PJSEL1_4_R {
        PJSEL1_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJSEL1_5"]
    #[inline(always)]
    pub fn pjsel1_5(&self) -> PJSEL1_5_R {
        PJSEL1_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJSEL1_6"]
    #[inline(always)]
    pub fn pjsel1_6(&self) -> PJSEL1_6_R {
        PJSEL1_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJSEL1_7"]
    #[inline(always)]
    pub fn pjsel1_7(&self) -> PJSEL1_7_R {
        PJSEL1_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJSEL1_0"]
    #[inline(always)]
    pub fn pjsel1_0(&mut self) -> PJSEL1_0_W {
        PJSEL1_0_W { w: self }
    }
    #[doc = "Bit 1 - PJSEL1_1"]
    #[inline(always)]
    pub fn pjsel1_1(&mut self) -> PJSEL1_1_W {
        PJSEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - PJSEL1_2"]
    #[inline(always)]
    pub fn pjsel1_2(&mut self) -> PJSEL1_2_W {
        PJSEL1_2_W { w: self }
    }
    #[doc = "Bit 3 - PJSEL1_3"]
    #[inline(always)]
    pub fn pjsel1_3(&mut self) -> PJSEL1_3_W {
        PJSEL1_3_W { w: self }
    }
    #[doc = "Bit 4 - PJSEL1_4"]
    #[inline(always)]
    pub fn pjsel1_4(&mut self) -> PJSEL1_4_W {
        PJSEL1_4_W { w: self }
    }
    #[doc = "Bit 5 - PJSEL1_5"]
    #[inline(always)]
    pub fn pjsel1_5(&mut self) -> PJSEL1_5_W {
        PJSEL1_5_W { w: self }
    }
    #[doc = "Bit 6 - PJSEL1_6"]
    #[inline(always)]
    pub fn pjsel1_6(&mut self) -> PJSEL1_6_W {
        PJSEL1_6_W { w: self }
    }
    #[doc = "Bit 7 - PJSEL1_7"]
    #[inline(always)]
    pub fn pjsel1_7(&mut self) -> PJSEL1_7_W {
        PJSEL1_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel1](index.html) module"]
pub struct PJSEL1_SPEC;
impl crate::RegisterSpec for PJSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel1::R](R) reader structure"]
impl crate::Readable for PJSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel1::W](W) writer structure"]
impl crate::Writable for PJSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL1 to value 0"]
impl crate::Resettable for PJSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
