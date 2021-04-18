#[doc = "Register `AESACTL1` reader"]
pub struct R(crate::R<AESACTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESACTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AESACTL1_SPEC>> for R {
    fn from(reader: crate::R<AESACTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESACTL1` writer"]
pub struct W(crate::W<AESACTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESACTL1_SPEC>;
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
impl core::convert::From<crate::W<AESACTL1_SPEC>> for W {
    fn from(writer: crate::W<AESACTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESBLKCNT0` reader - AES Cipher Block Counter Bit: 0"]
pub struct AESBLKCNT0_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT0` writer - AES Cipher Block Counter Bit: 0"]
pub struct AESBLKCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT0_W<'a> {
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
#[doc = "Field `AESBLKCNT1` reader - AES Cipher Block Counter Bit: 1"]
pub struct AESBLKCNT1_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT1` writer - AES Cipher Block Counter Bit: 1"]
pub struct AESBLKCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT1_W<'a> {
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
#[doc = "Field `AESBLKCNT2` reader - AES Cipher Block Counter Bit: 2"]
pub struct AESBLKCNT2_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT2` writer - AES Cipher Block Counter Bit: 2"]
pub struct AESBLKCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT2_W<'a> {
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
#[doc = "Field `AESBLKCNT3` reader - AES Cipher Block Counter Bit: 3"]
pub struct AESBLKCNT3_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT3` writer - AES Cipher Block Counter Bit: 3"]
pub struct AESBLKCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT3_W<'a> {
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
#[doc = "Field `AESBLKCNT4` reader - AES Cipher Block Counter Bit: 4"]
pub struct AESBLKCNT4_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT4` writer - AES Cipher Block Counter Bit: 4"]
pub struct AESBLKCNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT4_W<'a> {
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
#[doc = "Field `AESBLKCNT5` reader - AES Cipher Block Counter Bit: 5"]
pub struct AESBLKCNT5_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT5` writer - AES Cipher Block Counter Bit: 5"]
pub struct AESBLKCNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT5_W<'a> {
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
#[doc = "Field `AESBLKCNT6` reader - AES Cipher Block Counter Bit: 6"]
pub struct AESBLKCNT6_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT6` writer - AES Cipher Block Counter Bit: 6"]
pub struct AESBLKCNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT6_W<'a> {
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
#[doc = "Field `AESBLKCNT7` reader - AES Cipher Block Counter Bit: 7"]
pub struct AESBLKCNT7_R(crate::FieldReader<bool, bool>);
impl AESBLKCNT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBLKCNT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNT7` writer - AES Cipher Block Counter Bit: 7"]
pub struct AESBLKCNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AES Cipher Block Counter Bit: 0"]
    #[inline(always)]
    pub fn aesblkcnt0(&self) -> AESBLKCNT0_R {
        AESBLKCNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES Cipher Block Counter Bit: 1"]
    #[inline(always)]
    pub fn aesblkcnt1(&self) -> AESBLKCNT1_R {
        AESBLKCNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AES Cipher Block Counter Bit: 2"]
    #[inline(always)]
    pub fn aesblkcnt2(&self) -> AESBLKCNT2_R {
        AESBLKCNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AES Cipher Block Counter Bit: 3"]
    #[inline(always)]
    pub fn aesblkcnt3(&self) -> AESBLKCNT3_R {
        AESBLKCNT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AES Cipher Block Counter Bit: 4"]
    #[inline(always)]
    pub fn aesblkcnt4(&self) -> AESBLKCNT4_R {
        AESBLKCNT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AES Cipher Block Counter Bit: 5"]
    #[inline(always)]
    pub fn aesblkcnt5(&self) -> AESBLKCNT5_R {
        AESBLKCNT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AES Cipher Block Counter Bit: 6"]
    #[inline(always)]
    pub fn aesblkcnt6(&self) -> AESBLKCNT6_R {
        AESBLKCNT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AES Cipher Block Counter Bit: 7"]
    #[inline(always)]
    pub fn aesblkcnt7(&self) -> AESBLKCNT7_R {
        AESBLKCNT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Cipher Block Counter Bit: 0"]
    #[inline(always)]
    pub fn aesblkcnt0(&mut self) -> AESBLKCNT0_W {
        AESBLKCNT0_W { w: self }
    }
    #[doc = "Bit 1 - AES Cipher Block Counter Bit: 1"]
    #[inline(always)]
    pub fn aesblkcnt1(&mut self) -> AESBLKCNT1_W {
        AESBLKCNT1_W { w: self }
    }
    #[doc = "Bit 2 - AES Cipher Block Counter Bit: 2"]
    #[inline(always)]
    pub fn aesblkcnt2(&mut self) -> AESBLKCNT2_W {
        AESBLKCNT2_W { w: self }
    }
    #[doc = "Bit 3 - AES Cipher Block Counter Bit: 3"]
    #[inline(always)]
    pub fn aesblkcnt3(&mut self) -> AESBLKCNT3_W {
        AESBLKCNT3_W { w: self }
    }
    #[doc = "Bit 4 - AES Cipher Block Counter Bit: 4"]
    #[inline(always)]
    pub fn aesblkcnt4(&mut self) -> AESBLKCNT4_W {
        AESBLKCNT4_W { w: self }
    }
    #[doc = "Bit 5 - AES Cipher Block Counter Bit: 5"]
    #[inline(always)]
    pub fn aesblkcnt5(&mut self) -> AESBLKCNT5_W {
        AESBLKCNT5_W { w: self }
    }
    #[doc = "Bit 6 - AES Cipher Block Counter Bit: 6"]
    #[inline(always)]
    pub fn aesblkcnt6(&mut self) -> AESBLKCNT6_W {
        AESBLKCNT6_W { w: self }
    }
    #[doc = "Bit 7 - AES Cipher Block Counter Bit: 7"]
    #[inline(always)]
    pub fn aesblkcnt7(&mut self) -> AESBLKCNT7_W {
        AESBLKCNT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES accelerator control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl1](index.html) module"]
pub struct AESACTL1_SPEC;
impl crate::RegisterSpec for AESACTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesactl1::R](R) reader structure"]
impl crate::Readable for AESACTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesactl1::W](W) writer structure"]
impl crate::Writable for AESACTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESACTL1 to value 0"]
impl crate::Resettable for AESACTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
