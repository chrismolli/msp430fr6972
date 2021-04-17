#[doc = "Reader of register CECTL2"]
pub type R = crate::R<u16, super::CECTL2>;
#[doc = "Writer for register CECTL2"]
pub type W = crate::W<u16, super::CECTL2>;
#[doc = "Register CECTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CECTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comp. E Reference 0 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREF0_A {
    #[doc = "0: Comp. E Int. Ref.0 Select 0 : 1/32"]
    CEREF0_0 = 0,
    #[doc = "1: Comp. E Int. Ref.0 Select 1 : 2/32"]
    CEREF0_1 = 1,
    #[doc = "2: Comp. E Int. Ref.0 Select 2 : 3/32"]
    CEREF0_2 = 2,
    #[doc = "3: Comp. E Int. Ref.0 Select 3 : 4/32"]
    CEREF0_3 = 3,
    #[doc = "4: Comp. E Int. Ref.0 Select 4 : 5/32"]
    CEREF0_4 = 4,
    #[doc = "5: Comp. E Int. Ref.0 Select 5 : 6/32"]
    CEREF0_5 = 5,
    #[doc = "6: Comp. E Int. Ref.0 Select 6 : 7/32"]
    CEREF0_6 = 6,
    #[doc = "7: Comp. E Int. Ref.0 Select 7 : 8/32"]
    CEREF0_7 = 7,
    #[doc = "8: Comp. E Int. Ref.0 Select 0 : 9/32"]
    CEREF0_8 = 8,
    #[doc = "9: Comp. E Int. Ref.0 Select 1 : 10/32"]
    CEREF0_9 = 9,
    #[doc = "10: Comp. E Int. Ref.0 Select 2 : 11/32"]
    CEREF0_10 = 10,
    #[doc = "11: Comp. E Int. Ref.0 Select 3 : 12/32"]
    CEREF0_11 = 11,
    #[doc = "12: Comp. E Int. Ref.0 Select 4 : 13/32"]
    CEREF0_12 = 12,
    #[doc = "13: Comp. E Int. Ref.0 Select 5 : 14/32"]
    CEREF0_13 = 13,
    #[doc = "14: Comp. E Int. Ref.0 Select 6 : 15/32"]
    CEREF0_14 = 14,
    #[doc = "15: Comp. E Int. Ref.0 Select 7 : 16/32"]
    CEREF0_15 = 15,
    #[doc = "16: Comp. E Int. Ref.0 Select 0 : 17/32"]
    CEREF0_16 = 16,
    #[doc = "17: Comp. E Int. Ref.0 Select 1 : 18/32"]
    CEREF0_17 = 17,
    #[doc = "18: Comp. E Int. Ref.0 Select 2 : 19/32"]
    CEREF0_18 = 18,
    #[doc = "19: Comp. E Int. Ref.0 Select 3 : 20/32"]
    CEREF0_19 = 19,
    #[doc = "20: Comp. E Int. Ref.0 Select 4 : 21/32"]
    CEREF0_20 = 20,
    #[doc = "21: Comp. E Int. Ref.0 Select 5 : 22/32"]
    CEREF0_21 = 21,
    #[doc = "22: Comp. E Int. Ref.0 Select 6 : 23/32"]
    CEREF0_22 = 22,
    #[doc = "23: Comp. E Int. Ref.0 Select 7 : 24/32"]
    CEREF0_23 = 23,
    #[doc = "24: Comp. E Int. Ref.0 Select 0 : 25/32"]
    CEREF0_24 = 24,
    #[doc = "25: Comp. E Int. Ref.0 Select 1 : 26/32"]
    CEREF0_25 = 25,
    #[doc = "26: Comp. E Int. Ref.0 Select 2 : 27/32"]
    CEREF0_26 = 26,
    #[doc = "27: Comp. E Int. Ref.0 Select 3 : 28/32"]
    CEREF0_27 = 27,
    #[doc = "28: Comp. E Int. Ref.0 Select 4 : 29/32"]
    CEREF0_28 = 28,
    #[doc = "29: Comp. E Int. Ref.0 Select 5 : 30/32"]
    CEREF0_29 = 29,
    #[doc = "30: Comp. E Int. Ref.0 Select 6 : 31/32"]
    CEREF0_30 = 30,
    #[doc = "31: Comp. E Int. Ref.0 Select 7 : 32/32"]
    CEREF0_31 = 31,
}
impl From<CEREF0_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREF0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREF0`"]
pub type CEREF0_R = crate::R<u8, CEREF0_A>;
impl CEREF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREF0_A {
        match self.bits {
            0 => CEREF0_A::CEREF0_0,
            1 => CEREF0_A::CEREF0_1,
            2 => CEREF0_A::CEREF0_2,
            3 => CEREF0_A::CEREF0_3,
            4 => CEREF0_A::CEREF0_4,
            5 => CEREF0_A::CEREF0_5,
            6 => CEREF0_A::CEREF0_6,
            7 => CEREF0_A::CEREF0_7,
            8 => CEREF0_A::CEREF0_8,
            9 => CEREF0_A::CEREF0_9,
            10 => CEREF0_A::CEREF0_10,
            11 => CEREF0_A::CEREF0_11,
            12 => CEREF0_A::CEREF0_12,
            13 => CEREF0_A::CEREF0_13,
            14 => CEREF0_A::CEREF0_14,
            15 => CEREF0_A::CEREF0_15,
            16 => CEREF0_A::CEREF0_16,
            17 => CEREF0_A::CEREF0_17,
            18 => CEREF0_A::CEREF0_18,
            19 => CEREF0_A::CEREF0_19,
            20 => CEREF0_A::CEREF0_20,
            21 => CEREF0_A::CEREF0_21,
            22 => CEREF0_A::CEREF0_22,
            23 => CEREF0_A::CEREF0_23,
            24 => CEREF0_A::CEREF0_24,
            25 => CEREF0_A::CEREF0_25,
            26 => CEREF0_A::CEREF0_26,
            27 => CEREF0_A::CEREF0_27,
            28 => CEREF0_A::CEREF0_28,
            29 => CEREF0_A::CEREF0_29,
            30 => CEREF0_A::CEREF0_30,
            31 => CEREF0_A::CEREF0_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREF0_0`"]
    #[inline(always)]
    pub fn is_ceref0_0(&self) -> bool {
        *self == CEREF0_A::CEREF0_0
    }
    #[doc = "Checks if the value of the field is `CEREF0_1`"]
    #[inline(always)]
    pub fn is_ceref0_1(&self) -> bool {
        *self == CEREF0_A::CEREF0_1
    }
    #[doc = "Checks if the value of the field is `CEREF0_2`"]
    #[inline(always)]
    pub fn is_ceref0_2(&self) -> bool {
        *self == CEREF0_A::CEREF0_2
    }
    #[doc = "Checks if the value of the field is `CEREF0_3`"]
    #[inline(always)]
    pub fn is_ceref0_3(&self) -> bool {
        *self == CEREF0_A::CEREF0_3
    }
    #[doc = "Checks if the value of the field is `CEREF0_4`"]
    #[inline(always)]
    pub fn is_ceref0_4(&self) -> bool {
        *self == CEREF0_A::CEREF0_4
    }
    #[doc = "Checks if the value of the field is `CEREF0_5`"]
    #[inline(always)]
    pub fn is_ceref0_5(&self) -> bool {
        *self == CEREF0_A::CEREF0_5
    }
    #[doc = "Checks if the value of the field is `CEREF0_6`"]
    #[inline(always)]
    pub fn is_ceref0_6(&self) -> bool {
        *self == CEREF0_A::CEREF0_6
    }
    #[doc = "Checks if the value of the field is `CEREF0_7`"]
    #[inline(always)]
    pub fn is_ceref0_7(&self) -> bool {
        *self == CEREF0_A::CEREF0_7
    }
    #[doc = "Checks if the value of the field is `CEREF0_8`"]
    #[inline(always)]
    pub fn is_ceref0_8(&self) -> bool {
        *self == CEREF0_A::CEREF0_8
    }
    #[doc = "Checks if the value of the field is `CEREF0_9`"]
    #[inline(always)]
    pub fn is_ceref0_9(&self) -> bool {
        *self == CEREF0_A::CEREF0_9
    }
    #[doc = "Checks if the value of the field is `CEREF0_10`"]
    #[inline(always)]
    pub fn is_ceref0_10(&self) -> bool {
        *self == CEREF0_A::CEREF0_10
    }
    #[doc = "Checks if the value of the field is `CEREF0_11`"]
    #[inline(always)]
    pub fn is_ceref0_11(&self) -> bool {
        *self == CEREF0_A::CEREF0_11
    }
    #[doc = "Checks if the value of the field is `CEREF0_12`"]
    #[inline(always)]
    pub fn is_ceref0_12(&self) -> bool {
        *self == CEREF0_A::CEREF0_12
    }
    #[doc = "Checks if the value of the field is `CEREF0_13`"]
    #[inline(always)]
    pub fn is_ceref0_13(&self) -> bool {
        *self == CEREF0_A::CEREF0_13
    }
    #[doc = "Checks if the value of the field is `CEREF0_14`"]
    #[inline(always)]
    pub fn is_ceref0_14(&self) -> bool {
        *self == CEREF0_A::CEREF0_14
    }
    #[doc = "Checks if the value of the field is `CEREF0_15`"]
    #[inline(always)]
    pub fn is_ceref0_15(&self) -> bool {
        *self == CEREF0_A::CEREF0_15
    }
    #[doc = "Checks if the value of the field is `CEREF0_16`"]
    #[inline(always)]
    pub fn is_ceref0_16(&self) -> bool {
        *self == CEREF0_A::CEREF0_16
    }
    #[doc = "Checks if the value of the field is `CEREF0_17`"]
    #[inline(always)]
    pub fn is_ceref0_17(&self) -> bool {
        *self == CEREF0_A::CEREF0_17
    }
    #[doc = "Checks if the value of the field is `CEREF0_18`"]
    #[inline(always)]
    pub fn is_ceref0_18(&self) -> bool {
        *self == CEREF0_A::CEREF0_18
    }
    #[doc = "Checks if the value of the field is `CEREF0_19`"]
    #[inline(always)]
    pub fn is_ceref0_19(&self) -> bool {
        *self == CEREF0_A::CEREF0_19
    }
    #[doc = "Checks if the value of the field is `CEREF0_20`"]
    #[inline(always)]
    pub fn is_ceref0_20(&self) -> bool {
        *self == CEREF0_A::CEREF0_20
    }
    #[doc = "Checks if the value of the field is `CEREF0_21`"]
    #[inline(always)]
    pub fn is_ceref0_21(&self) -> bool {
        *self == CEREF0_A::CEREF0_21
    }
    #[doc = "Checks if the value of the field is `CEREF0_22`"]
    #[inline(always)]
    pub fn is_ceref0_22(&self) -> bool {
        *self == CEREF0_A::CEREF0_22
    }
    #[doc = "Checks if the value of the field is `CEREF0_23`"]
    #[inline(always)]
    pub fn is_ceref0_23(&self) -> bool {
        *self == CEREF0_A::CEREF0_23
    }
    #[doc = "Checks if the value of the field is `CEREF0_24`"]
    #[inline(always)]
    pub fn is_ceref0_24(&self) -> bool {
        *self == CEREF0_A::CEREF0_24
    }
    #[doc = "Checks if the value of the field is `CEREF0_25`"]
    #[inline(always)]
    pub fn is_ceref0_25(&self) -> bool {
        *self == CEREF0_A::CEREF0_25
    }
    #[doc = "Checks if the value of the field is `CEREF0_26`"]
    #[inline(always)]
    pub fn is_ceref0_26(&self) -> bool {
        *self == CEREF0_A::CEREF0_26
    }
    #[doc = "Checks if the value of the field is `CEREF0_27`"]
    #[inline(always)]
    pub fn is_ceref0_27(&self) -> bool {
        *self == CEREF0_A::CEREF0_27
    }
    #[doc = "Checks if the value of the field is `CEREF0_28`"]
    #[inline(always)]
    pub fn is_ceref0_28(&self) -> bool {
        *self == CEREF0_A::CEREF0_28
    }
    #[doc = "Checks if the value of the field is `CEREF0_29`"]
    #[inline(always)]
    pub fn is_ceref0_29(&self) -> bool {
        *self == CEREF0_A::CEREF0_29
    }
    #[doc = "Checks if the value of the field is `CEREF0_30`"]
    #[inline(always)]
    pub fn is_ceref0_30(&self) -> bool {
        *self == CEREF0_A::CEREF0_30
    }
    #[doc = "Checks if the value of the field is `CEREF0_31`"]
    #[inline(always)]
    pub fn is_ceref0_31(&self) -> bool {
        *self == CEREF0_A::CEREF0_31
    }
}
#[doc = "Write proxy for field `CEREF0`"]
pub struct CEREF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREF0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E Int. Ref.0 Select 0 : 1/32"]
    #[inline(always)]
    pub fn ceref0_0(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_0)
    }
    #[doc = "Comp. E Int. Ref.0 Select 1 : 2/32"]
    #[inline(always)]
    pub fn ceref0_1(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_1)
    }
    #[doc = "Comp. E Int. Ref.0 Select 2 : 3/32"]
    #[inline(always)]
    pub fn ceref0_2(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_2)
    }
    #[doc = "Comp. E Int. Ref.0 Select 3 : 4/32"]
    #[inline(always)]
    pub fn ceref0_3(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_3)
    }
    #[doc = "Comp. E Int. Ref.0 Select 4 : 5/32"]
    #[inline(always)]
    pub fn ceref0_4(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_4)
    }
    #[doc = "Comp. E Int. Ref.0 Select 5 : 6/32"]
    #[inline(always)]
    pub fn ceref0_5(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_5)
    }
    #[doc = "Comp. E Int. Ref.0 Select 6 : 7/32"]
    #[inline(always)]
    pub fn ceref0_6(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_6)
    }
    #[doc = "Comp. E Int. Ref.0 Select 7 : 8/32"]
    #[inline(always)]
    pub fn ceref0_7(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_7)
    }
    #[doc = "Comp. E Int. Ref.0 Select 0 : 9/32"]
    #[inline(always)]
    pub fn ceref0_8(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_8)
    }
    #[doc = "Comp. E Int. Ref.0 Select 1 : 10/32"]
    #[inline(always)]
    pub fn ceref0_9(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_9)
    }
    #[doc = "Comp. E Int. Ref.0 Select 2 : 11/32"]
    #[inline(always)]
    pub fn ceref0_10(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_10)
    }
    #[doc = "Comp. E Int. Ref.0 Select 3 : 12/32"]
    #[inline(always)]
    pub fn ceref0_11(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_11)
    }
    #[doc = "Comp. E Int. Ref.0 Select 4 : 13/32"]
    #[inline(always)]
    pub fn ceref0_12(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_12)
    }
    #[doc = "Comp. E Int. Ref.0 Select 5 : 14/32"]
    #[inline(always)]
    pub fn ceref0_13(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_13)
    }
    #[doc = "Comp. E Int. Ref.0 Select 6 : 15/32"]
    #[inline(always)]
    pub fn ceref0_14(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_14)
    }
    #[doc = "Comp. E Int. Ref.0 Select 7 : 16/32"]
    #[inline(always)]
    pub fn ceref0_15(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_15)
    }
    #[doc = "Comp. E Int. Ref.0 Select 0 : 17/32"]
    #[inline(always)]
    pub fn ceref0_16(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_16)
    }
    #[doc = "Comp. E Int. Ref.0 Select 1 : 18/32"]
    #[inline(always)]
    pub fn ceref0_17(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_17)
    }
    #[doc = "Comp. E Int. Ref.0 Select 2 : 19/32"]
    #[inline(always)]
    pub fn ceref0_18(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_18)
    }
    #[doc = "Comp. E Int. Ref.0 Select 3 : 20/32"]
    #[inline(always)]
    pub fn ceref0_19(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_19)
    }
    #[doc = "Comp. E Int. Ref.0 Select 4 : 21/32"]
    #[inline(always)]
    pub fn ceref0_20(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_20)
    }
    #[doc = "Comp. E Int. Ref.0 Select 5 : 22/32"]
    #[inline(always)]
    pub fn ceref0_21(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_21)
    }
    #[doc = "Comp. E Int. Ref.0 Select 6 : 23/32"]
    #[inline(always)]
    pub fn ceref0_22(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_22)
    }
    #[doc = "Comp. E Int. Ref.0 Select 7 : 24/32"]
    #[inline(always)]
    pub fn ceref0_23(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_23)
    }
    #[doc = "Comp. E Int. Ref.0 Select 0 : 25/32"]
    #[inline(always)]
    pub fn ceref0_24(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_24)
    }
    #[doc = "Comp. E Int. Ref.0 Select 1 : 26/32"]
    #[inline(always)]
    pub fn ceref0_25(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_25)
    }
    #[doc = "Comp. E Int. Ref.0 Select 2 : 27/32"]
    #[inline(always)]
    pub fn ceref0_26(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_26)
    }
    #[doc = "Comp. E Int. Ref.0 Select 3 : 28/32"]
    #[inline(always)]
    pub fn ceref0_27(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_27)
    }
    #[doc = "Comp. E Int. Ref.0 Select 4 : 29/32"]
    #[inline(always)]
    pub fn ceref0_28(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_28)
    }
    #[doc = "Comp. E Int. Ref.0 Select 5 : 30/32"]
    #[inline(always)]
    pub fn ceref0_29(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_29)
    }
    #[doc = "Comp. E Int. Ref.0 Select 6 : 31/32"]
    #[inline(always)]
    pub fn ceref0_30(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_30)
    }
    #[doc = "Comp. E Int. Ref.0 Select 7 : 32/32"]
    #[inline(always)]
    pub fn ceref0_31(self) -> &'a mut W {
        self.variant(CEREF0_A::CEREF0_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CERSEL`"]
pub type CERSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERSEL`"]
pub struct CERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CERSEL_W<'a> {
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
#[doc = "Comp. E Reference Source Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CERS_A {
    #[doc = "0: Comp. E Reference Source 0 : Off"]
    CERS_0 = 0,
    #[doc = "1: Comp. E Reference Source 1 : Vcc"]
    CERS_1 = 1,
    #[doc = "2: Comp. E Reference Source 2 : Shared Ref."]
    CERS_2 = 2,
    #[doc = "3: Comp. E Reference Source 3 : Shared Ref. / Off"]
    CERS_3 = 3,
}
impl From<CERS_A> for u8 {
    #[inline(always)]
    fn from(variant: CERS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CERS`"]
pub type CERS_R = crate::R<u8, CERS_A>;
impl CERS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERS_A {
        match self.bits {
            0 => CERS_A::CERS_0,
            1 => CERS_A::CERS_1,
            2 => CERS_A::CERS_2,
            3 => CERS_A::CERS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CERS_0`"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == CERS_A::CERS_0
    }
    #[doc = "Checks if the value of the field is `CERS_1`"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == CERS_A::CERS_1
    }
    #[doc = "Checks if the value of the field is `CERS_2`"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == CERS_A::CERS_2
    }
    #[doc = "Checks if the value of the field is `CERS_3`"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == CERS_A::CERS_3
    }
}
#[doc = "Write proxy for field `CERS`"]
pub struct CERS_W<'a> {
    w: &'a mut W,
}
impl<'a> CERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CERS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E Reference Source 0 : Off"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut W {
        self.variant(CERS_A::CERS_0)
    }
    #[doc = "Comp. E Reference Source 1 : Vcc"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut W {
        self.variant(CERS_A::CERS_1)
    }
    #[doc = "Comp. E Reference Source 2 : Shared Ref."]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut W {
        self.variant(CERS_A::CERS_2)
    }
    #[doc = "Comp. E Reference Source 3 : Shared Ref. / Off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut W {
        self.variant(CERS_A::CERS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Comp. E Reference 1 Resistor Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREF1_A {
    #[doc = "0: Comp. E Int. Ref.1 Select 0 : 1/32"]
    CEREF1_0 = 0,
    #[doc = "1: Comp. E Int. Ref.1 Select 1 : 2/32"]
    CEREF1_1 = 1,
    #[doc = "2: Comp. E Int. Ref.1 Select 2 : 3/32"]
    CEREF1_2 = 2,
    #[doc = "3: Comp. E Int. Ref.1 Select 3 : 4/32"]
    CEREF1_3 = 3,
    #[doc = "4: Comp. E Int. Ref.1 Select 4 : 5/32"]
    CEREF1_4 = 4,
    #[doc = "5: Comp. E Int. Ref.1 Select 5 : 6/32"]
    CEREF1_5 = 5,
    #[doc = "6: Comp. E Int. Ref.1 Select 6 : 7/32"]
    CEREF1_6 = 6,
    #[doc = "7: Comp. E Int. Ref.1 Select 7 : 8/32"]
    CEREF1_7 = 7,
    #[doc = "8: Comp. E Int. Ref.1 Select 0 : 9/32"]
    CEREF1_8 = 8,
    #[doc = "9: Comp. E Int. Ref.1 Select 1 : 10/32"]
    CEREF1_9 = 9,
    #[doc = "10: Comp. E Int. Ref.1 Select 2 : 11/32"]
    CEREF1_10 = 10,
    #[doc = "11: Comp. E Int. Ref.1 Select 3 : 12/32"]
    CEREF1_11 = 11,
    #[doc = "12: Comp. E Int. Ref.1 Select 4 : 13/32"]
    CEREF1_12 = 12,
    #[doc = "13: Comp. E Int. Ref.1 Select 5 : 14/32"]
    CEREF1_13 = 13,
    #[doc = "14: Comp. E Int. Ref.1 Select 6 : 15/32"]
    CEREF1_14 = 14,
    #[doc = "15: Comp. E Int. Ref.1 Select 7 : 16/32"]
    CEREF1_15 = 15,
    #[doc = "16: Comp. E Int. Ref.1 Select 0 : 17/32"]
    CEREF1_16 = 16,
    #[doc = "17: Comp. E Int. Ref.1 Select 1 : 18/32"]
    CEREF1_17 = 17,
    #[doc = "18: Comp. E Int. Ref.1 Select 2 : 19/32"]
    CEREF1_18 = 18,
    #[doc = "19: Comp. E Int. Ref.1 Select 3 : 20/32"]
    CEREF1_19 = 19,
    #[doc = "20: Comp. E Int. Ref.1 Select 4 : 21/32"]
    CEREF1_20 = 20,
    #[doc = "21: Comp. E Int. Ref.1 Select 5 : 22/32"]
    CEREF1_21 = 21,
    #[doc = "22: Comp. E Int. Ref.1 Select 6 : 23/32"]
    CEREF1_22 = 22,
    #[doc = "23: Comp. E Int. Ref.1 Select 7 : 24/32"]
    CEREF1_23 = 23,
    #[doc = "24: Comp. E Int. Ref.1 Select 0 : 25/32"]
    CEREF1_24 = 24,
    #[doc = "25: Comp. E Int. Ref.1 Select 1 : 26/32"]
    CEREF1_25 = 25,
    #[doc = "26: Comp. E Int. Ref.1 Select 2 : 27/32"]
    CEREF1_26 = 26,
    #[doc = "27: Comp. E Int. Ref.1 Select 3 : 28/32"]
    CEREF1_27 = 27,
    #[doc = "28: Comp. E Int. Ref.1 Select 4 : 29/32"]
    CEREF1_28 = 28,
    #[doc = "29: Comp. E Int. Ref.1 Select 5 : 30/32"]
    CEREF1_29 = 29,
    #[doc = "30: Comp. E Int. Ref.1 Select 6 : 31/32"]
    CEREF1_30 = 30,
    #[doc = "31: Comp. E Int. Ref.1 Select 7 : 32/32"]
    CEREF1_31 = 31,
}
impl From<CEREF1_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREF1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREF1`"]
pub type CEREF1_R = crate::R<u8, CEREF1_A>;
impl CEREF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREF1_A {
        match self.bits {
            0 => CEREF1_A::CEREF1_0,
            1 => CEREF1_A::CEREF1_1,
            2 => CEREF1_A::CEREF1_2,
            3 => CEREF1_A::CEREF1_3,
            4 => CEREF1_A::CEREF1_4,
            5 => CEREF1_A::CEREF1_5,
            6 => CEREF1_A::CEREF1_6,
            7 => CEREF1_A::CEREF1_7,
            8 => CEREF1_A::CEREF1_8,
            9 => CEREF1_A::CEREF1_9,
            10 => CEREF1_A::CEREF1_10,
            11 => CEREF1_A::CEREF1_11,
            12 => CEREF1_A::CEREF1_12,
            13 => CEREF1_A::CEREF1_13,
            14 => CEREF1_A::CEREF1_14,
            15 => CEREF1_A::CEREF1_15,
            16 => CEREF1_A::CEREF1_16,
            17 => CEREF1_A::CEREF1_17,
            18 => CEREF1_A::CEREF1_18,
            19 => CEREF1_A::CEREF1_19,
            20 => CEREF1_A::CEREF1_20,
            21 => CEREF1_A::CEREF1_21,
            22 => CEREF1_A::CEREF1_22,
            23 => CEREF1_A::CEREF1_23,
            24 => CEREF1_A::CEREF1_24,
            25 => CEREF1_A::CEREF1_25,
            26 => CEREF1_A::CEREF1_26,
            27 => CEREF1_A::CEREF1_27,
            28 => CEREF1_A::CEREF1_28,
            29 => CEREF1_A::CEREF1_29,
            30 => CEREF1_A::CEREF1_30,
            31 => CEREF1_A::CEREF1_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREF1_0`"]
    #[inline(always)]
    pub fn is_ceref1_0(&self) -> bool {
        *self == CEREF1_A::CEREF1_0
    }
    #[doc = "Checks if the value of the field is `CEREF1_1`"]
    #[inline(always)]
    pub fn is_ceref1_1(&self) -> bool {
        *self == CEREF1_A::CEREF1_1
    }
    #[doc = "Checks if the value of the field is `CEREF1_2`"]
    #[inline(always)]
    pub fn is_ceref1_2(&self) -> bool {
        *self == CEREF1_A::CEREF1_2
    }
    #[doc = "Checks if the value of the field is `CEREF1_3`"]
    #[inline(always)]
    pub fn is_ceref1_3(&self) -> bool {
        *self == CEREF1_A::CEREF1_3
    }
    #[doc = "Checks if the value of the field is `CEREF1_4`"]
    #[inline(always)]
    pub fn is_ceref1_4(&self) -> bool {
        *self == CEREF1_A::CEREF1_4
    }
    #[doc = "Checks if the value of the field is `CEREF1_5`"]
    #[inline(always)]
    pub fn is_ceref1_5(&self) -> bool {
        *self == CEREF1_A::CEREF1_5
    }
    #[doc = "Checks if the value of the field is `CEREF1_6`"]
    #[inline(always)]
    pub fn is_ceref1_6(&self) -> bool {
        *self == CEREF1_A::CEREF1_6
    }
    #[doc = "Checks if the value of the field is `CEREF1_7`"]
    #[inline(always)]
    pub fn is_ceref1_7(&self) -> bool {
        *self == CEREF1_A::CEREF1_7
    }
    #[doc = "Checks if the value of the field is `CEREF1_8`"]
    #[inline(always)]
    pub fn is_ceref1_8(&self) -> bool {
        *self == CEREF1_A::CEREF1_8
    }
    #[doc = "Checks if the value of the field is `CEREF1_9`"]
    #[inline(always)]
    pub fn is_ceref1_9(&self) -> bool {
        *self == CEREF1_A::CEREF1_9
    }
    #[doc = "Checks if the value of the field is `CEREF1_10`"]
    #[inline(always)]
    pub fn is_ceref1_10(&self) -> bool {
        *self == CEREF1_A::CEREF1_10
    }
    #[doc = "Checks if the value of the field is `CEREF1_11`"]
    #[inline(always)]
    pub fn is_ceref1_11(&self) -> bool {
        *self == CEREF1_A::CEREF1_11
    }
    #[doc = "Checks if the value of the field is `CEREF1_12`"]
    #[inline(always)]
    pub fn is_ceref1_12(&self) -> bool {
        *self == CEREF1_A::CEREF1_12
    }
    #[doc = "Checks if the value of the field is `CEREF1_13`"]
    #[inline(always)]
    pub fn is_ceref1_13(&self) -> bool {
        *self == CEREF1_A::CEREF1_13
    }
    #[doc = "Checks if the value of the field is `CEREF1_14`"]
    #[inline(always)]
    pub fn is_ceref1_14(&self) -> bool {
        *self == CEREF1_A::CEREF1_14
    }
    #[doc = "Checks if the value of the field is `CEREF1_15`"]
    #[inline(always)]
    pub fn is_ceref1_15(&self) -> bool {
        *self == CEREF1_A::CEREF1_15
    }
    #[doc = "Checks if the value of the field is `CEREF1_16`"]
    #[inline(always)]
    pub fn is_ceref1_16(&self) -> bool {
        *self == CEREF1_A::CEREF1_16
    }
    #[doc = "Checks if the value of the field is `CEREF1_17`"]
    #[inline(always)]
    pub fn is_ceref1_17(&self) -> bool {
        *self == CEREF1_A::CEREF1_17
    }
    #[doc = "Checks if the value of the field is `CEREF1_18`"]
    #[inline(always)]
    pub fn is_ceref1_18(&self) -> bool {
        *self == CEREF1_A::CEREF1_18
    }
    #[doc = "Checks if the value of the field is `CEREF1_19`"]
    #[inline(always)]
    pub fn is_ceref1_19(&self) -> bool {
        *self == CEREF1_A::CEREF1_19
    }
    #[doc = "Checks if the value of the field is `CEREF1_20`"]
    #[inline(always)]
    pub fn is_ceref1_20(&self) -> bool {
        *self == CEREF1_A::CEREF1_20
    }
    #[doc = "Checks if the value of the field is `CEREF1_21`"]
    #[inline(always)]
    pub fn is_ceref1_21(&self) -> bool {
        *self == CEREF1_A::CEREF1_21
    }
    #[doc = "Checks if the value of the field is `CEREF1_22`"]
    #[inline(always)]
    pub fn is_ceref1_22(&self) -> bool {
        *self == CEREF1_A::CEREF1_22
    }
    #[doc = "Checks if the value of the field is `CEREF1_23`"]
    #[inline(always)]
    pub fn is_ceref1_23(&self) -> bool {
        *self == CEREF1_A::CEREF1_23
    }
    #[doc = "Checks if the value of the field is `CEREF1_24`"]
    #[inline(always)]
    pub fn is_ceref1_24(&self) -> bool {
        *self == CEREF1_A::CEREF1_24
    }
    #[doc = "Checks if the value of the field is `CEREF1_25`"]
    #[inline(always)]
    pub fn is_ceref1_25(&self) -> bool {
        *self == CEREF1_A::CEREF1_25
    }
    #[doc = "Checks if the value of the field is `CEREF1_26`"]
    #[inline(always)]
    pub fn is_ceref1_26(&self) -> bool {
        *self == CEREF1_A::CEREF1_26
    }
    #[doc = "Checks if the value of the field is `CEREF1_27`"]
    #[inline(always)]
    pub fn is_ceref1_27(&self) -> bool {
        *self == CEREF1_A::CEREF1_27
    }
    #[doc = "Checks if the value of the field is `CEREF1_28`"]
    #[inline(always)]
    pub fn is_ceref1_28(&self) -> bool {
        *self == CEREF1_A::CEREF1_28
    }
    #[doc = "Checks if the value of the field is `CEREF1_29`"]
    #[inline(always)]
    pub fn is_ceref1_29(&self) -> bool {
        *self == CEREF1_A::CEREF1_29
    }
    #[doc = "Checks if the value of the field is `CEREF1_30`"]
    #[inline(always)]
    pub fn is_ceref1_30(&self) -> bool {
        *self == CEREF1_A::CEREF1_30
    }
    #[doc = "Checks if the value of the field is `CEREF1_31`"]
    #[inline(always)]
    pub fn is_ceref1_31(&self) -> bool {
        *self == CEREF1_A::CEREF1_31
    }
}
#[doc = "Write proxy for field `CEREF1`"]
pub struct CEREF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREF1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E Int. Ref.1 Select 0 : 1/32"]
    #[inline(always)]
    pub fn ceref1_0(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_0)
    }
    #[doc = "Comp. E Int. Ref.1 Select 1 : 2/32"]
    #[inline(always)]
    pub fn ceref1_1(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_1)
    }
    #[doc = "Comp. E Int. Ref.1 Select 2 : 3/32"]
    #[inline(always)]
    pub fn ceref1_2(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_2)
    }
    #[doc = "Comp. E Int. Ref.1 Select 3 : 4/32"]
    #[inline(always)]
    pub fn ceref1_3(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_3)
    }
    #[doc = "Comp. E Int. Ref.1 Select 4 : 5/32"]
    #[inline(always)]
    pub fn ceref1_4(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_4)
    }
    #[doc = "Comp. E Int. Ref.1 Select 5 : 6/32"]
    #[inline(always)]
    pub fn ceref1_5(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_5)
    }
    #[doc = "Comp. E Int. Ref.1 Select 6 : 7/32"]
    #[inline(always)]
    pub fn ceref1_6(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_6)
    }
    #[doc = "Comp. E Int. Ref.1 Select 7 : 8/32"]
    #[inline(always)]
    pub fn ceref1_7(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_7)
    }
    #[doc = "Comp. E Int. Ref.1 Select 0 : 9/32"]
    #[inline(always)]
    pub fn ceref1_8(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_8)
    }
    #[doc = "Comp. E Int. Ref.1 Select 1 : 10/32"]
    #[inline(always)]
    pub fn ceref1_9(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_9)
    }
    #[doc = "Comp. E Int. Ref.1 Select 2 : 11/32"]
    #[inline(always)]
    pub fn ceref1_10(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_10)
    }
    #[doc = "Comp. E Int. Ref.1 Select 3 : 12/32"]
    #[inline(always)]
    pub fn ceref1_11(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_11)
    }
    #[doc = "Comp. E Int. Ref.1 Select 4 : 13/32"]
    #[inline(always)]
    pub fn ceref1_12(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_12)
    }
    #[doc = "Comp. E Int. Ref.1 Select 5 : 14/32"]
    #[inline(always)]
    pub fn ceref1_13(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_13)
    }
    #[doc = "Comp. E Int. Ref.1 Select 6 : 15/32"]
    #[inline(always)]
    pub fn ceref1_14(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_14)
    }
    #[doc = "Comp. E Int. Ref.1 Select 7 : 16/32"]
    #[inline(always)]
    pub fn ceref1_15(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_15)
    }
    #[doc = "Comp. E Int. Ref.1 Select 0 : 17/32"]
    #[inline(always)]
    pub fn ceref1_16(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_16)
    }
    #[doc = "Comp. E Int. Ref.1 Select 1 : 18/32"]
    #[inline(always)]
    pub fn ceref1_17(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_17)
    }
    #[doc = "Comp. E Int. Ref.1 Select 2 : 19/32"]
    #[inline(always)]
    pub fn ceref1_18(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_18)
    }
    #[doc = "Comp. E Int. Ref.1 Select 3 : 20/32"]
    #[inline(always)]
    pub fn ceref1_19(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_19)
    }
    #[doc = "Comp. E Int. Ref.1 Select 4 : 21/32"]
    #[inline(always)]
    pub fn ceref1_20(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_20)
    }
    #[doc = "Comp. E Int. Ref.1 Select 5 : 22/32"]
    #[inline(always)]
    pub fn ceref1_21(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_21)
    }
    #[doc = "Comp. E Int. Ref.1 Select 6 : 23/32"]
    #[inline(always)]
    pub fn ceref1_22(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_22)
    }
    #[doc = "Comp. E Int. Ref.1 Select 7 : 24/32"]
    #[inline(always)]
    pub fn ceref1_23(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_23)
    }
    #[doc = "Comp. E Int. Ref.1 Select 0 : 25/32"]
    #[inline(always)]
    pub fn ceref1_24(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_24)
    }
    #[doc = "Comp. E Int. Ref.1 Select 1 : 26/32"]
    #[inline(always)]
    pub fn ceref1_25(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_25)
    }
    #[doc = "Comp. E Int. Ref.1 Select 2 : 27/32"]
    #[inline(always)]
    pub fn ceref1_26(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_26)
    }
    #[doc = "Comp. E Int. Ref.1 Select 3 : 28/32"]
    #[inline(always)]
    pub fn ceref1_27(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_27)
    }
    #[doc = "Comp. E Int. Ref.1 Select 4 : 29/32"]
    #[inline(always)]
    pub fn ceref1_28(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_28)
    }
    #[doc = "Comp. E Int. Ref.1 Select 5 : 30/32"]
    #[inline(always)]
    pub fn ceref1_29(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_29)
    }
    #[doc = "Comp. E Int. Ref.1 Select 6 : 31/32"]
    #[inline(always)]
    pub fn ceref1_30(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_30)
    }
    #[doc = "Comp. E Int. Ref.1 Select 7 : 32/32"]
    #[inline(always)]
    pub fn ceref1_31(self) -> &'a mut W {
        self.variant(CEREF1_A::CEREF1_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Comp. E Reference voltage level Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEREFL_A {
    #[doc = "0: Comp. E Reference voltage level 0 : None"]
    CEREFL_0 = 0,
    #[doc = "1: Comp. E Reference voltage level 1 : 1.2V"]
    CEREFL_1 = 1,
    #[doc = "2: Comp. E Reference voltage level 2 : 2.0V"]
    CEREFL_2 = 2,
    #[doc = "3: Comp. E Reference voltage level 3 : 2.5V"]
    CEREFL_3 = 3,
}
impl From<CEREFL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREFL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEREFL`"]
pub type CEREFL_R = crate::R<u8, CEREFL_A>;
impl CEREFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFL_A {
        match self.bits {
            0 => CEREFL_A::CEREFL_0,
            1 => CEREFL_A::CEREFL_1,
            2 => CEREFL_A::CEREFL_2,
            3 => CEREFL_A::CEREFL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREFL_0`"]
    #[inline(always)]
    pub fn is_cerefl_0(&self) -> bool {
        *self == CEREFL_A::CEREFL_0
    }
    #[doc = "Checks if the value of the field is `CEREFL_1`"]
    #[inline(always)]
    pub fn is_cerefl_1(&self) -> bool {
        *self == CEREFL_A::CEREFL_1
    }
    #[doc = "Checks if the value of the field is `CEREFL_2`"]
    #[inline(always)]
    pub fn is_cerefl_2(&self) -> bool {
        *self == CEREFL_A::CEREFL_2
    }
    #[doc = "Checks if the value of the field is `CEREFL_3`"]
    #[inline(always)]
    pub fn is_cerefl_3(&self) -> bool {
        *self == CEREFL_A::CEREFL_3
    }
}
#[doc = "Write proxy for field `CEREFL`"]
pub struct CEREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEREFL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E Reference voltage level 0 : None"]
    #[inline(always)]
    pub fn cerefl_0(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_0)
    }
    #[doc = "Comp. E Reference voltage level 1 : 1.2V"]
    #[inline(always)]
    pub fn cerefl_1(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_1)
    }
    #[doc = "Comp. E Reference voltage level 2 : 2.0V"]
    #[inline(always)]
    pub fn cerefl_2(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_2)
    }
    #[doc = "Comp. E Reference voltage level 3 : 2.5V"]
    #[inline(always)]
    pub fn cerefl_3(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `CEREFACC`"]
pub type CEREFACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEREFACC`"]
pub struct CEREFACC_W<'a> {
    w: &'a mut W,
}
impl<'a> CEREFACC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Comp. E Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> CEREF0_R {
        CEREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Comp. E Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CERSEL_R {
        CERSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Comp. E Reference Source Bit : 0"]
    #[inline(always)]
    pub fn cers(&self) -> CERS_R {
        CERS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Comp. E Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn ceref1(&self) -> CEREF1_R {
        CEREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Comp. E Reference voltage level Bit : 0"]
    #[inline(always)]
    pub fn cerefl(&self) -> CEREFL_R {
        CEREFL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Comp. E Reference Accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CEREFACC_R {
        CEREFACC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Comp. E Reference 0 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> CEREF0_W {
        CEREF0_W { w: self }
    }
    #[doc = "Bit 5 - Comp. E Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CERSEL_W {
        CERSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Comp. E Reference Source Bit : 0"]
    #[inline(always)]
    pub fn cers(&mut self) -> CERS_W {
        CERS_W { w: self }
    }
    #[doc = "Bits 8:12 - Comp. E Reference 1 Resistor Select Bit : 0"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> CEREF1_W {
        CEREF1_W { w: self }
    }
    #[doc = "Bits 13:14 - Comp. E Reference voltage level Bit : 0"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CEREFL_W {
        CEREFL_W { w: self }
    }
    #[doc = "Bit 15 - Comp. E Reference Accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CEREFACC_W {
        CEREFACC_W { w: self }
    }
}
