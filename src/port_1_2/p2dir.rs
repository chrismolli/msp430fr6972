#[doc = "Register `P2DIR` reader"]
pub struct R(crate::R<P2DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<P2DIR_SPEC>> for R {
    fn from(reader: crate::R<P2DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2DIR` writer"]
pub struct W(crate::W<P2DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2DIR_SPEC>;
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
impl core::convert::From<crate::W<P2DIR_SPEC>> for W {
    fn from(writer: crate::W<P2DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2DIR0` reader - P2DIR0"]
pub struct P2DIR0_R(crate::FieldReader<bool, bool>);
impl P2DIR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR0` writer - P2DIR0"]
pub struct P2DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR0_W<'a> {
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
#[doc = "Field `P2DIR1` reader - P2DIR1"]
pub struct P2DIR1_R(crate::FieldReader<bool, bool>);
impl P2DIR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR1` writer - P2DIR1"]
pub struct P2DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR1_W<'a> {
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
#[doc = "Field `P2DIR2` reader - P2DIR2"]
pub struct P2DIR2_R(crate::FieldReader<bool, bool>);
impl P2DIR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR2` writer - P2DIR2"]
pub struct P2DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR2_W<'a> {
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
#[doc = "Field `P2DIR3` reader - P2DIR3"]
pub struct P2DIR3_R(crate::FieldReader<bool, bool>);
impl P2DIR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR3` writer - P2DIR3"]
pub struct P2DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR3_W<'a> {
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
#[doc = "Field `P2DIR4` reader - P2DIR4"]
pub struct P2DIR4_R(crate::FieldReader<bool, bool>);
impl P2DIR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR4` writer - P2DIR4"]
pub struct P2DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR4_W<'a> {
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
#[doc = "Field `P2DIR5` reader - P2DIR5"]
pub struct P2DIR5_R(crate::FieldReader<bool, bool>);
impl P2DIR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR5` writer - P2DIR5"]
pub struct P2DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR5_W<'a> {
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
#[doc = "Field `P2DIR6` reader - P2DIR6"]
pub struct P2DIR6_R(crate::FieldReader<bool, bool>);
impl P2DIR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR6` writer - P2DIR6"]
pub struct P2DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR6_W<'a> {
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
#[doc = "Field `P2DIR7` reader - P2DIR7"]
pub struct P2DIR7_R(crate::FieldReader<bool, bool>);
impl P2DIR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2DIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DIR7` writer - P2DIR7"]
pub struct P2DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DIR7_W<'a> {
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
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&self) -> P2DIR0_R {
        P2DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&self) -> P2DIR1_R {
        P2DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&self) -> P2DIR2_R {
        P2DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&self) -> P2DIR3_R {
        P2DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&self) -> P2DIR4_R {
        P2DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&self) -> P2DIR5_R {
        P2DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&self) -> P2DIR6_R {
        P2DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&self) -> P2DIR7_R {
        P2DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2DIR0"]
    #[inline(always)]
    pub fn p2dir0(&mut self) -> P2DIR0_W {
        P2DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P2DIR1"]
    #[inline(always)]
    pub fn p2dir1(&mut self) -> P2DIR1_W {
        P2DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P2DIR2"]
    #[inline(always)]
    pub fn p2dir2(&mut self) -> P2DIR2_W {
        P2DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P2DIR3"]
    #[inline(always)]
    pub fn p2dir3(&mut self) -> P2DIR3_W {
        P2DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P2DIR4"]
    #[inline(always)]
    pub fn p2dir4(&mut self) -> P2DIR4_W {
        P2DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P2DIR5"]
    #[inline(always)]
    pub fn p2dir5(&mut self) -> P2DIR5_W {
        P2DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P2DIR6"]
    #[inline(always)]
    pub fn p2dir6(&mut self) -> P2DIR6_W {
        P2DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P2DIR7"]
    #[inline(always)]
    pub fn p2dir7(&mut self) -> P2DIR7_W {
        P2DIR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2dir](index.html) module"]
pub struct P2DIR_SPEC;
impl crate::RegisterSpec for P2DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2dir::R](R) reader structure"]
impl crate::Readable for P2DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2dir::W](W) writer structure"]
impl crate::Writable for P2DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2DIR to value 0"]
impl crate::Resettable for P2DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
