#[doc = "Register `GCCTL0` reader"]
pub struct R(crate::R<GCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GCCTL0_SPEC>> for R {
    fn from(reader: crate::R<GCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL0` writer"]
pub struct W(crate::W<GCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL0_SPEC>;
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
impl core::convert::From<crate::W<GCCTL0_SPEC>> for W {
    fn from(writer: crate::W<GCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRLPMPWR` reader - FRAM Enable FRAM auto power up after LPM"]
pub struct FRLPMPWR_R(crate::FieldReader<bool, bool>);
impl FRLPMPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRLPMPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRLPMPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRLPMPWR` writer - FRAM Enable FRAM auto power up after LPM"]
pub struct FRLPMPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLPMPWR_W<'a> {
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
#[doc = "Field `FRPWR` reader - FRAM Power Control"]
pub struct FRPWR_R(crate::FieldReader<bool, bool>);
impl FRPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRPWR` writer - FRAM Power Control"]
pub struct FRPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRPWR_W<'a> {
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
#[doc = "Field `ACCTEIE` reader - RESERVED"]
pub struct ACCTEIE_R(crate::FieldReader<bool, bool>);
impl ACCTEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCTEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCTEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCTEIE` writer - RESERVED"]
pub struct ACCTEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCTEIE_W<'a> {
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
#[doc = "Field `CBDIE` reader - Enable NMI event if correctable bit error detected"]
pub struct CBDIE_R(crate::FieldReader<bool, bool>);
impl CBDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBDIE` writer - Enable NMI event if correctable bit error detected"]
pub struct CBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDIE_W<'a> {
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
#[doc = "Field `UBDIE` reader - Enable NMI event if uncorrectable bit error detected"]
pub struct UBDIE_R(crate::FieldReader<bool, bool>);
impl UBDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UBDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBDIE` writer - Enable NMI event if uncorrectable bit error detected"]
pub struct UBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDIE_W<'a> {
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
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub struct UBDRSTEN_R(crate::FieldReader<bool, bool>);
impl UBDRSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UBDRSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBDRSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub struct UBDRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UBDRSTEN_W<'a> {
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
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FRLPMPWR_R {
        FRLPMPWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&self) -> FRPWR_R {
        FRPWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&self) -> ACCTEIE_R {
        ACCTEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&self) -> CBDIE_R {
        CBDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&self) -> UBDIE_R {
        UBDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UBDRSTEN_R {
        UBDRSTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FRLPMPWR_W {
        FRLPMPWR_W { w: self }
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FRPWR_W {
        FRPWR_W { w: self }
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&mut self) -> ACCTEIE_W {
        ACCTEIE_W { w: self }
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CBDIE_W {
        CBDIE_W { w: self }
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UBDIE_W {
        UBDIE_W { w: self }
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UBDRSTEN_W {
        UBDRSTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl0](index.html) module"]
pub struct GCCTL0_SPEC;
impl crate::RegisterSpec for GCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl0::R](R) reader structure"]
impl crate::Readable for GCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl0::W](W) writer structure"]
impl crate::Writable for GCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for GCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
