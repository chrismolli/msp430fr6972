#[doc = "Register `CEINT` reader"]
pub struct R(crate::R<CEINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CEINT_SPEC>> for R {
    fn from(reader: crate::R<CEINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEINT` writer"]
pub struct W(crate::W<CEINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEINT_SPEC>;
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
impl core::convert::From<crate::W<CEINT_SPEC>> for W {
    fn from(writer: crate::W<CEINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEIFG` reader - Comp. E Interrupt Flag"]
pub struct CEIFG_R(crate::FieldReader<bool, bool>);
impl CEIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIFG` writer - Comp. E Interrupt Flag"]
pub struct CEIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `CEIIFG` reader - Comp. E Interrupt Flag Inverted Polarity"]
pub struct CEIIFG_R(crate::FieldReader<bool, bool>);
impl CEIIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIIFG` writer - Comp. E Interrupt Flag Inverted Polarity"]
pub struct CEIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CERDYIFG` reader - Comp. E Comparator_E ready interrupt flag"]
pub struct CERDYIFG_R(crate::FieldReader<bool, bool>);
impl CERDYIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CERDYIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERDYIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERDYIFG` writer - Comp. E Comparator_E ready interrupt flag"]
pub struct CERDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CERDYIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CEIE` reader - Comp. E Interrupt Enable"]
pub struct CEIE_R(crate::FieldReader<bool, bool>);
impl CEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIE` writer - Comp. E Interrupt Enable"]
pub struct CEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CEIIE` reader - Comp. E Interrupt Enable Inverted Polarity"]
pub struct CEIIE_R(crate::FieldReader<bool, bool>);
impl CEIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIIE` writer - Comp. E Interrupt Enable Inverted Polarity"]
pub struct CEIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CERDYIE` reader - Comp. E Comparator_E ready interrupt enable"]
pub struct CERDYIE_R(crate::FieldReader<bool, bool>);
impl CERDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERDYIE` writer - Comp. E Comparator_E ready interrupt enable"]
pub struct CERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CERDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comp. E Interrupt Flag"]
    #[inline(always)]
    pub fn ceifg(&self) -> CEIFG_R {
        CEIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. E Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn ceiifg(&self) -> CEIIFG_R {
        CEIIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. E Comparator_E ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&self) -> CERDYIFG_R {
        CERDYIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. E Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. E Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn ceiie(&self) -> CEIIE_R {
        CEIIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. E Comparator_E ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&self) -> CERDYIE_R {
        CERDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. E Interrupt Flag"]
    #[inline(always)]
    pub fn ceifg(&mut self) -> CEIFG_W {
        CEIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comp. E Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn ceiifg(&mut self) -> CEIIFG_W {
        CEIIFG_W { w: self }
    }
    #[doc = "Bit 4 - Comp. E Comparator_E ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&mut self) -> CERDYIFG_W {
        CERDYIFG_W { w: self }
    }
    #[doc = "Bit 8 - Comp. E Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W {
        CEIE_W { w: self }
    }
    #[doc = "Bit 9 - Comp. E Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn ceiie(&mut self) -> CEIIE_W {
        CEIIE_W { w: self }
    }
    #[doc = "Bit 12 - Comp. E Comparator_E ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&mut self) -> CERDYIE_W {
        CERDYIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator E Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceint](index.html) module"]
pub struct CEINT_SPEC;
impl crate::RegisterSpec for CEINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ceint::R](R) reader structure"]
impl crate::Readable for CEINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ceint::W](W) writer structure"]
impl crate::Writable for CEINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEINT to value 0"]
impl crate::Resettable for CEINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
