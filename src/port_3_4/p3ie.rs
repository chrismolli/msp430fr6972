#[doc = "Register `P3IE` reader"]
pub struct R(crate::R<P3IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P3IE_SPEC>> for R {
    fn from(reader: crate::R<P3IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3IE` writer"]
pub struct W(crate::W<P3IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3IE_SPEC>;
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
impl core::convert::From<crate::W<P3IE_SPEC>> for W {
    fn from(writer: crate::W<P3IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IE0` reader - P3IE0"]
pub struct P3IE0_R(crate::FieldReader<bool, bool>);
impl P3IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE0` writer - P3IE0"]
pub struct P3IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE0_W<'a> {
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
#[doc = "Field `P3IE1` reader - P3IE1"]
pub struct P3IE1_R(crate::FieldReader<bool, bool>);
impl P3IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE1` writer - P3IE1"]
pub struct P3IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE1_W<'a> {
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
#[doc = "Field `P3IE2` reader - P3IE2"]
pub struct P3IE2_R(crate::FieldReader<bool, bool>);
impl P3IE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE2` writer - P3IE2"]
pub struct P3IE2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE2_W<'a> {
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
#[doc = "Field `P3IE3` reader - P3IE3"]
pub struct P3IE3_R(crate::FieldReader<bool, bool>);
impl P3IE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE3` writer - P3IE3"]
pub struct P3IE3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE3_W<'a> {
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
#[doc = "Field `P3IE4` reader - P3IE4"]
pub struct P3IE4_R(crate::FieldReader<bool, bool>);
impl P3IE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE4` writer - P3IE4"]
pub struct P3IE4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE4_W<'a> {
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
#[doc = "Field `P3IE5` reader - P3IE5"]
pub struct P3IE5_R(crate::FieldReader<bool, bool>);
impl P3IE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE5` writer - P3IE5"]
pub struct P3IE5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE5_W<'a> {
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
#[doc = "Field `P3IE6` reader - P3IE6"]
pub struct P3IE6_R(crate::FieldReader<bool, bool>);
impl P3IE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE6` writer - P3IE6"]
pub struct P3IE6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE6_W<'a> {
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
#[doc = "Field `P3IE7` reader - P3IE7"]
pub struct P3IE7_R(crate::FieldReader<bool, bool>);
impl P3IE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3IE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE7` writer - P3IE7"]
pub struct P3IE7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE7_W<'a> {
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
    #[doc = "Bit 0 - P3IE0"]
    #[inline(always)]
    pub fn p3ie0(&self) -> P3IE0_R {
        P3IE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3IE1"]
    #[inline(always)]
    pub fn p3ie1(&self) -> P3IE1_R {
        P3IE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3IE2"]
    #[inline(always)]
    pub fn p3ie2(&self) -> P3IE2_R {
        P3IE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3IE3"]
    #[inline(always)]
    pub fn p3ie3(&self) -> P3IE3_R {
        P3IE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3IE4"]
    #[inline(always)]
    pub fn p3ie4(&self) -> P3IE4_R {
        P3IE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3IE5"]
    #[inline(always)]
    pub fn p3ie5(&self) -> P3IE5_R {
        P3IE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3IE6"]
    #[inline(always)]
    pub fn p3ie6(&self) -> P3IE6_R {
        P3IE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3IE7"]
    #[inline(always)]
    pub fn p3ie7(&self) -> P3IE7_R {
        P3IE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3IE0"]
    #[inline(always)]
    pub fn p3ie0(&mut self) -> P3IE0_W {
        P3IE0_W { w: self }
    }
    #[doc = "Bit 1 - P3IE1"]
    #[inline(always)]
    pub fn p3ie1(&mut self) -> P3IE1_W {
        P3IE1_W { w: self }
    }
    #[doc = "Bit 2 - P3IE2"]
    #[inline(always)]
    pub fn p3ie2(&mut self) -> P3IE2_W {
        P3IE2_W { w: self }
    }
    #[doc = "Bit 3 - P3IE3"]
    #[inline(always)]
    pub fn p3ie3(&mut self) -> P3IE3_W {
        P3IE3_W { w: self }
    }
    #[doc = "Bit 4 - P3IE4"]
    #[inline(always)]
    pub fn p3ie4(&mut self) -> P3IE4_W {
        P3IE4_W { w: self }
    }
    #[doc = "Bit 5 - P3IE5"]
    #[inline(always)]
    pub fn p3ie5(&mut self) -> P3IE5_W {
        P3IE5_W { w: self }
    }
    #[doc = "Bit 6 - P3IE6"]
    #[inline(always)]
    pub fn p3ie6(&mut self) -> P3IE6_W {
        P3IE6_W { w: self }
    }
    #[doc = "Bit 7 - P3IE7"]
    #[inline(always)]
    pub fn p3ie7(&mut self) -> P3IE7_W {
        P3IE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ie](index.html) module"]
pub struct P3IE_SPEC;
impl crate::RegisterSpec for P3IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3ie::R](R) reader structure"]
impl crate::Readable for P3IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3ie::W](W) writer structure"]
impl crate::Writable for P3IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3IE to value 0"]
impl crate::Resettable for P3IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
