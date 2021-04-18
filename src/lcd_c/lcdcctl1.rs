#[doc = "Register `LCDCCTL1` reader"]
pub struct R(crate::R<LCDCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCCTL1_SPEC>> for R {
    fn from(reader: crate::R<LCDCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCCTL1` writer"]
pub struct W(crate::W<LCDCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCCTL1_SPEC>;
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
impl core::convert::From<crate::W<LCDCCTL1_SPEC>> for W {
    fn from(writer: crate::W<LCDCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDFRMIFG` reader - LCD_C LCD frame interrupt flag"]
pub struct LCDFRMIFG_R(crate::FieldReader<bool, bool>);
impl LCDFRMIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDFRMIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDFRMIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDFRMIFG` writer - LCD_C LCD frame interrupt flag"]
pub struct LCDFRMIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIFG_W<'a> {
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
#[doc = "Field `LCDBLKOFFIFG` reader - LCD_C LCD blinking off interrupt flag"]
pub struct LCDBLKOFFIFG_R(crate::FieldReader<bool, bool>);
impl LCDBLKOFFIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDBLKOFFIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBLKOFFIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBLKOFFIFG` writer - LCD_C LCD blinking off interrupt flag"]
pub struct LCDBLKOFFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIFG_W<'a> {
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
#[doc = "Field `LCDBLKONIFG` reader - LCD_C LCD blinking on interrupt flag"]
pub struct LCDBLKONIFG_R(crate::FieldReader<bool, bool>);
impl LCDBLKONIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDBLKONIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBLKONIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBLKONIFG` writer - LCD_C LCD blinking on interrupt flag"]
pub struct LCDBLKONIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIFG_W<'a> {
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
#[doc = "Field `LCDNOCAPIFG` reader - LCD_C No cpacitance connected interrupt flag"]
pub struct LCDNOCAPIFG_R(crate::FieldReader<bool, bool>);
impl LCDNOCAPIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDNOCAPIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDNOCAPIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDNOCAPIFG` writer - LCD_C No cpacitance connected interrupt flag"]
pub struct LCDNOCAPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDNOCAPIFG_W<'a> {
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
#[doc = "Field `LCDFRMIE` reader - LCD_C LCD frame interrupt enable"]
pub struct LCDFRMIE_R(crate::FieldReader<bool, bool>);
impl LCDFRMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDFRMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDFRMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDFRMIE` writer - LCD_C LCD frame interrupt enable"]
pub struct LCDFRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIE_W<'a> {
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
#[doc = "Field `LCDBLKOFFIE` reader - LCD_C LCD blinking off interrupt flag"]
pub struct LCDBLKOFFIE_R(crate::FieldReader<bool, bool>);
impl LCDBLKOFFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDBLKOFFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBLKOFFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBLKOFFIE` writer - LCD_C LCD blinking off interrupt flag"]
pub struct LCDBLKOFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIE_W<'a> {
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
#[doc = "Field `LCDBLKONIE` reader - LCD_C LCD blinking on interrupt flag"]
pub struct LCDBLKONIE_R(crate::FieldReader<bool, bool>);
impl LCDBLKONIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDBLKONIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBLKONIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDBLKONIE` writer - LCD_C LCD blinking on interrupt flag"]
pub struct LCDBLKONIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LCDNOCAPIE` reader - LCD_C No cpacitance connected interrupt enable"]
pub struct LCDNOCAPIE_R(crate::FieldReader<bool, bool>);
impl LCDNOCAPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDNOCAPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDNOCAPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDNOCAPIE` writer - LCD_C No cpacitance connected interrupt enable"]
pub struct LCDNOCAPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDNOCAPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LCDFRMIFG_R {
        LCDFRMIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LCDBLKOFFIFG_R {
        LCDBLKOFFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LCDBLKONIFG_R {
        LCDBLKONIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&self) -> LCDNOCAPIFG_R {
        LCDNOCAPIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LCDFRMIE_R {
        LCDFRMIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LCDBLKOFFIE_R {
        LCDBLKOFFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LCDBLKONIE_R {
        LCDBLKONIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&self) -> LCDNOCAPIE_R {
        LCDNOCAPIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LCDFRMIFG_W {
        LCDFRMIFG_W { w: self }
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LCDBLKOFFIFG_W {
        LCDBLKOFFIFG_W { w: self }
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LCDBLKONIFG_W {
        LCDBLKONIFG_W { w: self }
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&mut self) -> LCDNOCAPIFG_W {
        LCDNOCAPIFG_W { w: self }
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LCDFRMIE_W {
        LCDFRMIE_W { w: self }
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LCDBLKOFFIE_W {
        LCDBLKOFFIE_W { w: self }
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LCDBLKONIE_W {
        LCDBLKONIE_W { w: self }
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&mut self) -> LCDNOCAPIE_W {
        LCDNOCAPIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcctl1](index.html) module"]
pub struct LCDCCTL1_SPEC;
impl crate::RegisterSpec for LCDCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcctl1::R](R) reader structure"]
impl crate::Readable for LCDCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcctl1::W](W) writer structure"]
impl crate::Writable for LCDCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCCTL1 to value 0"]
impl crate::Resettable for LCDCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
