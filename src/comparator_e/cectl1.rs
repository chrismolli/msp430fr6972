#[doc = "Register `CECTL1` reader"]
pub struct R(crate::R<CECTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CECTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CECTL1_SPEC>> for R {
    fn from(reader: crate::R<CECTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CECTL1` writer"]
pub struct W(crate::W<CECTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CECTL1_SPEC>;
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
impl core::convert::From<crate::W<CECTL1_SPEC>> for W {
    fn from(writer: crate::W<CECTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEOUT` reader - Comp. E Output"]
pub struct CEOUT_R(crate::FieldReader<bool, bool>);
impl CEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEOUT` writer - Comp. E Output"]
pub struct CEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CEOUT_W<'a> {
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
#[doc = "Field `CEOUTPOL` reader - Comp. E Output Polarity"]
pub struct CEOUTPOL_R(crate::FieldReader<bool, bool>);
impl CEOUTPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEOUTPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEOUTPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEOUTPOL` writer - Comp. E Output Polarity"]
pub struct CEOUTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEOUTPOL_W<'a> {
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
#[doc = "Field `CEF` reader - Comp. E Enable Output Filter"]
pub struct CEF_R(crate::FieldReader<bool, bool>);
impl CEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEF` writer - Comp. E Enable Output Filter"]
pub struct CEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEF_W<'a> {
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
#[doc = "Field `CEIES` reader - Comp. E Interrupt Edge Select"]
pub struct CEIES_R(crate::FieldReader<bool, bool>);
impl CEIES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIES` writer - Comp. E Interrupt Edge Select"]
pub struct CEIES_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIES_W<'a> {
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
#[doc = "Field `CESHORT` reader - Comp. E Input Short"]
pub struct CESHORT_R(crate::FieldReader<bool, bool>);
impl CESHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CESHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CESHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CESHORT` writer - Comp. E Input Short"]
pub struct CESHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CESHORT_W<'a> {
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
#[doc = "Field `CEEX` reader - Comp. E Exchange Inputs"]
pub struct CEEX_R(crate::FieldReader<bool, bool>);
impl CEEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEEX` writer - Comp. E Exchange Inputs"]
pub struct CEEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CEEX_W<'a> {
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
#[doc = "Comp. E Filter delay Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEFDLY_A {
    #[doc = "0: Comp. E Filter delay 0 : 450ns"]
    CEFDLY_0 = 0,
    #[doc = "1: Comp. E Filter delay 1 : 900ns"]
    CEFDLY_1 = 1,
    #[doc = "2: Comp. E Filter delay 2 : 1800ns"]
    CEFDLY_2 = 2,
    #[doc = "3: Comp. E Filter delay 3 : 3600ns"]
    CEFDLY_3 = 3,
}
impl From<CEFDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: CEFDLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CEFDLY` reader - Comp. E Filter delay Bit 0"]
pub struct CEFDLY_R(crate::FieldReader<u8, CEFDLY_A>);
impl CEFDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CEFDLY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEFDLY_A {
        match self.bits {
            0 => CEFDLY_A::CEFDLY_0,
            1 => CEFDLY_A::CEFDLY_1,
            2 => CEFDLY_A::CEFDLY_2,
            3 => CEFDLY_A::CEFDLY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEFDLY_0`"]
    #[inline(always)]
    pub fn is_cefdly_0(&self) -> bool {
        **self == CEFDLY_A::CEFDLY_0
    }
    #[doc = "Checks if the value of the field is `CEFDLY_1`"]
    #[inline(always)]
    pub fn is_cefdly_1(&self) -> bool {
        **self == CEFDLY_A::CEFDLY_1
    }
    #[doc = "Checks if the value of the field is `CEFDLY_2`"]
    #[inline(always)]
    pub fn is_cefdly_2(&self) -> bool {
        **self == CEFDLY_A::CEFDLY_2
    }
    #[doc = "Checks if the value of the field is `CEFDLY_3`"]
    #[inline(always)]
    pub fn is_cefdly_3(&self) -> bool {
        **self == CEFDLY_A::CEFDLY_3
    }
}
impl core::ops::Deref for CEFDLY_R {
    type Target = crate::FieldReader<u8, CEFDLY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEFDLY` writer - Comp. E Filter delay Bit 0"]
pub struct CEFDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CEFDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEFDLY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. E Filter delay 0 : 450ns"]
    #[inline(always)]
    pub fn cefdly_0(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_0)
    }
    #[doc = "Comp. E Filter delay 1 : 900ns"]
    #[inline(always)]
    pub fn cefdly_1(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_1)
    }
    #[doc = "Comp. E Filter delay 2 : 1800ns"]
    #[inline(always)]
    pub fn cefdly_2(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_2)
    }
    #[doc = "Comp. E Filter delay 3 : 3600ns"]
    #[inline(always)]
    pub fn cefdly_3(self) -> &'a mut W {
        self.variant(CEFDLY_A::CEFDLY_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Comp. E Power mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEPWRMD_A {
    #[doc = "0: Comp. E Power mode 0"]
    CEPWRMD_0 = 0,
    #[doc = "1: Comp. E Power mode 1"]
    CEPWRMD_1 = 1,
    #[doc = "2: Comp. E Power mode 2"]
    CEPWRMD_2 = 2,
    #[doc = "3: Comp. E Power mode 3"]
    CEPWRMD_3 = 3,
}
impl From<CEPWRMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CEPWRMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CEPWRMD` reader - Comp. E Power mode Bit 0"]
pub struct CEPWRMD_R(crate::FieldReader<u8, CEPWRMD_A>);
impl CEPWRMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CEPWRMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPWRMD_A {
        match self.bits {
            0 => CEPWRMD_A::CEPWRMD_0,
            1 => CEPWRMD_A::CEPWRMD_1,
            2 => CEPWRMD_A::CEPWRMD_2,
            3 => CEPWRMD_A::CEPWRMD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_0`"]
    #[inline(always)]
    pub fn is_cepwrmd_0(&self) -> bool {
        **self == CEPWRMD_A::CEPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_1`"]
    #[inline(always)]
    pub fn is_cepwrmd_1(&self) -> bool {
        **self == CEPWRMD_A::CEPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_2`"]
    #[inline(always)]
    pub fn is_cepwrmd_2(&self) -> bool {
        **self == CEPWRMD_A::CEPWRMD_2
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_3`"]
    #[inline(always)]
    pub fn is_cepwrmd_3(&self) -> bool {
        **self == CEPWRMD_A::CEPWRMD_3
    }
}
impl core::ops::Deref for CEPWRMD_R {
    type Target = crate::FieldReader<u8, CEPWRMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPWRMD` writer - Comp. E Power mode Bit 0"]
pub struct CEPWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPWRMD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comp. E Power mode 0"]
    #[inline(always)]
    pub fn cepwrmd_0(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_0)
    }
    #[doc = "Comp. E Power mode 1"]
    #[inline(always)]
    pub fn cepwrmd_1(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_1)
    }
    #[doc = "Comp. E Power mode 2"]
    #[inline(always)]
    pub fn cepwrmd_2(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_2)
    }
    #[doc = "Comp. E Power mode 3"]
    #[inline(always)]
    pub fn cepwrmd_3(self) -> &'a mut W {
        self.variant(CEPWRMD_A::CEPWRMD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CEON` reader - Comp. E enable"]
pub struct CEON_R(crate::FieldReader<bool, bool>);
impl CEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEON` writer - Comp. E enable"]
pub struct CEON_W<'a> {
    w: &'a mut W,
}
impl<'a> CEON_W<'a> {
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
#[doc = "Field `CEMRVL` reader - Comp. E CEMRV Level"]
pub struct CEMRVL_R(crate::FieldReader<bool, bool>);
impl CEMRVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEMRVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEMRVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEMRVL` writer - Comp. E CEMRV Level"]
pub struct CEMRVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEMRVL_W<'a> {
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
#[doc = "Field `CEMRVS` reader - Comp. E Output selects between VREF0 or VREF1"]
pub struct CEMRVS_R(crate::FieldReader<bool, bool>);
impl CEMRVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEMRVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEMRVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEMRVS` writer - Comp. E Output selects between VREF0 or VREF1"]
pub struct CEMRVS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEMRVS_W<'a> {
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
    #[doc = "Bit 0 - Comp. E Output"]
    #[inline(always)]
    pub fn ceout(&self) -> CEOUT_R {
        CEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. E Output Polarity"]
    #[inline(always)]
    pub fn ceoutpol(&self) -> CEOUTPOL_R {
        CEOUTPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. E Enable Output Filter"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. E Interrupt Edge Select"]
    #[inline(always)]
    pub fn ceies(&self) -> CEIES_R {
        CEIES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. E Input Short"]
    #[inline(always)]
    pub fn ceshort(&self) -> CESHORT_R {
        CESHORT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. E Exchange Inputs"]
    #[inline(always)]
    pub fn ceex(&self) -> CEEX_R {
        CEEX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Comp. E Filter delay Bit 0"]
    #[inline(always)]
    pub fn cefdly(&self) -> CEFDLY_R {
        CEFDLY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Comp. E Power mode Bit 0"]
    #[inline(always)]
    pub fn cepwrmd(&self) -> CEPWRMD_R {
        CEPWRMD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Comp. E enable"]
    #[inline(always)]
    pub fn ceon(&self) -> CEON_R {
        CEON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comp. E CEMRV Level"]
    #[inline(always)]
    pub fn cemrvl(&self) -> CEMRVL_R {
        CEMRVL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. E Output selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs(&self) -> CEMRVS_R {
        CEMRVS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. E Output"]
    #[inline(always)]
    pub fn ceout(&mut self) -> CEOUT_W {
        CEOUT_W { w: self }
    }
    #[doc = "Bit 1 - Comp. E Output Polarity"]
    #[inline(always)]
    pub fn ceoutpol(&mut self) -> CEOUTPOL_W {
        CEOUTPOL_W { w: self }
    }
    #[doc = "Bit 2 - Comp. E Enable Output Filter"]
    #[inline(always)]
    pub fn cef(&mut self) -> CEF_W {
        CEF_W { w: self }
    }
    #[doc = "Bit 3 - Comp. E Interrupt Edge Select"]
    #[inline(always)]
    pub fn ceies(&mut self) -> CEIES_W {
        CEIES_W { w: self }
    }
    #[doc = "Bit 4 - Comp. E Input Short"]
    #[inline(always)]
    pub fn ceshort(&mut self) -> CESHORT_W {
        CESHORT_W { w: self }
    }
    #[doc = "Bit 5 - Comp. E Exchange Inputs"]
    #[inline(always)]
    pub fn ceex(&mut self) -> CEEX_W {
        CEEX_W { w: self }
    }
    #[doc = "Bits 6:7 - Comp. E Filter delay Bit 0"]
    #[inline(always)]
    pub fn cefdly(&mut self) -> CEFDLY_W {
        CEFDLY_W { w: self }
    }
    #[doc = "Bits 8:9 - Comp. E Power mode Bit 0"]
    #[inline(always)]
    pub fn cepwrmd(&mut self) -> CEPWRMD_W {
        CEPWRMD_W { w: self }
    }
    #[doc = "Bit 10 - Comp. E enable"]
    #[inline(always)]
    pub fn ceon(&mut self) -> CEON_W {
        CEON_W { w: self }
    }
    #[doc = "Bit 11 - Comp. E CEMRV Level"]
    #[inline(always)]
    pub fn cemrvl(&mut self) -> CEMRVL_W {
        CEMRVL_W { w: self }
    }
    #[doc = "Bit 12 - Comp. E Output selects between VREF0 or VREF1"]
    #[inline(always)]
    pub fn cemrvs(&mut self) -> CEMRVS_W {
        CEMRVS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator E Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cectl1](index.html) module"]
pub struct CECTL1_SPEC;
impl crate::RegisterSpec for CECTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cectl1::R](R) reader structure"]
impl crate::Readable for CECTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cectl1::W](W) writer structure"]
impl crate::Writable for CECTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CECTL1 to value 0"]
impl crate::Resettable for CECTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
