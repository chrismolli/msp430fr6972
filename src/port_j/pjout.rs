#[doc = "Register `PJOUT` reader"]
pub struct R(crate::R<PJOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PJOUT_SPEC>> for R {
    fn from(reader: crate::R<PJOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJOUT` writer"]
pub struct W(crate::W<PJOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJOUT_SPEC>;
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
impl core::convert::From<crate::W<PJOUT_SPEC>> for W {
    fn from(writer: crate::W<PJOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJOUT0` reader - PJOUT0"]
pub struct PJOUT0_R(crate::FieldReader<bool, bool>);
impl PJOUT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT0` writer - PJOUT0"]
pub struct PJOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT0_W<'a> {
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
#[doc = "Field `PJOUT1` reader - PJOUT1"]
pub struct PJOUT1_R(crate::FieldReader<bool, bool>);
impl PJOUT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT1` writer - PJOUT1"]
pub struct PJOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT1_W<'a> {
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
#[doc = "Field `PJOUT2` reader - PJOUT2"]
pub struct PJOUT2_R(crate::FieldReader<bool, bool>);
impl PJOUT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT2` writer - PJOUT2"]
pub struct PJOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT2_W<'a> {
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
#[doc = "Field `PJOUT3` reader - PJOUT3"]
pub struct PJOUT3_R(crate::FieldReader<bool, bool>);
impl PJOUT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT3` writer - PJOUT3"]
pub struct PJOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT3_W<'a> {
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
#[doc = "Field `PJOUT4` reader - PJOUT4"]
pub struct PJOUT4_R(crate::FieldReader<bool, bool>);
impl PJOUT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT4` writer - PJOUT4"]
pub struct PJOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT4_W<'a> {
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
#[doc = "Field `PJOUT5` reader - PJOUT5"]
pub struct PJOUT5_R(crate::FieldReader<bool, bool>);
impl PJOUT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT5` writer - PJOUT5"]
pub struct PJOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT5_W<'a> {
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
#[doc = "Field `PJOUT6` reader - PJOUT6"]
pub struct PJOUT6_R(crate::FieldReader<bool, bool>);
impl PJOUT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT6` writer - PJOUT6"]
pub struct PJOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT6_W<'a> {
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
#[doc = "Field `PJOUT7` reader - PJOUT7"]
pub struct PJOUT7_R(crate::FieldReader<bool, bool>);
impl PJOUT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PJOUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJOUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJOUT7` writer - PJOUT7"]
pub struct PJOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT7_W<'a> {
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
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&self) -> PJOUT0_R {
        PJOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&self) -> PJOUT1_R {
        PJOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&self) -> PJOUT2_R {
        PJOUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&self) -> PJOUT3_R {
        PJOUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJOUT4"]
    #[inline(always)]
    pub fn pjout4(&self) -> PJOUT4_R {
        PJOUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJOUT5"]
    #[inline(always)]
    pub fn pjout5(&self) -> PJOUT5_R {
        PJOUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJOUT6"]
    #[inline(always)]
    pub fn pjout6(&self) -> PJOUT6_R {
        PJOUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJOUT7"]
    #[inline(always)]
    pub fn pjout7(&self) -> PJOUT7_R {
        PJOUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&mut self) -> PJOUT0_W {
        PJOUT0_W { w: self }
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&mut self) -> PJOUT1_W {
        PJOUT1_W { w: self }
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&mut self) -> PJOUT2_W {
        PJOUT2_W { w: self }
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&mut self) -> PJOUT3_W {
        PJOUT3_W { w: self }
    }
    #[doc = "Bit 4 - PJOUT4"]
    #[inline(always)]
    pub fn pjout4(&mut self) -> PJOUT4_W {
        PJOUT4_W { w: self }
    }
    #[doc = "Bit 5 - PJOUT5"]
    #[inline(always)]
    pub fn pjout5(&mut self) -> PJOUT5_W {
        PJOUT5_W { w: self }
    }
    #[doc = "Bit 6 - PJOUT6"]
    #[inline(always)]
    pub fn pjout6(&mut self) -> PJOUT6_W {
        PJOUT6_W { w: self }
    }
    #[doc = "Bit 7 - PJOUT7"]
    #[inline(always)]
    pub fn pjout7(&mut self) -> PJOUT7_W {
        PJOUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjout](index.html) module"]
pub struct PJOUT_SPEC;
impl crate::RegisterSpec for PJOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjout::R](R) reader structure"]
impl crate::Readable for PJOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjout::W](W) writer structure"]
impl crate::Writable for PJOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PJOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
