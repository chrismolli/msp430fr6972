#[doc = "Register `ADC12IFGR2` reader"]
pub struct R(crate::R<ADC12IFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC12IFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC12IFGR2_SPEC>> for R {
    fn from(reader: crate::R<ADC12IFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC12IFGR2` writer"]
pub struct W(crate::W<ADC12IFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC12IFGR2_SPEC>;
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
impl core::convert::From<crate::W<ADC12IFGR2_SPEC>> for W {
    fn from(writer: crate::W<ADC12IFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC12INIFG` reader - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
pub struct ADC12INIFG_R(crate::FieldReader<bool, bool>);
impl ADC12INIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12INIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12INIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12INIFG` writer - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
pub struct ADC12INIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIFG_W<'a> {
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
#[doc = "Field `ADC12LOIFG` reader - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
pub struct ADC12LOIFG_R(crate::FieldReader<bool, bool>);
impl ADC12LOIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12LOIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12LOIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12LOIFG` writer - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
pub struct ADC12LOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIFG_W<'a> {
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
#[doc = "Field `ADC12HIIFG` reader - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
pub struct ADC12HIIFG_R(crate::FieldReader<bool, bool>);
impl ADC12HIIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12HIIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12HIIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12HIIFG` writer - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
pub struct ADC12HIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIFG_W<'a> {
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
#[doc = "Field `ADC12OVIFG` reader - ADC12 ADC12MEMx Overflow interrupt Flag"]
pub struct ADC12OVIFG_R(crate::FieldReader<bool, bool>);
impl ADC12OVIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12OVIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12OVIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12OVIFG` writer - ADC12 ADC12MEMx Overflow interrupt Flag"]
pub struct ADC12OVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIFG_W<'a> {
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
#[doc = "Field `ADC12TOVIFG` reader - ADC12 Timer Overflow interrupt Flag"]
pub struct ADC12TOVIFG_R(crate::FieldReader<bool, bool>);
impl ADC12TOVIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12TOVIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12TOVIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12TOVIFG` writer - ADC12 Timer Overflow interrupt Flag"]
pub struct ADC12TOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIFG_W<'a> {
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
#[doc = "Field `ADC12RDYIFG` reader - ADC12 local buffered reference ready interrupt Flag"]
pub struct ADC12RDYIFG_R(crate::FieldReader<bool, bool>);
impl ADC12RDYIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12RDYIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12RDYIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12RDYIFG` writer - ADC12 local buffered reference ready interrupt Flag"]
pub struct ADC12RDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIFG_W<'a> {
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
    #[doc = "Bit 1 - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inifg(&self) -> ADC12INIFG_R {
        ADC12INIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loifg(&self) -> ADC12LOIFG_R {
        ADC12LOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiifg(&self) -> ADC12HIIFG_R {
        ADC12HIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12ovifg(&self) -> ADC12OVIFG_R {
        ADC12OVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12tovifg(&self) -> ADC12TOVIFG_R {
        ADC12TOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt Flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&self) -> ADC12RDYIFG_R {
        ADC12RDYIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inifg(&mut self) -> ADC12INIFG_W {
        ADC12INIFG_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loifg(&mut self) -> ADC12LOIFG_W {
        ADC12LOIFG_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiifg(&mut self) -> ADC12HIIFG_W {
        ADC12HIIFG_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12ovifg(&mut self) -> ADC12OVIFG_W {
        ADC12OVIFG_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12tovifg(&mut self) -> ADC12TOVIFG_W {
        ADC12TOVIFG_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt Flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&mut self) -> ADC12RDYIFG_W {
        ADC12RDYIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC12 B Interrupt Flag 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc12ifgr2](index.html) module"]
pub struct ADC12IFGR2_SPEC;
impl crate::RegisterSpec for ADC12IFGR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc12ifgr2::R](R) reader structure"]
impl crate::Readable for ADC12IFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc12ifgr2::W](W) writer structure"]
impl crate::Writable for ADC12IFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC12IFGR2 to value 0"]
impl crate::Resettable for ADC12IFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
