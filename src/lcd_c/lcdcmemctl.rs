#[doc = "Register `LCDCMEMCTL` reader"]
pub struct R(crate::R<LCDCMEMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCMEMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCMEMCTL_SPEC>> for R {
    fn from(reader: crate::R<LCDCMEMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCMEMCTL` writer"]
pub struct W(crate::W<LCDCMEMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCMEMCTL_SPEC>;
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
impl core::convert::From<crate::W<LCDCMEMCTL_SPEC>> for W {
    fn from(writer: crate::W<LCDCMEMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDDISP` reader - LCD_C LCD memory registers for display"]
pub struct LCDDISP_R(crate::FieldReader<bool, bool>);
impl LCDDISP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDDISP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDDISP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDDISP` writer - LCD_C LCD memory registers for display"]
pub struct LCDDISP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDISP_W<'a> {
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
#[doc = "Field `LCDCLRM` reader - LCD_C Clear LCD memory"]
pub struct LCDCLRM_R(crate::FieldReader<bool, bool>);
impl LCDCLRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCLRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCLRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCLRM` writer - LCD_C Clear LCD memory"]
pub struct LCDCLRM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCLRM_W<'a> {
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
#[doc = "Field `LCDCLRBM` reader - LCD_C Clear LCD blinking memory"]
pub struct LCDCLRBM_R(crate::FieldReader<bool, bool>);
impl LCDCLRBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCLRBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCLRBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCLRBM` writer - LCD_C Clear LCD blinking memory"]
pub struct LCDCLRBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCLRBM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LCD_C LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LCDDISP_R {
        LCDDISP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_C Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LCDCLRM_R {
        LCDCLRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_C Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LCDCLRBM_R {
        LCDCLRBM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_C LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LCDDISP_W {
        LCDDISP_W { w: self }
    }
    #[doc = "Bit 1 - LCD_C Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LCDCLRM_W {
        LCDCLRM_W { w: self }
    }
    #[doc = "Bit 2 - LCD_C Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LCDCLRBM_W {
        LCDCLRBM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C memory control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcmemctl](index.html) module"]
pub struct LCDCMEMCTL_SPEC;
impl crate::RegisterSpec for LCDCMEMCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcmemctl::R](R) reader structure"]
impl crate::Readable for LCDCMEMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcmemctl::W](W) writer structure"]
impl crate::Writable for LCDCMEMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCMEMCTL to value 0"]
impl crate::Resettable for LCDCMEMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
