#[doc = "Register `P7SEL0` reader"]
pub struct R(crate::R<P7SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P7SEL0_SPEC>> for R {
    fn from(reader: crate::R<P7SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P7SEL0` writer"]
pub struct W(crate::W<P7SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P7SEL0_SPEC>;
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
impl core::convert::From<crate::W<P7SEL0_SPEC>> for W {
    fn from(writer: crate::W<P7SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7SEL0_0` reader - P7SEL0_0"]
pub struct P7SEL0_0_R(crate::FieldReader<bool, bool>);
impl P7SEL0_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_0` writer - P7SEL0_0"]
pub struct P7SEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_0_W<'a> {
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
#[doc = "Field `P7SEL0_1` reader - P7SEL0_1"]
pub struct P7SEL0_1_R(crate::FieldReader<bool, bool>);
impl P7SEL0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_1` writer - P7SEL0_1"]
pub struct P7SEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_1_W<'a> {
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
#[doc = "Field `P7SEL0_2` reader - P7SEL0_2"]
pub struct P7SEL0_2_R(crate::FieldReader<bool, bool>);
impl P7SEL0_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_2` writer - P7SEL0_2"]
pub struct P7SEL0_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_2_W<'a> {
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
#[doc = "Field `P7SEL0_3` reader - P7SEL0_3"]
pub struct P7SEL0_3_R(crate::FieldReader<bool, bool>);
impl P7SEL0_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_3` writer - P7SEL0_3"]
pub struct P7SEL0_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_3_W<'a> {
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
#[doc = "Field `P7SEL0_4` reader - P7SEL0_4"]
pub struct P7SEL0_4_R(crate::FieldReader<bool, bool>);
impl P7SEL0_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_4` writer - P7SEL0_4"]
pub struct P7SEL0_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_4_W<'a> {
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
#[doc = "Field `P7SEL0_5` reader - P7SEL0_5"]
pub struct P7SEL0_5_R(crate::FieldReader<bool, bool>);
impl P7SEL0_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_5` writer - P7SEL0_5"]
pub struct P7SEL0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_5_W<'a> {
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
#[doc = "Field `P7SEL0_6` reader - P7SEL0_6"]
pub struct P7SEL0_6_R(crate::FieldReader<bool, bool>);
impl P7SEL0_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_6` writer - P7SEL0_6"]
pub struct P7SEL0_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_6_W<'a> {
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
#[doc = "Field `P7SEL0_7` reader - P7SEL0_7"]
pub struct P7SEL0_7_R(crate::FieldReader<bool, bool>);
impl P7SEL0_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7SEL0_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL0_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL0_7` writer - P7SEL0_7"]
pub struct P7SEL0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL0_7_W<'a> {
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
    #[doc = "Bit 0 - P7SEL0_0"]
    #[inline(always)]
    pub fn p7sel0_0(&self) -> P7SEL0_0_R {
        P7SEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7SEL0_1"]
    #[inline(always)]
    pub fn p7sel0_1(&self) -> P7SEL0_1_R {
        P7SEL0_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7SEL0_2"]
    #[inline(always)]
    pub fn p7sel0_2(&self) -> P7SEL0_2_R {
        P7SEL0_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7SEL0_3"]
    #[inline(always)]
    pub fn p7sel0_3(&self) -> P7SEL0_3_R {
        P7SEL0_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7SEL0_4"]
    #[inline(always)]
    pub fn p7sel0_4(&self) -> P7SEL0_4_R {
        P7SEL0_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7SEL0_5"]
    #[inline(always)]
    pub fn p7sel0_5(&self) -> P7SEL0_5_R {
        P7SEL0_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7SEL0_6"]
    #[inline(always)]
    pub fn p7sel0_6(&self) -> P7SEL0_6_R {
        P7SEL0_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7SEL0_7"]
    #[inline(always)]
    pub fn p7sel0_7(&self) -> P7SEL0_7_R {
        P7SEL0_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7SEL0_0"]
    #[inline(always)]
    pub fn p7sel0_0(&mut self) -> P7SEL0_0_W {
        P7SEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - P7SEL0_1"]
    #[inline(always)]
    pub fn p7sel0_1(&mut self) -> P7SEL0_1_W {
        P7SEL0_1_W { w: self }
    }
    #[doc = "Bit 2 - P7SEL0_2"]
    #[inline(always)]
    pub fn p7sel0_2(&mut self) -> P7SEL0_2_W {
        P7SEL0_2_W { w: self }
    }
    #[doc = "Bit 3 - P7SEL0_3"]
    #[inline(always)]
    pub fn p7sel0_3(&mut self) -> P7SEL0_3_W {
        P7SEL0_3_W { w: self }
    }
    #[doc = "Bit 4 - P7SEL0_4"]
    #[inline(always)]
    pub fn p7sel0_4(&mut self) -> P7SEL0_4_W {
        P7SEL0_4_W { w: self }
    }
    #[doc = "Bit 5 - P7SEL0_5"]
    #[inline(always)]
    pub fn p7sel0_5(&mut self) -> P7SEL0_5_W {
        P7SEL0_5_W { w: self }
    }
    #[doc = "Bit 6 - P7SEL0_6"]
    #[inline(always)]
    pub fn p7sel0_6(&mut self) -> P7SEL0_6_W {
        P7SEL0_6_W { w: self }
    }
    #[doc = "Bit 7 - P7SEL0_7"]
    #[inline(always)]
    pub fn p7sel0_7(&mut self) -> P7SEL0_7_W {
        P7SEL0_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 7 Selection0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7sel0](index.html) module"]
pub struct P7SEL0_SPEC;
impl crate::RegisterSpec for P7SEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p7sel0::R](R) reader structure"]
impl crate::Readable for P7SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p7sel0::W](W) writer structure"]
impl crate::Writable for P7SEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P7SEL0 to value 0"]
impl crate::Resettable for P7SEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
