#[doc = "Register `P2SEL1` reader"]
pub struct R(crate::R<P2SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P2SEL1_SPEC>> for R {
    fn from(reader: crate::R<P2SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2SEL1` writer"]
pub struct W(crate::W<P2SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2SEL1_SPEC>;
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
impl core::convert::From<crate::W<P2SEL1_SPEC>> for W {
    fn from(writer: crate::W<P2SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2SEL1_0` reader - P2SEL1_0"]
pub struct P2SEL1_0_R(crate::FieldReader<bool, bool>);
impl P2SEL1_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_0` writer - P2SEL1_0"]
pub struct P2SEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_0_W<'a> {
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
#[doc = "Field `P2SEL1_1` reader - P2SEL1_1"]
pub struct P2SEL1_1_R(crate::FieldReader<bool, bool>);
impl P2SEL1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_1` writer - P2SEL1_1"]
pub struct P2SEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_1_W<'a> {
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
#[doc = "Field `P2SEL1_2` reader - P2SEL1_2"]
pub struct P2SEL1_2_R(crate::FieldReader<bool, bool>);
impl P2SEL1_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_2` writer - P2SEL1_2"]
pub struct P2SEL1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_2_W<'a> {
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
#[doc = "Field `P2SEL1_3` reader - P2SEL1_3"]
pub struct P2SEL1_3_R(crate::FieldReader<bool, bool>);
impl P2SEL1_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_3` writer - P2SEL1_3"]
pub struct P2SEL1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_3_W<'a> {
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
#[doc = "Field `P2SEL1_4` reader - P2SEL1_4"]
pub struct P2SEL1_4_R(crate::FieldReader<bool, bool>);
impl P2SEL1_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_4` writer - P2SEL1_4"]
pub struct P2SEL1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_4_W<'a> {
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
#[doc = "Field `P2SEL1_5` reader - P2SEL1_5"]
pub struct P2SEL1_5_R(crate::FieldReader<bool, bool>);
impl P2SEL1_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_5` writer - P2SEL1_5"]
pub struct P2SEL1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_5_W<'a> {
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
#[doc = "Field `P2SEL1_6` reader - P2SEL1_6"]
pub struct P2SEL1_6_R(crate::FieldReader<bool, bool>);
impl P2SEL1_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_6` writer - P2SEL1_6"]
pub struct P2SEL1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_6_W<'a> {
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
#[doc = "Field `P2SEL1_7` reader - P2SEL1_7"]
pub struct P2SEL1_7_R(crate::FieldReader<bool, bool>);
impl P2SEL1_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2SEL1_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2SEL1_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2SEL1_7` writer - P2SEL1_7"]
pub struct P2SEL1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2SEL1_7_W<'a> {
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
    #[doc = "Bit 0 - P2SEL1_0"]
    #[inline(always)]
    pub fn p2sel1_0(&self) -> P2SEL1_0_R {
        P2SEL1_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2SEL1_1"]
    #[inline(always)]
    pub fn p2sel1_1(&self) -> P2SEL1_1_R {
        P2SEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2SEL1_2"]
    #[inline(always)]
    pub fn p2sel1_2(&self) -> P2SEL1_2_R {
        P2SEL1_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2SEL1_3"]
    #[inline(always)]
    pub fn p2sel1_3(&self) -> P2SEL1_3_R {
        P2SEL1_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2SEL1_4"]
    #[inline(always)]
    pub fn p2sel1_4(&self) -> P2SEL1_4_R {
        P2SEL1_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2SEL1_5"]
    #[inline(always)]
    pub fn p2sel1_5(&self) -> P2SEL1_5_R {
        P2SEL1_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2SEL1_6"]
    #[inline(always)]
    pub fn p2sel1_6(&self) -> P2SEL1_6_R {
        P2SEL1_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2SEL1_7"]
    #[inline(always)]
    pub fn p2sel1_7(&self) -> P2SEL1_7_R {
        P2SEL1_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2SEL1_0"]
    #[inline(always)]
    pub fn p2sel1_0(&mut self) -> P2SEL1_0_W {
        P2SEL1_0_W { w: self }
    }
    #[doc = "Bit 1 - P2SEL1_1"]
    #[inline(always)]
    pub fn p2sel1_1(&mut self) -> P2SEL1_1_W {
        P2SEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - P2SEL1_2"]
    #[inline(always)]
    pub fn p2sel1_2(&mut self) -> P2SEL1_2_W {
        P2SEL1_2_W { w: self }
    }
    #[doc = "Bit 3 - P2SEL1_3"]
    #[inline(always)]
    pub fn p2sel1_3(&mut self) -> P2SEL1_3_W {
        P2SEL1_3_W { w: self }
    }
    #[doc = "Bit 4 - P2SEL1_4"]
    #[inline(always)]
    pub fn p2sel1_4(&mut self) -> P2SEL1_4_W {
        P2SEL1_4_W { w: self }
    }
    #[doc = "Bit 5 - P2SEL1_5"]
    #[inline(always)]
    pub fn p2sel1_5(&mut self) -> P2SEL1_5_W {
        P2SEL1_5_W { w: self }
    }
    #[doc = "Bit 6 - P2SEL1_6"]
    #[inline(always)]
    pub fn p2sel1_6(&mut self) -> P2SEL1_6_W {
        P2SEL1_6_W { w: self }
    }
    #[doc = "Bit 7 - P2SEL1_7"]
    #[inline(always)]
    pub fn p2sel1_7(&mut self) -> P2SEL1_7_W {
        P2SEL1_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2sel1](index.html) module"]
pub struct P2SEL1_SPEC;
impl crate::RegisterSpec for P2SEL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2sel1::R](R) reader structure"]
impl crate::Readable for P2SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2sel1::W](W) writer structure"]
impl crate::Writable for P2SEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2SEL1 to value 0"]
impl crate::Resettable for P2SEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
