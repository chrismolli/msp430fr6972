#[doc = "Register `P4IFG` reader"]
pub struct R(crate::R<P4IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P4IFG_SPEC>> for R {
    fn from(reader: crate::R<P4IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4IFG` writer"]
pub struct W(crate::W<P4IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4IFG_SPEC>;
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
impl core::convert::From<crate::W<P4IFG_SPEC>> for W {
    fn from(writer: crate::W<P4IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4IFG0` reader - P4IFG0"]
pub struct P4IFG0_R(crate::FieldReader<bool, bool>);
impl P4IFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG0` writer - P4IFG0"]
pub struct P4IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG0_W<'a> {
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
#[doc = "Field `P4IFG1` reader - P4IFG1"]
pub struct P4IFG1_R(crate::FieldReader<bool, bool>);
impl P4IFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG1` writer - P4IFG1"]
pub struct P4IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG1_W<'a> {
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
#[doc = "Field `P4IFG2` reader - P4IFG2"]
pub struct P4IFG2_R(crate::FieldReader<bool, bool>);
impl P4IFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG2` writer - P4IFG2"]
pub struct P4IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG2_W<'a> {
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
#[doc = "Field `P4IFG3` reader - P4IFG3"]
pub struct P4IFG3_R(crate::FieldReader<bool, bool>);
impl P4IFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG3` writer - P4IFG3"]
pub struct P4IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG3_W<'a> {
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
#[doc = "Field `P4IFG4` reader - P4IFG4"]
pub struct P4IFG4_R(crate::FieldReader<bool, bool>);
impl P4IFG4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG4` writer - P4IFG4"]
pub struct P4IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG4_W<'a> {
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
#[doc = "Field `P4IFG5` reader - P4IFG5"]
pub struct P4IFG5_R(crate::FieldReader<bool, bool>);
impl P4IFG5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG5` writer - P4IFG5"]
pub struct P4IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG5_W<'a> {
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
#[doc = "Field `P4IFG6` reader - P4IFG6"]
pub struct P4IFG6_R(crate::FieldReader<bool, bool>);
impl P4IFG6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG6` writer - P4IFG6"]
pub struct P4IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG6_W<'a> {
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
#[doc = "Field `P4IFG7` reader - P4IFG7"]
pub struct P4IFG7_R(crate::FieldReader<bool, bool>);
impl P4IFG7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG7` writer - P4IFG7"]
pub struct P4IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG7_W<'a> {
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
    #[doc = "Bit 0 - P4IFG0"]
    #[inline(always)]
    pub fn p4ifg0(&self) -> P4IFG0_R {
        P4IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IFG1"]
    #[inline(always)]
    pub fn p4ifg1(&self) -> P4IFG1_R {
        P4IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IFG2"]
    #[inline(always)]
    pub fn p4ifg2(&self) -> P4IFG2_R {
        P4IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IFG3"]
    #[inline(always)]
    pub fn p4ifg3(&self) -> P4IFG3_R {
        P4IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IFG4"]
    #[inline(always)]
    pub fn p4ifg4(&self) -> P4IFG4_R {
        P4IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IFG5"]
    #[inline(always)]
    pub fn p4ifg5(&self) -> P4IFG5_R {
        P4IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IFG6"]
    #[inline(always)]
    pub fn p4ifg6(&self) -> P4IFG6_R {
        P4IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IFG7"]
    #[inline(always)]
    pub fn p4ifg7(&self) -> P4IFG7_R {
        P4IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IFG0"]
    #[inline(always)]
    pub fn p4ifg0(&mut self) -> P4IFG0_W {
        P4IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P4IFG1"]
    #[inline(always)]
    pub fn p4ifg1(&mut self) -> P4IFG1_W {
        P4IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P4IFG2"]
    #[inline(always)]
    pub fn p4ifg2(&mut self) -> P4IFG2_W {
        P4IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P4IFG3"]
    #[inline(always)]
    pub fn p4ifg3(&mut self) -> P4IFG3_W {
        P4IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P4IFG4"]
    #[inline(always)]
    pub fn p4ifg4(&mut self) -> P4IFG4_W {
        P4IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P4IFG5"]
    #[inline(always)]
    pub fn p4ifg5(&mut self) -> P4IFG5_W {
        P4IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P4IFG6"]
    #[inline(always)]
    pub fn p4ifg6(&mut self) -> P4IFG6_W {
        P4IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P4IFG7"]
    #[inline(always)]
    pub fn p4ifg7(&mut self) -> P4IFG7_W {
        P4IFG7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 4 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ifg](index.html) module"]
pub struct P4IFG_SPEC;
impl crate::RegisterSpec for P4IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p4ifg::R](R) reader structure"]
impl crate::Readable for P4IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4ifg::W](W) writer structure"]
impl crate::Writable for P4IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4IFG to value 0"]
impl crate::Resettable for P4IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
