#[doc = "Register `MPUCTL1` reader"]
pub struct R(crate::R<MPUCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPUCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MPUCTL1_SPEC>> for R {
    fn from(reader: crate::R<MPUCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPUCTL1` writer"]
pub struct W(crate::W<MPUCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPUCTL1_SPEC>;
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
impl core::convert::From<crate::W<MPUCTL1_SPEC>> for W {
    fn from(writer: crate::W<MPUCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSEG1IFG` reader - MPU Main Memory Segment 1 violation interupt flag"]
pub struct MPUSEG1IFG_R(crate::FieldReader<bool, bool>);
impl MPUSEG1IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG1IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG1IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG1IFG` writer - MPU Main Memory Segment 1 violation interupt flag"]
pub struct MPUSEG1IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1IFG_W<'a> {
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
#[doc = "Field `MPUSEG2IFG` reader - MPU Main Memory Segment 2 violation interupt flag"]
pub struct MPUSEG2IFG_R(crate::FieldReader<bool, bool>);
impl MPUSEG2IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG2IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG2IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG2IFG` writer - MPU Main Memory Segment 2 violation interupt flag"]
pub struct MPUSEG2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2IFG_W<'a> {
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
#[doc = "Field `MPUSEG3IFG` reader - MPU Main Memory Segment 3 violation interupt flag"]
pub struct MPUSEG3IFG_R(crate::FieldReader<bool, bool>);
impl MPUSEG3IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEG3IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEG3IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEG3IFG` writer - MPU Main Memory Segment 3 violation interupt flag"]
pub struct MPUSEG3IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3IFG_W<'a> {
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
#[doc = "Field `MPUSEGIIFG` reader - MPU Info Memory Segment violation interupt flag"]
pub struct MPUSEGIIFG_R(crate::FieldReader<bool, bool>);
impl MPUSEGIIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIIFG` writer - MPU Info Memory Segment violation interupt flag"]
pub struct MPUSEGIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIIFG_W<'a> {
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
#[doc = "Field `MPUSEGIPIFG` reader - MPU IP Memory Segment violation interupt flag"]
pub struct MPUSEGIPIFG_R(crate::FieldReader<bool, bool>);
impl MPUSEGIPIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSEGIPIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSEGIPIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSEGIPIFG` writer - MPU IP Memory Segment violation interupt flag"]
pub struct MPUSEGIPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIPIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&self) -> MPUSEG1IFG_R {
        MPUSEG1IFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 2 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&self) -> MPUSEG2IFG_R {
        MPUSEG2IFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 3 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&self) -> MPUSEG3IFG_R {
        MPUSEG3IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Info Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegiifg(&self) -> MPUSEGIIFG_R {
        MPUSEGIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU IP Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegipifg(&self) -> MPUSEGIPIFG_R {
        MPUSEGIPIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Main Memory Segment 1 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg1ifg(&mut self) -> MPUSEG1IFG_W {
        MPUSEG1IFG_W { w: self }
    }
    #[doc = "Bit 1 - MPU Main Memory Segment 2 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg2ifg(&mut self) -> MPUSEG2IFG_W {
        MPUSEG2IFG_W { w: self }
    }
    #[doc = "Bit 2 - MPU Main Memory Segment 3 violation interupt flag"]
    #[inline(always)]
    pub fn mpuseg3ifg(&mut self) -> MPUSEG3IFG_W {
        MPUSEG3IFG_W { w: self }
    }
    #[doc = "Bit 3 - MPU Info Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegiifg(&mut self) -> MPUSEGIIFG_W {
        MPUSEGIIFG_W { w: self }
    }
    #[doc = "Bit 4 - MPU IP Memory Segment violation interupt flag"]
    #[inline(always)]
    pub fn mpusegipifg(&mut self) -> MPUSEGIPIFG_W {
        MPUSEGIPIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpuctl1](index.html) module"]
pub struct MPUCTL1_SPEC;
impl crate::RegisterSpec for MPUCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpuctl1::R](R) reader structure"]
impl crate::Readable for MPUCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpuctl1::W](W) writer structure"]
impl crate::Writable for MPUCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPUCTL1 to value 0"]
impl crate::Resettable for MPUCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
