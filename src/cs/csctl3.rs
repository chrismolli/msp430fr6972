#[doc = "Reader of register CSCTL3"]
pub type R = crate::R<u16, super::CSCTL3>;
#[doc = "Writer for register CSCTL3"]
pub type W = crate::W<u16, super::CSCTL3>;
#[doc = "Register CSCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: MCLK Source Divider 0"]
    DIVM_0 = 0,
    #[doc = "1: MCLK Source Divider 1"]
    DIVM_1 = 1,
    #[doc = "2: MCLK Source Divider 2"]
    DIVM_2 = 2,
    #[doc = "3: MCLK Source Divider 3"]
    DIVM_3 = 3,
    #[doc = "4: MCLK Source Divider 4"]
    DIVM_4 = 4,
    #[doc = "5: MCLK Source Divider 5"]
    DIVM_5 = 5,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVM`"]
pub type DIVM_R = crate::R<u8, DIVM_A>;
impl DIVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVM_A::DIVM_0),
            1 => Val(DIVM_A::DIVM_1),
            2 => Val(DIVM_A::DIVM_2),
            3 => Val(DIVM_A::DIVM_3),
            4 => Val(DIVM_A::DIVM_4),
            5 => Val(DIVM_A::DIVM_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVM_0`"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVM_A::DIVM_0
    }
    #[doc = "Checks if the value of the field is `DIVM_1`"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVM_A::DIVM_1
    }
    #[doc = "Checks if the value of the field is `DIVM_2`"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVM_A::DIVM_2
    }
    #[doc = "Checks if the value of the field is `DIVM_3`"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVM_A::DIVM_3
    }
    #[doc = "Checks if the value of the field is `DIVM_4`"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == DIVM_A::DIVM_4
    }
    #[doc = "Checks if the value of the field is `DIVM_5`"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == DIVM_A::DIVM_5
    }
}
#[doc = "Write proxy for field `DIVM`"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_3)
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_4)
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "SMCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: SMCLK Source Divider 0"]
    DIVS_0 = 0,
    #[doc = "1: SMCLK Source Divider 1"]
    DIVS_1 = 1,
    #[doc = "2: SMCLK Source Divider 2"]
    DIVS_2 = 2,
    #[doc = "3: SMCLK Source Divider 3"]
    DIVS_3 = 3,
    #[doc = "4: SMCLK Source Divider 4"]
    DIVS_4 = 4,
    #[doc = "5: SMCLK Source Divider 5"]
    DIVS_5 = 5,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVS`"]
pub type DIVS_R = crate::R<u8, DIVS_A>;
impl DIVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVS_A::DIVS_0),
            1 => Val(DIVS_A::DIVS_1),
            2 => Val(DIVS_A::DIVS_2),
            3 => Val(DIVS_A::DIVS_3),
            4 => Val(DIVS_A::DIVS_4),
            5 => Val(DIVS_A::DIVS_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVS_0`"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVS_A::DIVS_0
    }
    #[doc = "Checks if the value of the field is `DIVS_1`"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVS_A::DIVS_1
    }
    #[doc = "Checks if the value of the field is `DIVS_2`"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVS_A::DIVS_2
    }
    #[doc = "Checks if the value of the field is `DIVS_3`"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVS_A::DIVS_3
    }
    #[doc = "Checks if the value of the field is `DIVS_4`"]
    #[inline(always)]
    pub fn is_divs_4(&self) -> bool {
        *self == DIVS_A::DIVS_4
    }
    #[doc = "Checks if the value of the field is `DIVS_5`"]
    #[inline(always)]
    pub fn is_divs_5(&self) -> bool {
        *self == DIVS_A::DIVS_5
    }
}
#[doc = "Write proxy for field `DIVS`"]
pub struct DIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_3)
    }
    #[doc = "SMCLK Source Divider 4"]
    #[inline(always)]
    pub fn divs_4(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_4)
    }
    #[doc = "SMCLK Source Divider 5"]
    #[inline(always)]
    pub fn divs_5(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "ACLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: ACLK Source Divider 0"]
    DIVA_0 = 0,
    #[doc = "1: ACLK Source Divider 1"]
    DIVA_1 = 1,
    #[doc = "2: ACLK Source Divider 2"]
    DIVA_2 = 2,
    #[doc = "3: ACLK Source Divider 3"]
    DIVA_3 = 3,
    #[doc = "4: ACLK Source Divider 4"]
    DIVA_4 = 4,
    #[doc = "5: ACLK Source Divider 5"]
    DIVA_5 = 5,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVA_A::DIVA_0),
            1 => Val(DIVA_A::DIVA_1),
            2 => Val(DIVA_A::DIVA_2),
            3 => Val(DIVA_A::DIVA_3),
            4 => Val(DIVA_A::DIVA_4),
            5 => Val(DIVA_A::DIVA_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVA_0`"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "Checks if the value of the field is `DIVA_1`"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "Checks if the value of the field is `DIVA_2`"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "Checks if the value of the field is `DIVA_3`"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
    #[doc = "Checks if the value of the field is `DIVA_4`"]
    #[inline(always)]
    pub fn is_diva_4(&self) -> bool {
        *self == DIVA_A::DIVA_4
    }
    #[doc = "Checks if the value of the field is `DIVA_5`"]
    #[inline(always)]
    pub fn is_diva_5(&self) -> bool {
        *self == DIVA_A::DIVA_5
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ACLK Source Divider 0"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "ACLK Source Divider 1"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "ACLK Source Divider 2"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "ACLK Source Divider 3"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_3)
    }
    #[doc = "ACLK Source Divider 4"]
    #[inline(always)]
    pub fn diva_4(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_4)
    }
    #[doc = "ACLK Source Divider 5"]
    #[inline(always)]
    pub fn diva_5(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - ACLK Divider Bit: 0"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bits 4:6 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W { w: self }
    }
    #[doc = "Bits 8:10 - ACLK Divider Bit: 0"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
}
