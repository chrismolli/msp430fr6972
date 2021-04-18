#[doc = "Register `P3IFG` reader"]
pub struct R(crate::R<P3IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P3IFG_SPEC>> for R {
    fn from(reader: crate::R<P3IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3IFG` writer"]
pub struct W(crate::W<P3IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3IFG_SPEC>;
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
impl core::convert::From<crate::W<P3IFG_SPEC>> for W {
    fn from(writer: crate::W<P3IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IFG0` reader - P3IFG0"]
pub struct P3IFG0_R(crate::FieldReader<bool, bool>);
impl P3IFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG0` writer - P3IFG0"]
pub struct P3IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG0_W<'a> {
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
#[doc = "Field `P3IFG1` reader - P3IFG1"]
pub struct P3IFG1_R(crate::FieldReader<bool, bool>);
impl P3IFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG1` writer - P3IFG1"]
pub struct P3IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG1_W<'a> {
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
#[doc = "Field `P3IFG2` reader - P3IFG2"]
pub struct P3IFG2_R(crate::FieldReader<bool, bool>);
impl P3IFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG2` writer - P3IFG2"]
pub struct P3IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG2_W<'a> {
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
#[doc = "Field `P3IFG3` reader - P3IFG3"]
pub struct P3IFG3_R(crate::FieldReader<bool, bool>);
impl P3IFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG3` writer - P3IFG3"]
pub struct P3IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG3_W<'a> {
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
#[doc = "Field `P3IFG4` reader - P3IFG4"]
pub struct P3IFG4_R(crate::FieldReader<bool, bool>);
impl P3IFG4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG4` writer - P3IFG4"]
pub struct P3IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG4_W<'a> {
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
#[doc = "Field `P3IFG5` reader - P3IFG5"]
pub struct P3IFG5_R(crate::FieldReader<bool, bool>);
impl P3IFG5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG5` writer - P3IFG5"]
pub struct P3IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG5_W<'a> {
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
#[doc = "Field `P3IFG6` reader - P3IFG6"]
pub struct P3IFG6_R(crate::FieldReader<bool, bool>);
impl P3IFG6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG6` writer - P3IFG6"]
pub struct P3IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG6_W<'a> {
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
#[doc = "Field `P3IFG7` reader - P3IFG7"]
pub struct P3IFG7_R(crate::FieldReader<bool, bool>);
impl P3IFG7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG7` writer - P3IFG7"]
pub struct P3IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG7_W<'a> {
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
    #[doc = "Bit 0 - P3IFG0"]
    #[inline(always)]
    pub fn p3ifg0(&self) -> P3IFG0_R {
        P3IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IFG1"]
    #[inline(always)]
    pub fn p3ifg1(&self) -> P3IFG1_R {
        P3IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IFG2"]
    #[inline(always)]
    pub fn p3ifg2(&self) -> P3IFG2_R {
        P3IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IFG3"]
    #[inline(always)]
    pub fn p3ifg3(&self) -> P3IFG3_R {
        P3IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IFG4"]
    #[inline(always)]
    pub fn p3ifg4(&self) -> P3IFG4_R {
        P3IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IFG5"]
    #[inline(always)]
    pub fn p3ifg5(&self) -> P3IFG5_R {
        P3IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IFG6"]
    #[inline(always)]
    pub fn p3ifg6(&self) -> P3IFG6_R {
        P3IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IFG7"]
    #[inline(always)]
    pub fn p3ifg7(&self) -> P3IFG7_R {
        P3IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IFG0"]
    #[inline(always)]
    pub fn p3ifg0(&mut self) -> P3IFG0_W {
        P3IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P3IFG1"]
    #[inline(always)]
    pub fn p3ifg1(&mut self) -> P3IFG1_W {
        P3IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P3IFG2"]
    #[inline(always)]
    pub fn p3ifg2(&mut self) -> P3IFG2_W {
        P3IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P3IFG3"]
    #[inline(always)]
    pub fn p3ifg3(&mut self) -> P3IFG3_W {
        P3IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P3IFG4"]
    #[inline(always)]
    pub fn p3ifg4(&mut self) -> P3IFG4_W {
        P3IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P3IFG5"]
    #[inline(always)]
    pub fn p3ifg5(&mut self) -> P3IFG5_W {
        P3IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P3IFG6"]
    #[inline(always)]
    pub fn p3ifg6(&mut self) -> P3IFG6_W {
        P3IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P3IFG7"]
    #[inline(always)]
    pub fn p3ifg7(&mut self) -> P3IFG7_W {
        P3IFG7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ifg](index.html) module"]
pub struct P3IFG_SPEC;
impl crate::RegisterSpec for P3IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3ifg::R](R) reader structure"]
impl crate::Readable for P3IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3ifg::W](W) writer structure"]
impl crate::Writable for P3IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3IFG to value 0"]
impl crate::Resettable for P3IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
