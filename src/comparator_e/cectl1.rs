#[doc = "Reader of register CECTL1"]
pub type R = crate::R<u16, super::CECTL1>;
#[doc = "Writer for register CECTL1"]
pub type W = crate::W<u16, super::CECTL1>;
#[doc = "Register CECTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CECTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEOUT`"]
pub type CEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEOUT`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CEOUTPOL`"]
pub type CEOUTPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEOUTPOL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CEF`"]
pub type CEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CEIES`"]
pub type CEIES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIES`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CESHORT`"]
pub type CESHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CESHORT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CEEX`"]
pub type CEEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEEX`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
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
#[doc = "Reader of field `CEFDLY`"]
pub type CEFDLY_R = crate::R<u8, CEFDLY_A>;
impl CEFDLY_R {
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
        *self == CEFDLY_A::CEFDLY_0
    }
    #[doc = "Checks if the value of the field is `CEFDLY_1`"]
    #[inline(always)]
    pub fn is_cefdly_1(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_1
    }
    #[doc = "Checks if the value of the field is `CEFDLY_2`"]
    #[inline(always)]
    pub fn is_cefdly_2(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_2
    }
    #[doc = "Checks if the value of the field is `CEFDLY_3`"]
    #[inline(always)]
    pub fn is_cefdly_3(&self) -> bool {
        *self == CEFDLY_A::CEFDLY_3
    }
}
#[doc = "Write proxy for field `CEFDLY`"]
pub struct CEFDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CEFDLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEFDLY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
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
#[doc = "Reader of field `CEPWRMD`"]
pub type CEPWRMD_R = crate::R<u8, CEPWRMD_A>;
impl CEPWRMD_R {
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
        *self == CEPWRMD_A::CEPWRMD_0
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_1`"]
    #[inline(always)]
    pub fn is_cepwrmd_1(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_1
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_2`"]
    #[inline(always)]
    pub fn is_cepwrmd_2(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_2
    }
    #[doc = "Checks if the value of the field is `CEPWRMD_3`"]
    #[inline(always)]
    pub fn is_cepwrmd_3(&self) -> bool {
        *self == CEPWRMD_A::CEPWRMD_3
    }
}
#[doc = "Write proxy for field `CEPWRMD`"]
pub struct CEPWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPWRMD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CEON`"]
pub type CEON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEON`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CEMRVL`"]
pub type CEMRVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEMRVL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CEMRVS`"]
pub type CEMRVS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEMRVS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
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
}
