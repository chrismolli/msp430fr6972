#[doc = "Register `PJSEL0` reader"]
pub struct R(crate::R<PJSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PJSEL0_SPEC>> for R {
    fn from(reader: crate::R<PJSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL0` writer"]
pub struct W(crate::W<PJSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL0_SPEC>;
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
impl core::convert::From<crate::W<PJSEL0_SPEC>> for W {
    fn from(writer: crate::W<PJSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL0_0` reader - PJSEL0_0"]
pub struct PJSEL0_0_R(crate::FieldReader<bool, bool>);
impl PJSEL0_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_0` writer - PJSEL0_0"]
pub struct PJSEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_0_W<'a> {
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
#[doc = "Field `PJSEL0_1` reader - PJSEL0_1"]
pub struct PJSEL0_1_R(crate::FieldReader<bool, bool>);
impl PJSEL0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_1` writer - PJSEL0_1"]
pub struct PJSEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_1_W<'a> {
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
#[doc = "Field `PJSEL0_2` reader - PJSEL0_2"]
pub struct PJSEL0_2_R(crate::FieldReader<bool, bool>);
impl PJSEL0_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_2` writer - PJSEL0_2"]
pub struct PJSEL0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_2_W<'a> {
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
#[doc = "Field `PJSEL0_3` reader - PJSEL0_3"]
pub struct PJSEL0_3_R(crate::FieldReader<bool, bool>);
impl PJSEL0_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_3` writer - PJSEL0_3"]
pub struct PJSEL0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_3_W<'a> {
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
#[doc = "Field `PJSEL0_4` reader - PJSEL0_4"]
pub struct PJSEL0_4_R(crate::FieldReader<bool, bool>);
impl PJSEL0_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_4` writer - PJSEL0_4"]
pub struct PJSEL0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_4_W<'a> {
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
#[doc = "Field `PJSEL0_5` reader - PJSEL0_5"]
pub struct PJSEL0_5_R(crate::FieldReader<bool, bool>);
impl PJSEL0_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_5` writer - PJSEL0_5"]
pub struct PJSEL0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_5_W<'a> {
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
#[doc = "Field `PJSEL0_6` reader - PJSEL0_6"]
pub struct PJSEL0_6_R(crate::FieldReader<bool, bool>);
impl PJSEL0_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_6` writer - PJSEL0_6"]
pub struct PJSEL0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_6_W<'a> {
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
#[doc = "Field `PJSEL0_7` reader - PJSEL0_7"]
pub struct PJSEL0_7_R(crate::FieldReader<bool, bool>);
impl PJSEL0_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJSEL0_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL0_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL0_7` writer - PJSEL0_7"]
pub struct PJSEL0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL0_7_W<'a> {
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
    #[doc = "Bit 0 - PJSEL0_0"]
    #[inline(always)]
    pub fn pjsel0_0(&self) -> PJSEL0_0_R {
        PJSEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJSEL0_1"]
    #[inline(always)]
    pub fn pjsel0_1(&self) -> PJSEL0_1_R {
        PJSEL0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJSEL0_2"]
    #[inline(always)]
    pub fn pjsel0_2(&self) -> PJSEL0_2_R {
        PJSEL0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJSEL0_3"]
    #[inline(always)]
    pub fn pjsel0_3(&self) -> PJSEL0_3_R {
        PJSEL0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJSEL0_4"]
    #[inline(always)]
    pub fn pjsel0_4(&self) -> PJSEL0_4_R {
        PJSEL0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJSEL0_5"]
    #[inline(always)]
    pub fn pjsel0_5(&self) -> PJSEL0_5_R {
        PJSEL0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJSEL0_6"]
    #[inline(always)]
    pub fn pjsel0_6(&self) -> PJSEL0_6_R {
        PJSEL0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJSEL0_7"]
    #[inline(always)]
    pub fn pjsel0_7(&self) -> PJSEL0_7_R {
        PJSEL0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJSEL0_0"]
    #[inline(always)]
    pub fn pjsel0_0(&mut self) -> PJSEL0_0_W {
        PJSEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - PJSEL0_1"]
    #[inline(always)]
    pub fn pjsel0_1(&mut self) -> PJSEL0_1_W {
        PJSEL0_1_W { w: self }
    }
    #[doc = "Bit 2 - PJSEL0_2"]
    #[inline(always)]
    pub fn pjsel0_2(&mut self) -> PJSEL0_2_W {
        PJSEL0_2_W { w: self }
    }
    #[doc = "Bit 3 - PJSEL0_3"]
    #[inline(always)]
    pub fn pjsel0_3(&mut self) -> PJSEL0_3_W {
        PJSEL0_3_W { w: self }
    }
    #[doc = "Bit 4 - PJSEL0_4"]
    #[inline(always)]
    pub fn pjsel0_4(&mut self) -> PJSEL0_4_W {
        PJSEL0_4_W { w: self }
    }
    #[doc = "Bit 5 - PJSEL0_5"]
    #[inline(always)]
    pub fn pjsel0_5(&mut self) -> PJSEL0_5_W {
        PJSEL0_5_W { w: self }
    }
    #[doc = "Bit 6 - PJSEL0_6"]
    #[inline(always)]
    pub fn pjsel0_6(&mut self) -> PJSEL0_6_W {
        PJSEL0_6_W { w: self }
    }
    #[doc = "Bit 7 - PJSEL0_7"]
    #[inline(always)]
    pub fn pjsel0_7(&mut self) -> PJSEL0_7_W {
        PJSEL0_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel0](index.html) module"]
pub struct PJSEL0_SPEC;
impl crate::RegisterSpec for PJSEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel0::R](R) reader structure"]
impl crate::Readable for PJSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel0::W](W) writer structure"]
impl crate::Writable for PJSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL0 to value 0"]
impl crate::Resettable for PJSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
