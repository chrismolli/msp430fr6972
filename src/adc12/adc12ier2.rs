#[doc = "Register `ADC12IER2` reader"]
pub struct R(crate::R<ADC12IER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC12IER2_SPEC>> for R {
    fn from(reader: crate::R<ADC12IER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IER2` writer"]
pub struct W(crate::W<ADC12IER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IER2_SPEC>;
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
impl core::convert::From<crate::W<ADC12IER2_SPEC>> for W {
    fn from(writer: crate::W<ADC12IER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12INIE` reader - ADC12 Interrupt enable for the inside of window of the Window comparator"]
pub struct ADC12INIE_R(crate::FieldReader<bool, bool>);
impl ADC12INIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12INIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12INIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12INIE` writer - ADC12 Interrupt enable for the inside of window of the Window comparator"]
pub struct ADC12INIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIE_W<'a> {
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
#[doc = "Field `ADC12LOIE` reader - ADC12 Interrupt enable for lower threshold of the Window comparator"]
pub struct ADC12LOIE_R(crate::FieldReader<bool, bool>);
impl ADC12LOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12LOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12LOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12LOIE` writer - ADC12 Interrupt enable for lower threshold of the Window comparator"]
pub struct ADC12LOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ADC12HIIE` reader - ADC12 Interrupt enable for upper threshold of the Window comparator"]
pub struct ADC12HIIE_R(crate::FieldReader<bool, bool>);
impl ADC12HIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12HIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12HIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12HIIE` writer - ADC12 Interrupt enable for upper threshold of the Window comparator"]
pub struct ADC12HIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ADC12OVIE` reader - ADC12 ADC12MEMx Overflow interrupt enable"]
pub struct ADC12OVIE_R(crate::FieldReader<bool, bool>);
impl ADC12OVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12OVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12OVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12OVIE` writer - ADC12 ADC12MEMx Overflow interrupt enable"]
pub struct ADC12OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIE_W<'a> {
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
#[doc = "Field `ADC12TOVIE` reader - ADC12 Timer Overflow interrupt enable"]
pub struct ADC12TOVIE_R(crate::FieldReader<bool, bool>);
impl ADC12TOVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12TOVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12TOVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12TOVIE` writer - ADC12 Timer Overflow interrupt enable"]
pub struct ADC12TOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ADC12RDYIE` reader - ADC12 local buffered reference ready interrupt enable"]
pub struct ADC12RDYIE_R(crate::FieldReader<bool, bool>);
impl ADC12RDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12RDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12RDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12RDYIE` writer - ADC12 local buffered reference ready interrupt enable"]
pub struct ADC12RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - ADC12 Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inie(&self) -> ADC12INIE_R {
        ADC12INIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loie(&self) -> ADC12LOIE_R {
        ADC12LOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiie(&self) -> ADC12HIIE_R {
        ADC12HIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc12rdyie(&self) -> ADC12RDYIE_R {
        ADC12RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC12 Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inie(&mut self) -> ADC12INIE_W {
        ADC12INIE_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loie(&mut self) -> ADC12LOIE_W {
        ADC12LOIE_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiie(&mut self) -> ADC12HIIE_W {
        ADC12HIIE_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W {
        ADC12OVIE_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W {
        ADC12TOVIE_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc12rdyie(&mut self) -> ADC12RDYIE_W {
        ADC12RDYIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 B Interrupt Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ier2](index.html) module"]
pub struct ADC12IER2_SPEC;
impl crate::RegisterSpec for ADC12IER2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ier2::R](R) reader structure"]
impl crate::Readable for ADC12IER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ier2::W](W) writer structure"]
impl crate::Writable for ADC12IER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IER2 to value 0"]
impl crate::Resettable for ADC12IER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
