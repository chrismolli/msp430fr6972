#[doc = "Register `P9REN` reader"]
pub struct R(crate::R<P9REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P9REN_SPEC>> for R {
    fn from(reader: crate::R<P9REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9REN` writer"]
pub struct W(crate::W<P9REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9REN_SPEC>;
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
impl core::convert::From<crate::W<P9REN_SPEC>> for W {
    fn from(writer: crate::W<P9REN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9REN0` reader - P9REN0"]
pub struct P9REN0_R(crate::FieldReader<bool, bool>);
impl P9REN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN0` writer - P9REN0"]
pub struct P9REN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN0_W<'a> {
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
#[doc = "Field `P9REN1` reader - P9REN1"]
pub struct P9REN1_R(crate::FieldReader<bool, bool>);
impl P9REN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN1` writer - P9REN1"]
pub struct P9REN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN1_W<'a> {
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
#[doc = "Field `P9REN2` reader - P9REN2"]
pub struct P9REN2_R(crate::FieldReader<bool, bool>);
impl P9REN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN2` writer - P9REN2"]
pub struct P9REN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN2_W<'a> {
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
#[doc = "Field `P9REN3` reader - P9REN3"]
pub struct P9REN3_R(crate::FieldReader<bool, bool>);
impl P9REN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN3` writer - P9REN3"]
pub struct P9REN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN3_W<'a> {
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
#[doc = "Field `P9REN4` reader - P9REN4"]
pub struct P9REN4_R(crate::FieldReader<bool, bool>);
impl P9REN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN4` writer - P9REN4"]
pub struct P9REN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN4_W<'a> {
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
#[doc = "Field `P9REN5` reader - P9REN5"]
pub struct P9REN5_R(crate::FieldReader<bool, bool>);
impl P9REN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN5` writer - P9REN5"]
pub struct P9REN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN5_W<'a> {
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
#[doc = "Field `P9REN6` reader - P9REN6"]
pub struct P9REN6_R(crate::FieldReader<bool, bool>);
impl P9REN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN6` writer - P9REN6"]
pub struct P9REN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN6_W<'a> {
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
#[doc = "Field `P9REN7` reader - P9REN7"]
pub struct P9REN7_R(crate::FieldReader<bool, bool>);
impl P9REN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9REN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9REN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9REN7` writer - P9REN7"]
pub struct P9REN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9REN7_W<'a> {
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
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&self) -> P9REN0_R {
        P9REN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&self) -> P9REN1_R {
        P9REN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&self) -> P9REN2_R {
        P9REN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&self) -> P9REN3_R {
        P9REN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&self) -> P9REN4_R {
        P9REN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&self) -> P9REN5_R {
        P9REN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&self) -> P9REN6_R {
        P9REN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&self) -> P9REN7_R {
        P9REN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9REN0"]
    #[inline(always)]
    pub fn p9ren0(&mut self) -> P9REN0_W {
        P9REN0_W { w: self }
    }
    #[doc = "Bit 1 - P9REN1"]
    #[inline(always)]
    pub fn p9ren1(&mut self) -> P9REN1_W {
        P9REN1_W { w: self }
    }
    #[doc = "Bit 2 - P9REN2"]
    #[inline(always)]
    pub fn p9ren2(&mut self) -> P9REN2_W {
        P9REN2_W { w: self }
    }
    #[doc = "Bit 3 - P9REN3"]
    #[inline(always)]
    pub fn p9ren3(&mut self) -> P9REN3_W {
        P9REN3_W { w: self }
    }
    #[doc = "Bit 4 - P9REN4"]
    #[inline(always)]
    pub fn p9ren4(&mut self) -> P9REN4_W {
        P9REN4_W { w: self }
    }
    #[doc = "Bit 5 - P9REN5"]
    #[inline(always)]
    pub fn p9ren5(&mut self) -> P9REN5_W {
        P9REN5_W { w: self }
    }
    #[doc = "Bit 6 - P9REN6"]
    #[inline(always)]
    pub fn p9ren6(&mut self) -> P9REN6_W {
        P9REN6_W { w: self }
    }
    #[doc = "Bit 7 - P9REN7"]
    #[inline(always)]
    pub fn p9ren7(&mut self) -> P9REN7_W {
        P9REN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9ren](index.html) module"]
pub struct P9REN_SPEC;
impl crate::RegisterSpec for P9REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p9ren::R](R) reader structure"]
impl crate::Readable for P9REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9ren::W](W) writer structure"]
impl crate::Writable for P9REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P9REN to value 0"]
impl crate::Resettable for P9REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
