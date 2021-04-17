#[doc = "Reader of register CECTL0"]
pub type R = crate::R<u16, super::CECTL0>;
#[doc = "Writer for register CECTL0"]
pub type W = crate::W<u16, super::CECTL0>;
#[doc = "Register CECTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CECTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comp. E Pos. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEIPSEL_A {
    #[doc = "0: Comp. E V+ terminal Input Select: Channel 0"]
    CEIPSEL_0 = 0,
    #[doc = "1: Comp. E V+ terminal Input Select: Channel 1"]
    CEIPSEL_1 = 1,
    #[doc = "2: Comp. E V+ terminal Input Select: Channel 2"]
    CEIPSEL_2 = 2,
    #[doc = "3: Comp. E V+ terminal Input Select: Channel 3"]
    CEIPSEL_3 = 3,
    #[doc = "4: Comp. E V+ terminal Input Select: Channel 4"]
    CEIPSEL_4 = 4,
    #[doc = "5: Comp. E V+ terminal Input Select: Channel 5"]
    CEIPSEL_5 = 5,
    #[doc = "6: Comp. E V+ terminal Input Select: Channel 6"]
    CEIPSEL_6 = 6,
    #[doc = "7: Comp. E V+ terminal Input Select: Channel 7"]
    CEIPSEL_7 = 7,
    #[doc = "8: Comp. E V+ terminal Input Select: Channel 8"]
    CEIPSEL_8 = 8,
    #[doc = "9: Comp. E V+ terminal Input Select: Channel 9"]
    CEIPSEL_9 = 9,
    #[doc = "10: Comp. E V+ terminal Input Select: Channel 10"]
    CEIPSEL_10 = 10,
    #[doc = "11: Comp. E V+ terminal Input Select: Channel 11"]
    CEIPSEL_11 = 11,
    #[doc = "12: Comp. E V+ terminal Input Select: Channel 12"]
    CEIPSEL_12 = 12,
    #[doc = "13: Comp. E V+ terminal Input Select: Channel 13"]
    CEIPSEL_13 = 13,
    #[doc = "14: Comp. E V+ terminal Input Select: Channel 14"]
    CEIPSEL_14 = 14,
    #[doc = "15: Comp. E V+ terminal Input Select: Channel 15"]
    CEIPSEL_15 = 15,
}
impl From<CEIPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEIPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIPSEL`"]
pub type CEIPSEL_R = crate::R<u8, CEIPSEL_A>;
impl CEIPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIPSEL_A {
        match self.bits {
            0 => CEIPSEL_A::CEIPSEL_0,
            1 => CEIPSEL_A::CEIPSEL_1,
            2 => CEIPSEL_A::CEIPSEL_2,
            3 => CEIPSEL_A::CEIPSEL_3,
            4 => CEIPSEL_A::CEIPSEL_4,
            5 => CEIPSEL_A::CEIPSEL_5,
            6 => CEIPSEL_A::CEIPSEL_6,
            7 => CEIPSEL_A::CEIPSEL_7,
            8 => CEIPSEL_A::CEIPSEL_8,
            9 => CEIPSEL_A::CEIPSEL_9,
            10 => CEIPSEL_A::CEIPSEL_10,
            11 => CEIPSEL_A::CEIPSEL_11,
            12 => CEIPSEL_A::CEIPSEL_12,
            13 => CEIPSEL_A::CEIPSEL_13,
            14 => CEIPSEL_A::CEIPSEL_14,
            15 => CEIPSEL_A::CEIPSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_0`"]
    #[inline(always)]
    pub fn is_ceipsel_0(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_0
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_1`"]
    #[inline(always)]
    pub fn is_ceipsel_1(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_1
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_2`"]
    #[inline(always)]
    pub fn is_ceipsel_2(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_2
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_3`"]
    #[inline(always)]
    pub fn is_ceipsel_3(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_3
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_4`"]
    #[inline(always)]
    pub fn is_ceipsel_4(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_4
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_5`"]
    #[inline(always)]
    pub fn is_ceipsel_5(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_5
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_6`"]
    #[inline(always)]
    pub fn is_ceipsel_6(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_6
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_7`"]
    #[inline(always)]
    pub fn is_ceipsel_7(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_7
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_8`"]
    #[inline(always)]
    pub fn is_ceipsel_8(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_8
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_9`"]
    #[inline(always)]
    pub fn is_ceipsel_9(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_9
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_10`"]
    #[inline(always)]
    pub fn is_ceipsel_10(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_10
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_11`"]
    #[inline(always)]
    pub fn is_ceipsel_11(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_11
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_12`"]
    #[inline(always)]
    pub fn is_ceipsel_12(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_12
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_13`"]
    #[inline(always)]
    pub fn is_ceipsel_13(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_13
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_14`"]
    #[inline(always)]
    pub fn is_ceipsel_14(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_14
    }
    #[doc = "Checks if the value of the field is `CEIPSEL_15`"]
    #[inline(always)]
    pub fn is_ceipsel_15(&self) -> bool {
        *self == CEIPSEL_A::CEIPSEL_15
    }
}
#[doc = "Write proxy for field `CEIPSEL`"]
pub struct CEIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIPSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn ceipsel_0(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_0)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn ceipsel_1(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_1)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn ceipsel_2(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_2)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn ceipsel_3(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_3)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn ceipsel_4(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_4)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn ceipsel_5(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_5)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn ceipsel_6(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_6)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn ceipsel_7(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_7)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn ceipsel_8(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_8)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn ceipsel_9(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_9)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn ceipsel_10(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_10)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn ceipsel_11(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_11)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn ceipsel_12(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_12)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn ceipsel_13(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_13)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn ceipsel_14(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_14)
    }
    #[doc = "Comp. E V+ terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn ceipsel_15(self) -> &'a mut W {
        self.variant(CEIPSEL_A::CEIPSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CEIPEN`"]
pub type CEIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIPEN`"]
pub struct CEIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Comp. E Neg. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CEIMSEL_A {
    #[doc = "0: Comp. E V- Terminal Input Select: Channel 0"]
    CEIMSEL_0 = 0,
    #[doc = "1: Comp. E V- Terminal Input Select: Channel 1"]
    CEIMSEL_1 = 1,
    #[doc = "2: Comp. E V- Terminal Input Select: Channel 2"]
    CEIMSEL_2 = 2,
    #[doc = "3: Comp. E V- Terminal Input Select: Channel 3"]
    CEIMSEL_3 = 3,
    #[doc = "4: Comp. E V- Terminal Input Select: Channel 4"]
    CEIMSEL_4 = 4,
    #[doc = "5: Comp. E V- Terminal Input Select: Channel 5"]
    CEIMSEL_5 = 5,
    #[doc = "6: Comp. E V- Terminal Input Select: Channel 6"]
    CEIMSEL_6 = 6,
    #[doc = "7: Comp. E V- Terminal Input Select: Channel 7"]
    CEIMSEL_7 = 7,
    #[doc = "8: Comp. E V- terminal Input Select: Channel 8"]
    CEIMSEL_8 = 8,
    #[doc = "9: Comp. E V- terminal Input Select: Channel 9"]
    CEIMSEL_9 = 9,
    #[doc = "10: Comp. E V- terminal Input Select: Channel 10"]
    CEIMSEL_10 = 10,
    #[doc = "11: Comp. E V- terminal Input Select: Channel 11"]
    CEIMSEL_11 = 11,
    #[doc = "12: Comp. E V- terminal Input Select: Channel 12"]
    CEIMSEL_12 = 12,
    #[doc = "13: Comp. E V- terminal Input Select: Channel 13"]
    CEIMSEL_13 = 13,
    #[doc = "14: Comp. E V- terminal Input Select: Channel 14"]
    CEIMSEL_14 = 14,
    #[doc = "15: Comp. E V- terminal Input Select: Channel 15"]
    CEIMSEL_15 = 15,
}
impl From<CEIMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEIMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CEIMSEL`"]
pub type CEIMSEL_R = crate::R<u8, CEIMSEL_A>;
impl CEIMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIMSEL_A {
        match self.bits {
            0 => CEIMSEL_A::CEIMSEL_0,
            1 => CEIMSEL_A::CEIMSEL_1,
            2 => CEIMSEL_A::CEIMSEL_2,
            3 => CEIMSEL_A::CEIMSEL_3,
            4 => CEIMSEL_A::CEIMSEL_4,
            5 => CEIMSEL_A::CEIMSEL_5,
            6 => CEIMSEL_A::CEIMSEL_6,
            7 => CEIMSEL_A::CEIMSEL_7,
            8 => CEIMSEL_A::CEIMSEL_8,
            9 => CEIMSEL_A::CEIMSEL_9,
            10 => CEIMSEL_A::CEIMSEL_10,
            11 => CEIMSEL_A::CEIMSEL_11,
            12 => CEIMSEL_A::CEIMSEL_12,
            13 => CEIMSEL_A::CEIMSEL_13,
            14 => CEIMSEL_A::CEIMSEL_14,
            15 => CEIMSEL_A::CEIMSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_0`"]
    #[inline(always)]
    pub fn is_ceimsel_0(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_0
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_1`"]
    #[inline(always)]
    pub fn is_ceimsel_1(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_1
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_2`"]
    #[inline(always)]
    pub fn is_ceimsel_2(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_2
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_3`"]
    #[inline(always)]
    pub fn is_ceimsel_3(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_3
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_4`"]
    #[inline(always)]
    pub fn is_ceimsel_4(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_4
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_5`"]
    #[inline(always)]
    pub fn is_ceimsel_5(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_5
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_6`"]
    #[inline(always)]
    pub fn is_ceimsel_6(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_6
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_7`"]
    #[inline(always)]
    pub fn is_ceimsel_7(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_7
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_8`"]
    #[inline(always)]
    pub fn is_ceimsel_8(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_8
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_9`"]
    #[inline(always)]
    pub fn is_ceimsel_9(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_9
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_10`"]
    #[inline(always)]
    pub fn is_ceimsel_10(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_10
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_11`"]
    #[inline(always)]
    pub fn is_ceimsel_11(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_11
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_12`"]
    #[inline(always)]
    pub fn is_ceimsel_12(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_12
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_13`"]
    #[inline(always)]
    pub fn is_ceimsel_13(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_13
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_14`"]
    #[inline(always)]
    pub fn is_ceimsel_14(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_14
    }
    #[doc = "Checks if the value of the field is `CEIMSEL_15`"]
    #[inline(always)]
    pub fn is_ceimsel_15(&self) -> bool {
        *self == CEIMSEL_A::CEIMSEL_15
    }
}
#[doc = "Write proxy for field `CEIMSEL`"]
pub struct CEIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIMSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn ceimsel_0(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_0)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn ceimsel_1(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_1)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn ceimsel_2(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_2)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn ceimsel_3(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_3)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn ceimsel_4(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_4)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn ceimsel_5(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_5)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn ceimsel_6(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_6)
    }
    #[doc = "Comp. E V- Terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn ceimsel_7(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_7)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn ceimsel_8(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_8)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn ceimsel_9(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_9)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn ceimsel_10(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_10)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn ceimsel_11(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_11)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn ceimsel_12(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_12)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn ceimsel_13(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_13)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn ceimsel_14(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_14)
    }
    #[doc = "Comp. E V- terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn ceimsel_15(self) -> &'a mut W {
        self.variant(CEIMSEL_A::CEIMSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CEIMEN`"]
pub type CEIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEIMEN`"]
pub struct CEIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIMEN_W<'a> {
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
    #[doc = "Bits 0:3 - Comp. E Pos. Channel Input Select 0"]
    #[inline(always)]
    pub fn ceipsel(&self) -> CEIPSEL_R {
        CEIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Comp. E Pos. Channel Input Enable"]
    #[inline(always)]
    pub fn ceipen(&self) -> CEIPEN_R {
        CEIPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Comp. E Neg. Channel Input Select 0"]
    #[inline(always)]
    pub fn ceimsel(&self) -> CEIMSEL_R {
        CEIMSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comp. E Neg. Channel Input Enable"]
    #[inline(always)]
    pub fn ceimen(&self) -> CEIMEN_R {
        CEIMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comp. E Pos. Channel Input Select 0"]
    #[inline(always)]
    pub fn ceipsel(&mut self) -> CEIPSEL_W {
        CEIPSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comp. E Pos. Channel Input Enable"]
    #[inline(always)]
    pub fn ceipen(&mut self) -> CEIPEN_W {
        CEIPEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Comp. E Neg. Channel Input Select 0"]
    #[inline(always)]
    pub fn ceimsel(&mut self) -> CEIMSEL_W {
        CEIMSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comp. E Neg. Channel Input Enable"]
    #[inline(always)]
    pub fn ceimen(&mut self) -> CEIMEN_W {
        CEIMEN_W { w: self }
    }
}
