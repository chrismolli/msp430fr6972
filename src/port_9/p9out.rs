#[doc = "Register `P9OUT` reader"]
pub struct R(crate::R<P9OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P9OUT_SPEC>> for R {
    fn from(reader: crate::R<P9OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9OUT` writer"]
pub struct W(crate::W<P9OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9OUT_SPEC>;
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
impl core::convert::From<crate::W<P9OUT_SPEC>> for W {
    fn from(writer: crate::W<P9OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9OUT0` reader - P9OUT0"]
pub struct P9OUT0_R(crate::FieldReader<bool, bool>);
impl P9OUT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT0` writer - P9OUT0"]
pub struct P9OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `P9OUT1` reader - P9OUT1"]
pub struct P9OUT1_R(crate::FieldReader<bool, bool>);
impl P9OUT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT1` writer - P9OUT1"]
pub struct P9OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `P9OUT2` reader - P9OUT2"]
pub struct P9OUT2_R(crate::FieldReader<bool, bool>);
impl P9OUT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT2` writer - P9OUT2"]
pub struct P9OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `P9OUT3` reader - P9OUT3"]
pub struct P9OUT3_R(crate::FieldReader<bool, bool>);
impl P9OUT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT3` writer - P9OUT3"]
pub struct P9OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `P9OUT4` reader - P9OUT4"]
pub struct P9OUT4_R(crate::FieldReader<bool, bool>);
impl P9OUT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT4` writer - P9OUT4"]
pub struct P9OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `P9OUT5` reader - P9OUT5"]
pub struct P9OUT5_R(crate::FieldReader<bool, bool>);
impl P9OUT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT5` writer - P9OUT5"]
pub struct P9OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `P9OUT6` reader - P9OUT6"]
pub struct P9OUT6_R(crate::FieldReader<bool, bool>);
impl P9OUT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT6` writer - P9OUT6"]
pub struct P9OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `P9OUT7` reader - P9OUT7"]
pub struct P9OUT7_R(crate::FieldReader<bool, bool>);
impl P9OUT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT7` writer - P9OUT7"]
pub struct P9OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&self) -> P9OUT0_R {
        P9OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&self) -> P9OUT1_R {
        P9OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&self) -> P9OUT2_R {
        P9OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&self) -> P9OUT3_R {
        P9OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&self) -> P9OUT4_R {
        P9OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&self) -> P9OUT5_R {
        P9OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&self) -> P9OUT6_R {
        P9OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&self) -> P9OUT7_R {
        P9OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9OUT0"]
    #[inline(always)]
    pub fn p9out0(&mut self) -> P9OUT0_W {
        P9OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P9OUT1"]
    #[inline(always)]
    pub fn p9out1(&mut self) -> P9OUT1_W {
        P9OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P9OUT2"]
    #[inline(always)]
    pub fn p9out2(&mut self) -> P9OUT2_W {
        P9OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P9OUT3"]
    #[inline(always)]
    pub fn p9out3(&mut self) -> P9OUT3_W {
        P9OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P9OUT4"]
    #[inline(always)]
    pub fn p9out4(&mut self) -> P9OUT4_W {
        P9OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P9OUT5"]
    #[inline(always)]
    pub fn p9out5(&mut self) -> P9OUT5_W {
        P9OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P9OUT6"]
    #[inline(always)]
    pub fn p9out6(&mut self) -> P9OUT6_W {
        P9OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P9OUT7"]
    #[inline(always)]
    pub fn p9out7(&mut self) -> P9OUT7_W {
        P9OUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9out](index.html) module"]
pub struct P9OUT_SPEC;
impl crate::RegisterSpec for P9OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9out::R](R) reader structure"]
impl crate::Readable for P9OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9out::W](W) writer structure"]
impl crate::Writable for P9OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9OUT to value 0"]
impl crate::Resettable for P9OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
