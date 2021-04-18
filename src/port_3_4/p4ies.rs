#[doc = "Register `P4IES` reader"]
pub struct R(crate::R<P4IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P4IES_SPEC>> for R {
    fn from(reader: crate::R<P4IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4IES` writer"]
pub struct W(crate::W<P4IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4IES_SPEC>;
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
impl core::convert::From<crate::W<P4IES_SPEC>> for W {
    fn from(writer: crate::W<P4IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4IES0` reader - P4IES0"]
pub struct P4IES0_R(crate::FieldReader<bool, bool>);
impl P4IES0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES0` writer - P4IES0"]
pub struct P4IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES0_W<'a> {
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
#[doc = "Field `P4IES1` reader - P4IES1"]
pub struct P4IES1_R(crate::FieldReader<bool, bool>);
impl P4IES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES1` writer - P4IES1"]
pub struct P4IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES1_W<'a> {
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
#[doc = "Field `P4IES2` reader - P4IES2"]
pub struct P4IES2_R(crate::FieldReader<bool, bool>);
impl P4IES2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES2` writer - P4IES2"]
pub struct P4IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES2_W<'a> {
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
#[doc = "Field `P4IES3` reader - P4IES3"]
pub struct P4IES3_R(crate::FieldReader<bool, bool>);
impl P4IES3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES3` writer - P4IES3"]
pub struct P4IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES3_W<'a> {
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
#[doc = "Field `P4IES4` reader - P4IES4"]
pub struct P4IES4_R(crate::FieldReader<bool, bool>);
impl P4IES4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES4` writer - P4IES4"]
pub struct P4IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES4_W<'a> {
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
#[doc = "Field `P4IES5` reader - P4IES5"]
pub struct P4IES5_R(crate::FieldReader<bool, bool>);
impl P4IES5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES5` writer - P4IES5"]
pub struct P4IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES5_W<'a> {
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
#[doc = "Field `P4IES6` reader - P4IES6"]
pub struct P4IES6_R(crate::FieldReader<bool, bool>);
impl P4IES6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES6` writer - P4IES6"]
pub struct P4IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES6_W<'a> {
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
#[doc = "Field `P4IES7` reader - P4IES7"]
pub struct P4IES7_R(crate::FieldReader<bool, bool>);
impl P4IES7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4IES7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IES7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IES7` writer - P4IES7"]
pub struct P4IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IES7_W<'a> {
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
    #[doc = "Bit 0 - P4IES0"]
    #[inline(always)]
    pub fn p4ies0(&self) -> P4IES0_R {
        P4IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4IES1"]
    #[inline(always)]
    pub fn p4ies1(&self) -> P4IES1_R {
        P4IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4IES2"]
    #[inline(always)]
    pub fn p4ies2(&self) -> P4IES2_R {
        P4IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4IES3"]
    #[inline(always)]
    pub fn p4ies3(&self) -> P4IES3_R {
        P4IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4IES4"]
    #[inline(always)]
    pub fn p4ies4(&self) -> P4IES4_R {
        P4IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4IES5"]
    #[inline(always)]
    pub fn p4ies5(&self) -> P4IES5_R {
        P4IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4IES6"]
    #[inline(always)]
    pub fn p4ies6(&self) -> P4IES6_R {
        P4IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4IES7"]
    #[inline(always)]
    pub fn p4ies7(&self) -> P4IES7_R {
        P4IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4IES0"]
    #[inline(always)]
    pub fn p4ies0(&mut self) -> P4IES0_W {
        P4IES0_W { w: self }
    }
    #[doc = "Bit 1 - P4IES1"]
    #[inline(always)]
    pub fn p4ies1(&mut self) -> P4IES1_W {
        P4IES1_W { w: self }
    }
    #[doc = "Bit 2 - P4IES2"]
    #[inline(always)]
    pub fn p4ies2(&mut self) -> P4IES2_W {
        P4IES2_W { w: self }
    }
    #[doc = "Bit 3 - P4IES3"]
    #[inline(always)]
    pub fn p4ies3(&mut self) -> P4IES3_W {
        P4IES3_W { w: self }
    }
    #[doc = "Bit 4 - P4IES4"]
    #[inline(always)]
    pub fn p4ies4(&mut self) -> P4IES4_W {
        P4IES4_W { w: self }
    }
    #[doc = "Bit 5 - P4IES5"]
    #[inline(always)]
    pub fn p4ies5(&mut self) -> P4IES5_W {
        P4IES5_W { w: self }
    }
    #[doc = "Bit 6 - P4IES6"]
    #[inline(always)]
    pub fn p4ies6(&mut self) -> P4IES6_W {
        P4IES6_W { w: self }
    }
    #[doc = "Bit 7 - P4IES7"]
    #[inline(always)]
    pub fn p4ies7(&mut self) -> P4IES7_W {
        P4IES7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 4 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ies](index.html) module"]
pub struct P4IES_SPEC;
impl crate::RegisterSpec for P4IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p4ies::R](R) reader structure"]
impl crate::Readable for P4IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4ies::W](W) writer structure"]
impl crate::Writable for P4IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4IES to value 0"]
impl crate::Resettable for P4IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
