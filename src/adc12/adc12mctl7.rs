#[doc = "Reader of register ADC12MCTL7"]
pub type R = crate::R<u16, super::ADC12MCTL7>;
#[doc = "Writer for register ADC12MCTL7"]
pub type W = crate::W<u16, super::ADC12MCTL7>;
#[doc = "Register ADC12MCTL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12MCTL7 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC12 Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12INCH_A {
    #[doc = "0: ADC12 Input Channel 0"]
    ADC12INCH_0 = 0,
    #[doc = "1: ADC12 Input Channel 1"]
    ADC12INCH_1 = 1,
    #[doc = "2: ADC12 Input Channel 2"]
    ADC12INCH_2 = 2,
    #[doc = "3: ADC12 Input Channel 3"]
    ADC12INCH_3 = 3,
    #[doc = "4: ADC12 Input Channel 4"]
    ADC12INCH_4 = 4,
    #[doc = "5: ADC12 Input Channel 5"]
    ADC12INCH_5 = 5,
    #[doc = "6: ADC12 Input Channel 6"]
    ADC12INCH_6 = 6,
    #[doc = "7: ADC12 Input Channel 7"]
    ADC12INCH_7 = 7,
    #[doc = "8: ADC12 Input Channel 8"]
    ADC12INCH_8 = 8,
    #[doc = "9: ADC12 Input Channel 9"]
    ADC12INCH_9 = 9,
    #[doc = "10: ADC12 Input Channel 10"]
    ADC12INCH_10 = 10,
    #[doc = "11: ADC12 Input Channel 11"]
    ADC12INCH_11 = 11,
    #[doc = "12: ADC12 Input Channel 12"]
    ADC12INCH_12 = 12,
    #[doc = "13: ADC12 Input Channel 13"]
    ADC12INCH_13 = 13,
    #[doc = "14: ADC12 Input Channel 14"]
    ADC12INCH_14 = 14,
    #[doc = "15: ADC12 Input Channel 15"]
    ADC12INCH_15 = 15,
    #[doc = "16: ADC12 Input Channel 16"]
    ADC12INCH_16 = 16,
    #[doc = "17: ADC12 Input Channel 17"]
    ADC12INCH_17 = 17,
    #[doc = "18: ADC12 Input Channel 18"]
    ADC12INCH_18 = 18,
    #[doc = "19: ADC12 Input Channel 19"]
    ADC12INCH_19 = 19,
    #[doc = "20: ADC12 Input Channel 20"]
    ADC12INCH_20 = 20,
    #[doc = "21: ADC12 Input Channel 21"]
    ADC12INCH_21 = 21,
    #[doc = "22: ADC12 Input Channel 22"]
    ADC12INCH_22 = 22,
    #[doc = "23: ADC12 Input Channel 23"]
    ADC12INCH_23 = 23,
    #[doc = "24: ADC12 Input Channel 24"]
    ADC12INCH_24 = 24,
    #[doc = "25: ADC12 Input Channel 25"]
    ADC12INCH_25 = 25,
    #[doc = "26: ADC12 Input Channel 26"]
    ADC12INCH_26 = 26,
    #[doc = "27: ADC12 Input Channel 27"]
    ADC12INCH_27 = 27,
    #[doc = "28: ADC12 Input Channel 28"]
    ADC12INCH_28 = 28,
    #[doc = "29: ADC12 Input Channel 29"]
    ADC12INCH_29 = 29,
    #[doc = "30: ADC12 Input Channel 30"]
    ADC12INCH_30 = 30,
    #[doc = "31: ADC12 Input Channel 31"]
    ADC12INCH_31 = 31,
}
impl From<ADC12INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12INCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12INCH`"]
pub type ADC12INCH_R = crate::R<u8, ADC12INCH_A>;
impl ADC12INCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12INCH_A {
        match self.bits {
            0 => ADC12INCH_A::ADC12INCH_0,
            1 => ADC12INCH_A::ADC12INCH_1,
            2 => ADC12INCH_A::ADC12INCH_2,
            3 => ADC12INCH_A::ADC12INCH_3,
            4 => ADC12INCH_A::ADC12INCH_4,
            5 => ADC12INCH_A::ADC12INCH_5,
            6 => ADC12INCH_A::ADC12INCH_6,
            7 => ADC12INCH_A::ADC12INCH_7,
            8 => ADC12INCH_A::ADC12INCH_8,
            9 => ADC12INCH_A::ADC12INCH_9,
            10 => ADC12INCH_A::ADC12INCH_10,
            11 => ADC12INCH_A::ADC12INCH_11,
            12 => ADC12INCH_A::ADC12INCH_12,
            13 => ADC12INCH_A::ADC12INCH_13,
            14 => ADC12INCH_A::ADC12INCH_14,
            15 => ADC12INCH_A::ADC12INCH_15,
            16 => ADC12INCH_A::ADC12INCH_16,
            17 => ADC12INCH_A::ADC12INCH_17,
            18 => ADC12INCH_A::ADC12INCH_18,
            19 => ADC12INCH_A::ADC12INCH_19,
            20 => ADC12INCH_A::ADC12INCH_20,
            21 => ADC12INCH_A::ADC12INCH_21,
            22 => ADC12INCH_A::ADC12INCH_22,
            23 => ADC12INCH_A::ADC12INCH_23,
            24 => ADC12INCH_A::ADC12INCH_24,
            25 => ADC12INCH_A::ADC12INCH_25,
            26 => ADC12INCH_A::ADC12INCH_26,
            27 => ADC12INCH_A::ADC12INCH_27,
            28 => ADC12INCH_A::ADC12INCH_28,
            29 => ADC12INCH_A::ADC12INCH_29,
            30 => ADC12INCH_A::ADC12INCH_30,
            31 => ADC12INCH_A::ADC12INCH_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_0`"]
    #[inline(always)]
    pub fn is_adc12inch_0(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_1`"]
    #[inline(always)]
    pub fn is_adc12inch_1(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_2`"]
    #[inline(always)]
    pub fn is_adc12inch_2(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_3`"]
    #[inline(always)]
    pub fn is_adc12inch_3(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_4`"]
    #[inline(always)]
    pub fn is_adc12inch_4(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_5`"]
    #[inline(always)]
    pub fn is_adc12inch_5(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_6`"]
    #[inline(always)]
    pub fn is_adc12inch_6(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_7`"]
    #[inline(always)]
    pub fn is_adc12inch_7(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_8`"]
    #[inline(always)]
    pub fn is_adc12inch_8(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_9`"]
    #[inline(always)]
    pub fn is_adc12inch_9(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_10`"]
    #[inline(always)]
    pub fn is_adc12inch_10(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_11`"]
    #[inline(always)]
    pub fn is_adc12inch_11(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_12`"]
    #[inline(always)]
    pub fn is_adc12inch_12(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_13`"]
    #[inline(always)]
    pub fn is_adc12inch_13(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_14`"]
    #[inline(always)]
    pub fn is_adc12inch_14(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_15`"]
    #[inline(always)]
    pub fn is_adc12inch_15(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_15
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_16`"]
    #[inline(always)]
    pub fn is_adc12inch_16(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_16
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_17`"]
    #[inline(always)]
    pub fn is_adc12inch_17(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_17
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_18`"]
    #[inline(always)]
    pub fn is_adc12inch_18(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_18
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_19`"]
    #[inline(always)]
    pub fn is_adc12inch_19(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_19
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_20`"]
    #[inline(always)]
    pub fn is_adc12inch_20(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_20
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_21`"]
    #[inline(always)]
    pub fn is_adc12inch_21(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_21
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_22`"]
    #[inline(always)]
    pub fn is_adc12inch_22(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_22
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_23`"]
    #[inline(always)]
    pub fn is_adc12inch_23(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_23
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_24`"]
    #[inline(always)]
    pub fn is_adc12inch_24(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_24
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_25`"]
    #[inline(always)]
    pub fn is_adc12inch_25(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_25
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_26`"]
    #[inline(always)]
    pub fn is_adc12inch_26(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_26
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_27`"]
    #[inline(always)]
    pub fn is_adc12inch_27(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_27
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_28`"]
    #[inline(always)]
    pub fn is_adc12inch_28(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_28
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_29`"]
    #[inline(always)]
    pub fn is_adc12inch_29(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_29
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_30`"]
    #[inline(always)]
    pub fn is_adc12inch_30(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_30
    }
    #[doc = "Checks if the value of the field is `ADC12INCH_31`"]
    #[inline(always)]
    pub fn is_adc12inch_31(&self) -> bool {
        *self == ADC12INCH_A::ADC12INCH_31
    }
}
#[doc = "Write proxy for field `ADC12INCH`"]
pub struct ADC12INCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12INCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Input Channel 0"]
    #[inline(always)]
    pub fn adc12inch_0(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_0)
    }
    #[doc = "ADC12 Input Channel 1"]
    #[inline(always)]
    pub fn adc12inch_1(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_1)
    }
    #[doc = "ADC12 Input Channel 2"]
    #[inline(always)]
    pub fn adc12inch_2(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_2)
    }
    #[doc = "ADC12 Input Channel 3"]
    #[inline(always)]
    pub fn adc12inch_3(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_3)
    }
    #[doc = "ADC12 Input Channel 4"]
    #[inline(always)]
    pub fn adc12inch_4(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_4)
    }
    #[doc = "ADC12 Input Channel 5"]
    #[inline(always)]
    pub fn adc12inch_5(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_5)
    }
    #[doc = "ADC12 Input Channel 6"]
    #[inline(always)]
    pub fn adc12inch_6(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_6)
    }
    #[doc = "ADC12 Input Channel 7"]
    #[inline(always)]
    pub fn adc12inch_7(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_7)
    }
    #[doc = "ADC12 Input Channel 8"]
    #[inline(always)]
    pub fn adc12inch_8(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_8)
    }
    #[doc = "ADC12 Input Channel 9"]
    #[inline(always)]
    pub fn adc12inch_9(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_9)
    }
    #[doc = "ADC12 Input Channel 10"]
    #[inline(always)]
    pub fn adc12inch_10(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_10)
    }
    #[doc = "ADC12 Input Channel 11"]
    #[inline(always)]
    pub fn adc12inch_11(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_11)
    }
    #[doc = "ADC12 Input Channel 12"]
    #[inline(always)]
    pub fn adc12inch_12(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_12)
    }
    #[doc = "ADC12 Input Channel 13"]
    #[inline(always)]
    pub fn adc12inch_13(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_13)
    }
    #[doc = "ADC12 Input Channel 14"]
    #[inline(always)]
    pub fn adc12inch_14(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_14)
    }
    #[doc = "ADC12 Input Channel 15"]
    #[inline(always)]
    pub fn adc12inch_15(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_15)
    }
    #[doc = "ADC12 Input Channel 16"]
    #[inline(always)]
    pub fn adc12inch_16(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_16)
    }
    #[doc = "ADC12 Input Channel 17"]
    #[inline(always)]
    pub fn adc12inch_17(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_17)
    }
    #[doc = "ADC12 Input Channel 18"]
    #[inline(always)]
    pub fn adc12inch_18(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_18)
    }
    #[doc = "ADC12 Input Channel 19"]
    #[inline(always)]
    pub fn adc12inch_19(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_19)
    }
    #[doc = "ADC12 Input Channel 20"]
    #[inline(always)]
    pub fn adc12inch_20(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_20)
    }
    #[doc = "ADC12 Input Channel 21"]
    #[inline(always)]
    pub fn adc12inch_21(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_21)
    }
    #[doc = "ADC12 Input Channel 22"]
    #[inline(always)]
    pub fn adc12inch_22(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_22)
    }
    #[doc = "ADC12 Input Channel 23"]
    #[inline(always)]
    pub fn adc12inch_23(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_23)
    }
    #[doc = "ADC12 Input Channel 24"]
    #[inline(always)]
    pub fn adc12inch_24(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_24)
    }
    #[doc = "ADC12 Input Channel 25"]
    #[inline(always)]
    pub fn adc12inch_25(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_25)
    }
    #[doc = "ADC12 Input Channel 26"]
    #[inline(always)]
    pub fn adc12inch_26(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_26)
    }
    #[doc = "ADC12 Input Channel 27"]
    #[inline(always)]
    pub fn adc12inch_27(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_27)
    }
    #[doc = "ADC12 Input Channel 28"]
    #[inline(always)]
    pub fn adc12inch_28(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_28)
    }
    #[doc = "ADC12 Input Channel 29"]
    #[inline(always)]
    pub fn adc12inch_29(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_29)
    }
    #[doc = "ADC12 Input Channel 30"]
    #[inline(always)]
    pub fn adc12inch_30(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_30)
    }
    #[doc = "ADC12 Input Channel 31"]
    #[inline(always)]
    pub fn adc12inch_31(self) -> &'a mut W {
        self.variant(ADC12INCH_A::ADC12INCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADC12EOS`"]
pub type ADC12EOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12EOS`"]
pub struct ADC12EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EOS_W<'a> {
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
#[doc = "ADC12 VR Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12VRSEL_A {
    #[doc = "0: ADC12 Select Reference 0"]
    ADC12VRSEL_0 = 0,
    #[doc = "1: ADC12 Select Reference 1"]
    ADC12VRSEL_1 = 1,
    #[doc = "2: ADC12 Select Reference 2"]
    ADC12VRSEL_2 = 2,
    #[doc = "3: ADC12 Select Reference 3"]
    ADC12VRSEL_3 = 3,
    #[doc = "4: ADC12 Select Reference 4"]
    ADC12VRSEL_4 = 4,
    #[doc = "5: ADC12 Select Reference 5"]
    ADC12VRSEL_5 = 5,
    #[doc = "6: ADC12 Select Reference 6"]
    ADC12VRSEL_6 = 6,
    #[doc = "7: ADC12 Select Reference 7"]
    ADC12VRSEL_7 = 7,
    #[doc = "8: ADC12 Select Reference 8"]
    ADC12VRSEL_8 = 8,
    #[doc = "9: ADC12 Select Reference 9"]
    ADC12VRSEL_9 = 9,
    #[doc = "10: ADC12 Select Reference 10"]
    ADC12VRSEL_10 = 10,
    #[doc = "11: ADC12 Select Reference 11"]
    ADC12VRSEL_11 = 11,
    #[doc = "12: ADC12 Select Reference 12"]
    ADC12VRSEL_12 = 12,
    #[doc = "13: ADC12 Select Reference 13"]
    ADC12VRSEL_13 = 13,
    #[doc = "14: ADC12 Select Reference 14"]
    ADC12VRSEL_14 = 14,
    #[doc = "15: ADC12 Select Reference 15"]
    ADC12VRSEL_15 = 15,
}
impl From<ADC12VRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12VRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12VRSEL`"]
pub type ADC12VRSEL_R = crate::R<u8, ADC12VRSEL_A>;
impl ADC12VRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12VRSEL_A {
        match self.bits {
            0 => ADC12VRSEL_A::ADC12VRSEL_0,
            1 => ADC12VRSEL_A::ADC12VRSEL_1,
            2 => ADC12VRSEL_A::ADC12VRSEL_2,
            3 => ADC12VRSEL_A::ADC12VRSEL_3,
            4 => ADC12VRSEL_A::ADC12VRSEL_4,
            5 => ADC12VRSEL_A::ADC12VRSEL_5,
            6 => ADC12VRSEL_A::ADC12VRSEL_6,
            7 => ADC12VRSEL_A::ADC12VRSEL_7,
            8 => ADC12VRSEL_A::ADC12VRSEL_8,
            9 => ADC12VRSEL_A::ADC12VRSEL_9,
            10 => ADC12VRSEL_A::ADC12VRSEL_10,
            11 => ADC12VRSEL_A::ADC12VRSEL_11,
            12 => ADC12VRSEL_A::ADC12VRSEL_12,
            13 => ADC12VRSEL_A::ADC12VRSEL_13,
            14 => ADC12VRSEL_A::ADC12VRSEL_14,
            15 => ADC12VRSEL_A::ADC12VRSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_0`"]
    #[inline(always)]
    pub fn is_adc12vrsel_0(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_1`"]
    #[inline(always)]
    pub fn is_adc12vrsel_1(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_2`"]
    #[inline(always)]
    pub fn is_adc12vrsel_2(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_3`"]
    #[inline(always)]
    pub fn is_adc12vrsel_3(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_3
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_4`"]
    #[inline(always)]
    pub fn is_adc12vrsel_4(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_4
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_5`"]
    #[inline(always)]
    pub fn is_adc12vrsel_5(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_5
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_6`"]
    #[inline(always)]
    pub fn is_adc12vrsel_6(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_6
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_7`"]
    #[inline(always)]
    pub fn is_adc12vrsel_7(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_7
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_8`"]
    #[inline(always)]
    pub fn is_adc12vrsel_8(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_8
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_9`"]
    #[inline(always)]
    pub fn is_adc12vrsel_9(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_9
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_10`"]
    #[inline(always)]
    pub fn is_adc12vrsel_10(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_10
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_11`"]
    #[inline(always)]
    pub fn is_adc12vrsel_11(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_11
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_12`"]
    #[inline(always)]
    pub fn is_adc12vrsel_12(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_12
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_13`"]
    #[inline(always)]
    pub fn is_adc12vrsel_13(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_13
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_14`"]
    #[inline(always)]
    pub fn is_adc12vrsel_14(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_14
    }
    #[doc = "Checks if the value of the field is `ADC12VRSEL_15`"]
    #[inline(always)]
    pub fn is_adc12vrsel_15(&self) -> bool {
        *self == ADC12VRSEL_A::ADC12VRSEL_15
    }
}
#[doc = "Write proxy for field `ADC12VRSEL`"]
pub struct ADC12VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12VRSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Select Reference 0"]
    #[inline(always)]
    pub fn adc12vrsel_0(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_0)
    }
    #[doc = "ADC12 Select Reference 1"]
    #[inline(always)]
    pub fn adc12vrsel_1(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_1)
    }
    #[doc = "ADC12 Select Reference 2"]
    #[inline(always)]
    pub fn adc12vrsel_2(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_2)
    }
    #[doc = "ADC12 Select Reference 3"]
    #[inline(always)]
    pub fn adc12vrsel_3(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_3)
    }
    #[doc = "ADC12 Select Reference 4"]
    #[inline(always)]
    pub fn adc12vrsel_4(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_4)
    }
    #[doc = "ADC12 Select Reference 5"]
    #[inline(always)]
    pub fn adc12vrsel_5(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_5)
    }
    #[doc = "ADC12 Select Reference 6"]
    #[inline(always)]
    pub fn adc12vrsel_6(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_6)
    }
    #[doc = "ADC12 Select Reference 7"]
    #[inline(always)]
    pub fn adc12vrsel_7(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_7)
    }
    #[doc = "ADC12 Select Reference 8"]
    #[inline(always)]
    pub fn adc12vrsel_8(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_8)
    }
    #[doc = "ADC12 Select Reference 9"]
    #[inline(always)]
    pub fn adc12vrsel_9(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_9)
    }
    #[doc = "ADC12 Select Reference 10"]
    #[inline(always)]
    pub fn adc12vrsel_10(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_10)
    }
    #[doc = "ADC12 Select Reference 11"]
    #[inline(always)]
    pub fn adc12vrsel_11(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_11)
    }
    #[doc = "ADC12 Select Reference 12"]
    #[inline(always)]
    pub fn adc12vrsel_12(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_12)
    }
    #[doc = "ADC12 Select Reference 13"]
    #[inline(always)]
    pub fn adc12vrsel_13(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_13)
    }
    #[doc = "ADC12 Select Reference 14"]
    #[inline(always)]
    pub fn adc12vrsel_14(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_14)
    }
    #[doc = "ADC12 Select Reference 15"]
    #[inline(always)]
    pub fn adc12vrsel_15(self) -> &'a mut W {
        self.variant(ADC12VRSEL_A::ADC12VRSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADC12DIF`"]
pub type ADC12DIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12DIF`"]
pub struct ADC12DIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ADC12WINC`"]
pub type ADC12WINC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12WINC`"]
pub struct ADC12WINC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12WINC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc12inch(&self) -> ADC12INCH_R {
        ADC12INCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn adc12eos(&self) -> ADC12EOS_R {
        ADC12EOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADC12 VR Select Bit 0"]
    #[inline(always)]
    pub fn adc12vrsel(&self) -> ADC12VRSEL_R {
        ADC12VRSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - ADC12 Differential mode (only for even Registers)"]
    #[inline(always)]
    pub fn adc12dif(&self) -> ADC12DIF_R {
        ADC12DIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&self) -> ADC12WINC_R {
        ADC12WINC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC12 Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adc12inch(&mut self) -> ADC12INCH_W {
        ADC12INCH_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 End of Sequence"]
    #[inline(always)]
    pub fn adc12eos(&mut self) -> ADC12EOS_W {
        ADC12EOS_W { w: self }
    }
    #[doc = "Bits 8:11 - ADC12 VR Select Bit 0"]
    #[inline(always)]
    pub fn adc12vrsel(&mut self) -> ADC12VRSEL_W {
        ADC12VRSEL_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Differential mode (only for even Registers)"]
    #[inline(always)]
    pub fn adc12dif(&mut self) -> ADC12DIF_W {
        ADC12DIF_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Comparator window enable"]
    #[inline(always)]
    pub fn adc12winc(&mut self) -> ADC12WINC_W {
        ADC12WINC_W { w: self }
    }
}
