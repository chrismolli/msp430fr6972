#[doc = "Register `P1SELC` reader"]
pub struct R(crate::R<P1SELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1SELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P1SELC_SPEC>> for R {
    fn from(reader: crate::R<P1SELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1SELC` writer"]
pub struct W(crate::W<P1SELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1SELC_SPEC>;
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
impl core::convert::From<crate::W<P1SELC_SPEC>> for W {
    fn from(writer: crate::W<P1SELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1SELC_0` reader - P1SELC_0"]
pub struct P1SELC_0_R(crate::FieldReader<bool, bool>);
impl P1SELC_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_0` writer - P1SELC_0"]
pub struct P1SELC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_0_W<'a> {
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
#[doc = "Field `P1SELC_1` reader - P1SELC_1"]
pub struct P1SELC_1_R(crate::FieldReader<bool, bool>);
impl P1SELC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_1` writer - P1SELC_1"]
pub struct P1SELC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_1_W<'a> {
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
#[doc = "Field `P1SELC_2` reader - P1SELC_2"]
pub struct P1SELC_2_R(crate::FieldReader<bool, bool>);
impl P1SELC_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_2` writer - P1SELC_2"]
pub struct P1SELC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_2_W<'a> {
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
#[doc = "Field `P1SELC_3` reader - P1SELC_3"]
pub struct P1SELC_3_R(crate::FieldReader<bool, bool>);
impl P1SELC_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_3` writer - P1SELC_3"]
pub struct P1SELC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_3_W<'a> {
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
#[doc = "Field `P1SELC_4` reader - P1SELC_4"]
pub struct P1SELC_4_R(crate::FieldReader<bool, bool>);
impl P1SELC_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_4` writer - P1SELC_4"]
pub struct P1SELC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_4_W<'a> {
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
#[doc = "Field `P1SELC_5` reader - P1SELC_5"]
pub struct P1SELC_5_R(crate::FieldReader<bool, bool>);
impl P1SELC_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_5` writer - P1SELC_5"]
pub struct P1SELC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_5_W<'a> {
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
#[doc = "Field `P1SELC_6` reader - P1SELC_6"]
pub struct P1SELC_6_R(crate::FieldReader<bool, bool>);
impl P1SELC_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_6` writer - P1SELC_6"]
pub struct P1SELC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_6_W<'a> {
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
#[doc = "Field `P1SELC_7` reader - P1SELC_7"]
pub struct P1SELC_7_R(crate::FieldReader<bool, bool>);
impl P1SELC_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1SELC_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1SELC_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1SELC_7` writer - P1SELC_7"]
pub struct P1SELC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1SELC_7_W<'a> {
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
    #[doc = "Bit 0 - P1SELC_0"]
    #[inline(always)]
    pub fn p1selc_0(&self) -> P1SELC_0_R {
        P1SELC_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1SELC_1"]
    #[inline(always)]
    pub fn p1selc_1(&self) -> P1SELC_1_R {
        P1SELC_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1SELC_2"]
    #[inline(always)]
    pub fn p1selc_2(&self) -> P1SELC_2_R {
        P1SELC_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1SELC_3"]
    #[inline(always)]
    pub fn p1selc_3(&self) -> P1SELC_3_R {
        P1SELC_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1SELC_4"]
    #[inline(always)]
    pub fn p1selc_4(&self) -> P1SELC_4_R {
        P1SELC_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1SELC_5"]
    #[inline(always)]
    pub fn p1selc_5(&self) -> P1SELC_5_R {
        P1SELC_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1SELC_6"]
    #[inline(always)]
    pub fn p1selc_6(&self) -> P1SELC_6_R {
        P1SELC_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1SELC_7"]
    #[inline(always)]
    pub fn p1selc_7(&self) -> P1SELC_7_R {
        P1SELC_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SELC_0"]
    #[inline(always)]
    pub fn p1selc_0(&mut self) -> P1SELC_0_W {
        P1SELC_0_W { w: self }
    }
    #[doc = "Bit 1 - P1SELC_1"]
    #[inline(always)]
    pub fn p1selc_1(&mut self) -> P1SELC_1_W {
        P1SELC_1_W { w: self }
    }
    #[doc = "Bit 2 - P1SELC_2"]
    #[inline(always)]
    pub fn p1selc_2(&mut self) -> P1SELC_2_W {
        P1SELC_2_W { w: self }
    }
    #[doc = "Bit 3 - P1SELC_3"]
    #[inline(always)]
    pub fn p1selc_3(&mut self) -> P1SELC_3_W {
        P1SELC_3_W { w: self }
    }
    #[doc = "Bit 4 - P1SELC_4"]
    #[inline(always)]
    pub fn p1selc_4(&mut self) -> P1SELC_4_W {
        P1SELC_4_W { w: self }
    }
    #[doc = "Bit 5 - P1SELC_5"]
    #[inline(always)]
    pub fn p1selc_5(&mut self) -> P1SELC_5_W {
        P1SELC_5_W { w: self }
    }
    #[doc = "Bit 6 - P1SELC_6"]
    #[inline(always)]
    pub fn p1selc_6(&mut self) -> P1SELC_6_W {
        P1SELC_6_W { w: self }
    }
    #[doc = "Bit 7 - P1SELC_7"]
    #[inline(always)]
    pub fn p1selc_7(&mut self) -> P1SELC_7_W {
        P1SELC_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Complement Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1selc](index.html) module"]
pub struct P1SELC_SPEC;
impl crate::RegisterSpec for P1SELC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1selc::R](R) reader structure"]
impl crate::Readable for P1SELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1selc::W](W) writer structure"]
impl crate::Writable for P1SELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1SELC to value 0"]
impl crate::Resettable for P1SELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
