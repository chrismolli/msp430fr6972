#[doc = "Reader of register ADC12CTL3"]
pub type R = crate::R<u16, super::ADC12CTL3>;
#[doc = "Writer for register ADC12CTL3"]
pub type W = crate::W<u16, super::ADC12CTL3>;
#[doc = "Register ADC12CTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC12 Conversion Start Address Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12CSTARTADD_A {
    #[doc = "0: ADC12 Conversion Start Address: 0"]
    ADC12CSTARTADD_0 = 0,
    #[doc = "1: ADC12 Conversion Start Address: 1"]
    ADC12CSTARTADD_1 = 1,
    #[doc = "2: ADC12 Conversion Start Address: 2"]
    ADC12CSTARTADD_2 = 2,
    #[doc = "3: ADC12 Conversion Start Address: 3"]
    ADC12CSTARTADD_3 = 3,
    #[doc = "4: ADC12 Conversion Start Address: 4"]
    ADC12CSTARTADD_4 = 4,
    #[doc = "5: ADC12 Conversion Start Address: 5"]
    ADC12CSTARTADD_5 = 5,
    #[doc = "6: ADC12 Conversion Start Address: 6"]
    ADC12CSTARTADD_6 = 6,
    #[doc = "7: ADC12 Conversion Start Address: 7"]
    ADC12CSTARTADD_7 = 7,
    #[doc = "8: ADC12 Conversion Start Address: 8"]
    ADC12CSTARTADD_8 = 8,
    #[doc = "9: ADC12 Conversion Start Address: 9"]
    ADC12CSTARTADD_9 = 9,
    #[doc = "10: ADC12 Conversion Start Address: 10"]
    ADC12CSTARTADD_10 = 10,
    #[doc = "11: ADC12 Conversion Start Address: 11"]
    ADC12CSTARTADD_11 = 11,
    #[doc = "12: ADC12 Conversion Start Address: 12"]
    ADC12CSTARTADD_12 = 12,
    #[doc = "13: ADC12 Conversion Start Address: 13"]
    ADC12CSTARTADD_13 = 13,
    #[doc = "14: ADC12 Conversion Start Address: 14"]
    ADC12CSTARTADD_14 = 14,
    #[doc = "15: ADC12 Conversion Start Address: 15"]
    ADC12CSTARTADD_15 = 15,
    #[doc = "16: ADC12 Conversion Start Address: 16"]
    ADC12CSTARTADD_16 = 16,
    #[doc = "17: ADC12 Conversion Start Address: 17"]
    ADC12CSTARTADD_17 = 17,
    #[doc = "18: ADC12 Conversion Start Address: 18"]
    ADC12CSTARTADD_18 = 18,
    #[doc = "19: ADC12 Conversion Start Address: 19"]
    ADC12CSTARTADD_19 = 19,
    #[doc = "20: ADC12 Conversion Start Address: 20"]
    ADC12CSTARTADD_20 = 20,
    #[doc = "21: ADC12 Conversion Start Address: 21"]
    ADC12CSTARTADD_21 = 21,
    #[doc = "22: ADC12 Conversion Start Address: 22"]
    ADC12CSTARTADD_22 = 22,
    #[doc = "23: ADC12 Conversion Start Address: 23"]
    ADC12CSTARTADD_23 = 23,
    #[doc = "24: ADC12 Conversion Start Address: 24"]
    ADC12CSTARTADD_24 = 24,
    #[doc = "25: ADC12 Conversion Start Address: 25"]
    ADC12CSTARTADD_25 = 25,
    #[doc = "26: ADC12 Conversion Start Address: 26"]
    ADC12CSTARTADD_26 = 26,
    #[doc = "27: ADC12 Conversion Start Address: 27"]
    ADC12CSTARTADD_27 = 27,
    #[doc = "28: ADC12 Conversion Start Address: 28"]
    ADC12CSTARTADD_28 = 28,
    #[doc = "29: ADC12 Conversion Start Address: 29"]
    ADC12CSTARTADD_29 = 29,
    #[doc = "30: ADC12 Conversion Start Address: 30"]
    ADC12CSTARTADD_30 = 30,
    #[doc = "31: ADC12 Conversion Start Address: 31"]
    ADC12CSTARTADD_31 = 31,
}
impl From<ADC12CSTARTADD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CSTARTADD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12CSTARTADD`"]
pub type ADC12CSTARTADD_R = crate::R<u8, ADC12CSTARTADD_A>;
impl ADC12CSTARTADD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CSTARTADD_A {
        match self.bits {
            0 => ADC12CSTARTADD_A::ADC12CSTARTADD_0,
            1 => ADC12CSTARTADD_A::ADC12CSTARTADD_1,
            2 => ADC12CSTARTADD_A::ADC12CSTARTADD_2,
            3 => ADC12CSTARTADD_A::ADC12CSTARTADD_3,
            4 => ADC12CSTARTADD_A::ADC12CSTARTADD_4,
            5 => ADC12CSTARTADD_A::ADC12CSTARTADD_5,
            6 => ADC12CSTARTADD_A::ADC12CSTARTADD_6,
            7 => ADC12CSTARTADD_A::ADC12CSTARTADD_7,
            8 => ADC12CSTARTADD_A::ADC12CSTARTADD_8,
            9 => ADC12CSTARTADD_A::ADC12CSTARTADD_9,
            10 => ADC12CSTARTADD_A::ADC12CSTARTADD_10,
            11 => ADC12CSTARTADD_A::ADC12CSTARTADD_11,
            12 => ADC12CSTARTADD_A::ADC12CSTARTADD_12,
            13 => ADC12CSTARTADD_A::ADC12CSTARTADD_13,
            14 => ADC12CSTARTADD_A::ADC12CSTARTADD_14,
            15 => ADC12CSTARTADD_A::ADC12CSTARTADD_15,
            16 => ADC12CSTARTADD_A::ADC12CSTARTADD_16,
            17 => ADC12CSTARTADD_A::ADC12CSTARTADD_17,
            18 => ADC12CSTARTADD_A::ADC12CSTARTADD_18,
            19 => ADC12CSTARTADD_A::ADC12CSTARTADD_19,
            20 => ADC12CSTARTADD_A::ADC12CSTARTADD_20,
            21 => ADC12CSTARTADD_A::ADC12CSTARTADD_21,
            22 => ADC12CSTARTADD_A::ADC12CSTARTADD_22,
            23 => ADC12CSTARTADD_A::ADC12CSTARTADD_23,
            24 => ADC12CSTARTADD_A::ADC12CSTARTADD_24,
            25 => ADC12CSTARTADD_A::ADC12CSTARTADD_25,
            26 => ADC12CSTARTADD_A::ADC12CSTARTADD_26,
            27 => ADC12CSTARTADD_A::ADC12CSTARTADD_27,
            28 => ADC12CSTARTADD_A::ADC12CSTARTADD_28,
            29 => ADC12CSTARTADD_A::ADC12CSTARTADD_29,
            30 => ADC12CSTARTADD_A::ADC12CSTARTADD_30,
            31 => ADC12CSTARTADD_A::ADC12CSTARTADD_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_0`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_0(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_0
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_1`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_1(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_1
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_2`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_2(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_2
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_3`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_3(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_3
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_4`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_4(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_4
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_5`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_5(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_5
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_6`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_6(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_6
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_7`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_7(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_7
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_8`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_8(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_8
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_9`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_9(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_9
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_10`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_10(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_10
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_11`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_11(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_11
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_12`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_12(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_12
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_13`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_13(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_13
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_14`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_14(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_14
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_15`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_15(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_15
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_16`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_16(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_16
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_17`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_17(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_17
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_18`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_18(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_18
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_19`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_19(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_19
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_20`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_20(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_20
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_21`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_21(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_21
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_22`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_22(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_22
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_23`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_23(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_23
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_24`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_24(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_24
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_25`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_25(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_25
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_26`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_26(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_26
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_27`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_27(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_27
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_28`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_28(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_28
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_29`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_29(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_29
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_30`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_30(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_30
    }
    #[doc = "Checks if the value of the field is `ADC12CSTARTADD_31`"]
    #[inline(always)]
    pub fn is_adc12cstartadd_31(&self) -> bool {
        *self == ADC12CSTARTADD_A::ADC12CSTARTADD_31
    }
}
#[doc = "Write proxy for field `ADC12CSTARTADD`"]
pub struct ADC12CSTARTADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12CSTARTADD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12CSTARTADD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Conversion Start Address: 0"]
    #[inline(always)]
    pub fn adc12cstartadd_0(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_0)
    }
    #[doc = "ADC12 Conversion Start Address: 1"]
    #[inline(always)]
    pub fn adc12cstartadd_1(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_1)
    }
    #[doc = "ADC12 Conversion Start Address: 2"]
    #[inline(always)]
    pub fn adc12cstartadd_2(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_2)
    }
    #[doc = "ADC12 Conversion Start Address: 3"]
    #[inline(always)]
    pub fn adc12cstartadd_3(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_3)
    }
    #[doc = "ADC12 Conversion Start Address: 4"]
    #[inline(always)]
    pub fn adc12cstartadd_4(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_4)
    }
    #[doc = "ADC12 Conversion Start Address: 5"]
    #[inline(always)]
    pub fn adc12cstartadd_5(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_5)
    }
    #[doc = "ADC12 Conversion Start Address: 6"]
    #[inline(always)]
    pub fn adc12cstartadd_6(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_6)
    }
    #[doc = "ADC12 Conversion Start Address: 7"]
    #[inline(always)]
    pub fn adc12cstartadd_7(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_7)
    }
    #[doc = "ADC12 Conversion Start Address: 8"]
    #[inline(always)]
    pub fn adc12cstartadd_8(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_8)
    }
    #[doc = "ADC12 Conversion Start Address: 9"]
    #[inline(always)]
    pub fn adc12cstartadd_9(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_9)
    }
    #[doc = "ADC12 Conversion Start Address: 10"]
    #[inline(always)]
    pub fn adc12cstartadd_10(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_10)
    }
    #[doc = "ADC12 Conversion Start Address: 11"]
    #[inline(always)]
    pub fn adc12cstartadd_11(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_11)
    }
    #[doc = "ADC12 Conversion Start Address: 12"]
    #[inline(always)]
    pub fn adc12cstartadd_12(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_12)
    }
    #[doc = "ADC12 Conversion Start Address: 13"]
    #[inline(always)]
    pub fn adc12cstartadd_13(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_13)
    }
    #[doc = "ADC12 Conversion Start Address: 14"]
    #[inline(always)]
    pub fn adc12cstartadd_14(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_14)
    }
    #[doc = "ADC12 Conversion Start Address: 15"]
    #[inline(always)]
    pub fn adc12cstartadd_15(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_15)
    }
    #[doc = "ADC12 Conversion Start Address: 16"]
    #[inline(always)]
    pub fn adc12cstartadd_16(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_16)
    }
    #[doc = "ADC12 Conversion Start Address: 17"]
    #[inline(always)]
    pub fn adc12cstartadd_17(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_17)
    }
    #[doc = "ADC12 Conversion Start Address: 18"]
    #[inline(always)]
    pub fn adc12cstartadd_18(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_18)
    }
    #[doc = "ADC12 Conversion Start Address: 19"]
    #[inline(always)]
    pub fn adc12cstartadd_19(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_19)
    }
    #[doc = "ADC12 Conversion Start Address: 20"]
    #[inline(always)]
    pub fn adc12cstartadd_20(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_20)
    }
    #[doc = "ADC12 Conversion Start Address: 21"]
    #[inline(always)]
    pub fn adc12cstartadd_21(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_21)
    }
    #[doc = "ADC12 Conversion Start Address: 22"]
    #[inline(always)]
    pub fn adc12cstartadd_22(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_22)
    }
    #[doc = "ADC12 Conversion Start Address: 23"]
    #[inline(always)]
    pub fn adc12cstartadd_23(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_23)
    }
    #[doc = "ADC12 Conversion Start Address: 24"]
    #[inline(always)]
    pub fn adc12cstartadd_24(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_24)
    }
    #[doc = "ADC12 Conversion Start Address: 25"]
    #[inline(always)]
    pub fn adc12cstartadd_25(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_25)
    }
    #[doc = "ADC12 Conversion Start Address: 26"]
    #[inline(always)]
    pub fn adc12cstartadd_26(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_26)
    }
    #[doc = "ADC12 Conversion Start Address: 27"]
    #[inline(always)]
    pub fn adc12cstartadd_27(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_27)
    }
    #[doc = "ADC12 Conversion Start Address: 28"]
    #[inline(always)]
    pub fn adc12cstartadd_28(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_28)
    }
    #[doc = "ADC12 Conversion Start Address: 29"]
    #[inline(always)]
    pub fn adc12cstartadd_29(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_29)
    }
    #[doc = "ADC12 Conversion Start Address: 30"]
    #[inline(always)]
    pub fn adc12cstartadd_30(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_30)
    }
    #[doc = "ADC12 Conversion Start Address: 31"]
    #[inline(always)]
    pub fn adc12cstartadd_31(self) -> &'a mut W {
        self.variant(ADC12CSTARTADD_A::ADC12CSTARTADD_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADC12BATMAP`"]
pub type ADC12BATMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12BATMAP`"]
pub struct ADC12BATMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12BATMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC12TCMAP`"]
pub type ADC12TCMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12TCMAP`"]
pub struct ADC12TCMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TCMAP_W<'a> {
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
#[doc = "Reader of field `ADC12ICH0MAP`"]
pub type ADC12ICH0MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ICH0MAP`"]
pub struct ADC12ICH0MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH0MAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADC12ICH1MAP`"]
pub type ADC12ICH1MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ICH1MAP`"]
pub struct ADC12ICH1MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH1MAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ADC12ICH2MAP`"]
pub type ADC12ICH2MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ICH2MAP`"]
pub struct ADC12ICH2MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH2MAP_W<'a> {
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
#[doc = "Reader of field `ADC12ICH3MAP`"]
pub type ADC12ICH3MAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ICH3MAP`"]
pub struct ADC12ICH3MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ICH3MAP_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - ADC12 Conversion Start Address Bit: 0"]
    #[inline(always)]
    pub fn adc12cstartadd(&self) -> ADC12CSTARTADD_R {
        ADC12CSTARTADD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - ADC12 Internal AVCC/2 select"]
    #[inline(always)]
    pub fn adc12batmap(&self) -> ADC12BATMAP_R {
        ADC12BATMAP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Internal TempSensor select"]
    #[inline(always)]
    pub fn adc12tcmap(&self) -> ADC12TCMAP_R {
        ADC12TCMAP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Internal Channel 0 select"]
    #[inline(always)]
    pub fn adc12ich0map(&self) -> ADC12ICH0MAP_R {
        ADC12ICH0MAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Internal Channel 1 select"]
    #[inline(always)]
    pub fn adc12ich1map(&self) -> ADC12ICH1MAP_R {
        ADC12ICH1MAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Internal Channel 2 select"]
    #[inline(always)]
    pub fn adc12ich2map(&self) -> ADC12ICH2MAP_R {
        ADC12ICH2MAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Internal Channel 3 select"]
    #[inline(always)]
    pub fn adc12ich3map(&self) -> ADC12ICH3MAP_R {
        ADC12ICH3MAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC12 Conversion Start Address Bit: 0"]
    #[inline(always)]
    pub fn adc12cstartadd(&mut self) -> ADC12CSTARTADD_W {
        ADC12CSTARTADD_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Internal AVCC/2 select"]
    #[inline(always)]
    pub fn adc12batmap(&mut self) -> ADC12BATMAP_W {
        ADC12BATMAP_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Internal TempSensor select"]
    #[inline(always)]
    pub fn adc12tcmap(&mut self) -> ADC12TCMAP_W {
        ADC12TCMAP_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Internal Channel 0 select"]
    #[inline(always)]
    pub fn adc12ich0map(&mut self) -> ADC12ICH0MAP_W {
        ADC12ICH0MAP_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Internal Channel 1 select"]
    #[inline(always)]
    pub fn adc12ich1map(&mut self) -> ADC12ICH1MAP_W {
        ADC12ICH1MAP_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Internal Channel 2 select"]
    #[inline(always)]
    pub fn adc12ich2map(&mut self) -> ADC12ICH2MAP_W {
        ADC12ICH2MAP_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Internal Channel 3 select"]
    #[inline(always)]
    pub fn adc12ich3map(&mut self) -> ADC12ICH3MAP_W {
        ADC12ICH3MAP_W { w: self }
    }
}
