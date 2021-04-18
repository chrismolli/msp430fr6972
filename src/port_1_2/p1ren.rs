#[doc = "Register `P1REN` reader"]
pub struct R(crate::R<P1REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P1REN_SPEC>> for R {
    fn from(reader: crate::R<P1REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1REN` writer"]
pub struct W(crate::W<P1REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1REN_SPEC>;
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
impl core::convert::From<crate::W<P1REN_SPEC>> for W {
    fn from(writer: crate::W<P1REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1REN0` reader - P1REN0"]
pub struct P1REN0_R(crate::FieldReader<bool, bool>);
impl P1REN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN0` writer - P1REN0"]
pub struct P1REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN0_W<'a> {
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
#[doc = "Field `P1REN1` reader - P1REN1"]
pub struct P1REN1_R(crate::FieldReader<bool, bool>);
impl P1REN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN1` writer - P1REN1"]
pub struct P1REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN1_W<'a> {
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
#[doc = "Field `P1REN2` reader - P1REN2"]
pub struct P1REN2_R(crate::FieldReader<bool, bool>);
impl P1REN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN2` writer - P1REN2"]
pub struct P1REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN2_W<'a> {
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
#[doc = "Field `P1REN3` reader - P1REN3"]
pub struct P1REN3_R(crate::FieldReader<bool, bool>);
impl P1REN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN3` writer - P1REN3"]
pub struct P1REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN3_W<'a> {
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
#[doc = "Field `P1REN4` reader - P1REN4"]
pub struct P1REN4_R(crate::FieldReader<bool, bool>);
impl P1REN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN4` writer - P1REN4"]
pub struct P1REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN4_W<'a> {
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
#[doc = "Field `P1REN5` reader - P1REN5"]
pub struct P1REN5_R(crate::FieldReader<bool, bool>);
impl P1REN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN5` writer - P1REN5"]
pub struct P1REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN5_W<'a> {
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
#[doc = "Field `P1REN6` reader - P1REN6"]
pub struct P1REN6_R(crate::FieldReader<bool, bool>);
impl P1REN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN6` writer - P1REN6"]
pub struct P1REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN6_W<'a> {
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
#[doc = "Field `P1REN7` reader - P1REN7"]
pub struct P1REN7_R(crate::FieldReader<bool, bool>);
impl P1REN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1REN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN7` writer - P1REN7"]
pub struct P1REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN7_W<'a> {
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
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    pub fn p1ren0(&self) -> P1REN0_R {
        P1REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    pub fn p1ren1(&self) -> P1REN1_R {
        P1REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    pub fn p1ren2(&self) -> P1REN2_R {
        P1REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    pub fn p1ren3(&self) -> P1REN3_R {
        P1REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    pub fn p1ren4(&self) -> P1REN4_R {
        P1REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    pub fn p1ren5(&self) -> P1REN5_R {
        P1REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    pub fn p1ren6(&self) -> P1REN6_R {
        P1REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    pub fn p1ren7(&self) -> P1REN7_R {
        P1REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1REN0"]
    #[inline(always)]
    pub fn p1ren0(&mut self) -> P1REN0_W {
        P1REN0_W { w: self }
    }
    #[doc = "Bit 1 - P1REN1"]
    #[inline(always)]
    pub fn p1ren1(&mut self) -> P1REN1_W {
        P1REN1_W { w: self }
    }
    #[doc = "Bit 2 - P1REN2"]
    #[inline(always)]
    pub fn p1ren2(&mut self) -> P1REN2_W {
        P1REN2_W { w: self }
    }
    #[doc = "Bit 3 - P1REN3"]
    #[inline(always)]
    pub fn p1ren3(&mut self) -> P1REN3_W {
        P1REN3_W { w: self }
    }
    #[doc = "Bit 4 - P1REN4"]
    #[inline(always)]
    pub fn p1ren4(&mut self) -> P1REN4_W {
        P1REN4_W { w: self }
    }
    #[doc = "Bit 5 - P1REN5"]
    #[inline(always)]
    pub fn p1ren5(&mut self) -> P1REN5_W {
        P1REN5_W { w: self }
    }
    #[doc = "Bit 6 - P1REN6"]
    #[inline(always)]
    pub fn p1ren6(&mut self) -> P1REN6_W {
        P1REN6_W { w: self }
    }
    #[doc = "Bit 7 - P1REN7"]
    #[inline(always)]
    pub fn p1ren7(&mut self) -> P1REN7_W {
        P1REN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ren](index.html) module"]
pub struct P1REN_SPEC;
impl crate::RegisterSpec for P1REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ren::R](R) reader structure"]
impl crate::Readable for P1REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ren::W](W) writer structure"]
impl crate::Writable for P1REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1REN to value 0"]
impl crate::Resettable for P1REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
