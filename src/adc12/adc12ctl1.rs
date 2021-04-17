#[doc = "Reader of register ADC12CTL1"]
pub type R = crate::R<u16, super::ADC12CTL1>;
#[doc = "Writer for register ADC12CTL1"]
pub type W = crate::W<u16, super::ADC12CTL1>;
#[doc = "Register ADC12CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12BUSY`"]
pub type ADC12BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12BUSY`"]
pub struct ADC12BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12BUSY_W<'a> {
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
#[doc = "ADC12 Conversion Sequence Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12CONSEQ_A {
    #[doc = "0: ADC12 Conversion Sequence Select: 0"]
    ADC12CONSEQ_0 = 0,
    #[doc = "1: ADC12 Conversion Sequence Select: 1"]
    ADC12CONSEQ_1 = 1,
    #[doc = "2: ADC12 Conversion Sequence Select: 2"]
    ADC12CONSEQ_2 = 2,
    #[doc = "3: ADC12 Conversion Sequence Select: 3"]
    ADC12CONSEQ_3 = 3,
}
impl From<ADC12CONSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12CONSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12CONSEQ`"]
pub type ADC12CONSEQ_R = crate::R<u8, ADC12CONSEQ_A>;
impl ADC12CONSEQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12CONSEQ_A {
        match self.bits {
            0 => ADC12CONSEQ_A::ADC12CONSEQ_0,
            1 => ADC12CONSEQ_A::ADC12CONSEQ_1,
            2 => ADC12CONSEQ_A::ADC12CONSEQ_2,
            3 => ADC12CONSEQ_A::ADC12CONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_0`"]
    #[inline(always)]
    pub fn is_adc12conseq_0(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_1`"]
    #[inline(always)]
    pub fn is_adc12conseq_1(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_2`"]
    #[inline(always)]
    pub fn is_adc12conseq_2(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADC12CONSEQ_3`"]
    #[inline(always)]
    pub fn is_adc12conseq_3(&self) -> bool {
        *self == ADC12CONSEQ_A::ADC12CONSEQ_3
    }
}
#[doc = "Write proxy for field `ADC12CONSEQ`"]
pub struct ADC12CONSEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12CONSEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12CONSEQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Conversion Sequence Select: 0"]
    #[inline(always)]
    pub fn adc12conseq_0(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_0)
    }
    #[doc = "ADC12 Conversion Sequence Select: 1"]
    #[inline(always)]
    pub fn adc12conseq_1(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_1)
    }
    #[doc = "ADC12 Conversion Sequence Select: 2"]
    #[inline(always)]
    pub fn adc12conseq_2(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_2)
    }
    #[doc = "ADC12 Conversion Sequence Select: 3"]
    #[inline(always)]
    pub fn adc12conseq_3(self) -> &'a mut W {
        self.variant(ADC12CONSEQ_A::ADC12CONSEQ_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u16) & 0x03) << 1);
        self.w
    }
}
#[doc = "ADC12 Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SSEL_A {
    #[doc = "0: ADC12 Clock Source Select: 0"]
    ADC12SSEL_0 = 0,
    #[doc = "1: ADC12 Clock Source Select: 1"]
    ADC12SSEL_1 = 1,
    #[doc = "2: ADC12 Clock Source Select: 2"]
    ADC12SSEL_2 = 2,
    #[doc = "3: ADC12 Clock Source Select: 3"]
    ADC12SSEL_3 = 3,
}
impl From<ADC12SSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SSEL`"]
pub type ADC12SSEL_R = crate::R<u8, ADC12SSEL_A>;
impl ADC12SSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SSEL_A {
        match self.bits {
            0 => ADC12SSEL_A::ADC12SSEL_0,
            1 => ADC12SSEL_A::ADC12SSEL_1,
            2 => ADC12SSEL_A::ADC12SSEL_2,
            3 => ADC12SSEL_A::ADC12SSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_0`"]
    #[inline(always)]
    pub fn is_adc12ssel_0(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_1`"]
    #[inline(always)]
    pub fn is_adc12ssel_1(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_2`"]
    #[inline(always)]
    pub fn is_adc12ssel_2(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_2
    }
    #[doc = "Checks if the value of the field is `ADC12SSEL_3`"]
    #[inline(always)]
    pub fn is_adc12ssel_3(&self) -> bool {
        *self == ADC12SSEL_A::ADC12SSEL_3
    }
}
#[doc = "Write proxy for field `ADC12SSEL`"]
pub struct ADC12SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Clock Source Select: 0"]
    #[inline(always)]
    pub fn adc12ssel_0(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_0)
    }
    #[doc = "ADC12 Clock Source Select: 1"]
    #[inline(always)]
    pub fn adc12ssel_1(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_1)
    }
    #[doc = "ADC12 Clock Source Select: 2"]
    #[inline(always)]
    pub fn adc12ssel_2(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_2)
    }
    #[doc = "ADC12 Clock Source Select: 3"]
    #[inline(always)]
    pub fn adc12ssel_3(self) -> &'a mut W {
        self.variant(ADC12SSEL_A::ADC12SSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u16) & 0x03) << 3);
        self.w
    }
}
#[doc = "ADC12 Clock Divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12DIV_A {
    #[doc = "0: ADC12 Clock Divider Select: 0"]
    ADC12DIV_0 = 0,
    #[doc = "1: ADC12 Clock Divider Select: 1"]
    ADC12DIV_1 = 1,
    #[doc = "2: ADC12 Clock Divider Select: 2"]
    ADC12DIV_2 = 2,
    #[doc = "3: ADC12 Clock Divider Select: 3"]
    ADC12DIV_3 = 3,
    #[doc = "4: ADC12 Clock Divider Select: 4"]
    ADC12DIV_4 = 4,
    #[doc = "5: ADC12 Clock Divider Select: 5"]
    ADC12DIV_5 = 5,
    #[doc = "6: ADC12 Clock Divider Select: 6"]
    ADC12DIV_6 = 6,
    #[doc = "7: ADC12 Clock Divider Select: 7"]
    ADC12DIV_7 = 7,
}
impl From<ADC12DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12DIV`"]
pub type ADC12DIV_R = crate::R<u8, ADC12DIV_A>;
impl ADC12DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12DIV_A {
        match self.bits {
            0 => ADC12DIV_A::ADC12DIV_0,
            1 => ADC12DIV_A::ADC12DIV_1,
            2 => ADC12DIV_A::ADC12DIV_2,
            3 => ADC12DIV_A::ADC12DIV_3,
            4 => ADC12DIV_A::ADC12DIV_4,
            5 => ADC12DIV_A::ADC12DIV_5,
            6 => ADC12DIV_A::ADC12DIV_6,
            7 => ADC12DIV_A::ADC12DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_0`"]
    #[inline(always)]
    pub fn is_adc12div_0(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_0
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_1`"]
    #[inline(always)]
    pub fn is_adc12div_1(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_1
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_2`"]
    #[inline(always)]
    pub fn is_adc12div_2(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_2
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_3`"]
    #[inline(always)]
    pub fn is_adc12div_3(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_3
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_4`"]
    #[inline(always)]
    pub fn is_adc12div_4(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_4
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_5`"]
    #[inline(always)]
    pub fn is_adc12div_5(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_5
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_6`"]
    #[inline(always)]
    pub fn is_adc12div_6(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_6
    }
    #[doc = "Checks if the value of the field is `ADC12DIV_7`"]
    #[inline(always)]
    pub fn is_adc12div_7(&self) -> bool {
        *self == ADC12DIV_A::ADC12DIV_7
    }
}
#[doc = "Write proxy for field `ADC12DIV`"]
pub struct ADC12DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Clock Divider Select: 0"]
    #[inline(always)]
    pub fn adc12div_0(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_0)
    }
    #[doc = "ADC12 Clock Divider Select: 1"]
    #[inline(always)]
    pub fn adc12div_1(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_1)
    }
    #[doc = "ADC12 Clock Divider Select: 2"]
    #[inline(always)]
    pub fn adc12div_2(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_2)
    }
    #[doc = "ADC12 Clock Divider Select: 3"]
    #[inline(always)]
    pub fn adc12div_3(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_3)
    }
    #[doc = "ADC12 Clock Divider Select: 4"]
    #[inline(always)]
    pub fn adc12div_4(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_4)
    }
    #[doc = "ADC12 Clock Divider Select: 5"]
    #[inline(always)]
    pub fn adc12div_5(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_5)
    }
    #[doc = "ADC12 Clock Divider Select: 6"]
    #[inline(always)]
    pub fn adc12div_6(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_6)
    }
    #[doc = "ADC12 Clock Divider Select: 7"]
    #[inline(always)]
    pub fn adc12div_7(self) -> &'a mut W {
        self.variant(ADC12DIV_A::ADC12DIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADC12ISSH`"]
pub type ADC12ISSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ISSH`"]
pub struct ADC12ISSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ISSH_W<'a> {
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
#[doc = "Reader of field `ADC12SHP`"]
pub type ADC12SHP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12SHP`"]
pub struct ADC12SHP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHP_W<'a> {
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
#[doc = "ADC12 Sample/Hold Source Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHS_A {
    #[doc = "0: ADC12 Sample/Hold Source: 0"]
    ADC12SHS_0 = 0,
    #[doc = "1: ADC12 Sample/Hold Source: 1"]
    ADC12SHS_1 = 1,
    #[doc = "2: ADC12 Sample/Hold Source: 2"]
    ADC12SHS_2 = 2,
    #[doc = "3: ADC12 Sample/Hold Source: 3"]
    ADC12SHS_3 = 3,
    #[doc = "4: ADC12 Sample/Hold Source: 4"]
    ADC12SHS_4 = 4,
    #[doc = "5: ADC12 Sample/Hold Source: 5"]
    ADC12SHS_5 = 5,
    #[doc = "6: ADC12 Sample/Hold Source: 6"]
    ADC12SHS_6 = 6,
    #[doc = "7: ADC12 Sample/Hold Source: 7"]
    ADC12SHS_7 = 7,
}
impl From<ADC12SHS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHS`"]
pub type ADC12SHS_R = crate::R<u8, ADC12SHS_A>;
impl ADC12SHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHS_A {
        match self.bits {
            0 => ADC12SHS_A::ADC12SHS_0,
            1 => ADC12SHS_A::ADC12SHS_1,
            2 => ADC12SHS_A::ADC12SHS_2,
            3 => ADC12SHS_A::ADC12SHS_3,
            4 => ADC12SHS_A::ADC12SHS_4,
            5 => ADC12SHS_A::ADC12SHS_5,
            6 => ADC12SHS_A::ADC12SHS_6,
            7 => ADC12SHS_A::ADC12SHS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_0`"]
    #[inline(always)]
    pub fn is_adc12shs_0(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_1`"]
    #[inline(always)]
    pub fn is_adc12shs_1(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_2`"]
    #[inline(always)]
    pub fn is_adc12shs_2(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_3`"]
    #[inline(always)]
    pub fn is_adc12shs_3(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_4`"]
    #[inline(always)]
    pub fn is_adc12shs_4(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_5`"]
    #[inline(always)]
    pub fn is_adc12shs_5(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_6`"]
    #[inline(always)]
    pub fn is_adc12shs_6(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHS_7`"]
    #[inline(always)]
    pub fn is_adc12shs_7(&self) -> bool {
        *self == ADC12SHS_A::ADC12SHS_7
    }
}
#[doc = "Write proxy for field `ADC12SHS`"]
pub struct ADC12SHS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Sample/Hold Source: 0"]
    #[inline(always)]
    pub fn adc12shs_0(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_0)
    }
    #[doc = "ADC12 Sample/Hold Source: 1"]
    #[inline(always)]
    pub fn adc12shs_1(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_1)
    }
    #[doc = "ADC12 Sample/Hold Source: 2"]
    #[inline(always)]
    pub fn adc12shs_2(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_2)
    }
    #[doc = "ADC12 Sample/Hold Source: 3"]
    #[inline(always)]
    pub fn adc12shs_3(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_3)
    }
    #[doc = "ADC12 Sample/Hold Source: 4"]
    #[inline(always)]
    pub fn adc12shs_4(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_4)
    }
    #[doc = "ADC12 Sample/Hold Source: 5"]
    #[inline(always)]
    pub fn adc12shs_5(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_5)
    }
    #[doc = "ADC12 Sample/Hold Source: 6"]
    #[inline(always)]
    pub fn adc12shs_6(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_6)
    }
    #[doc = "ADC12 Sample/Hold Source: 7"]
    #[inline(always)]
    pub fn adc12shs_7(self) -> &'a mut W {
        self.variant(ADC12SHS_A::ADC12SHS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u16) & 0x07) << 10);
        self.w
    }
}
#[doc = "ADC12 Predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12PDIV_A {
    #[doc = "0: ADC12 Clock predivider Select 0"]
    ADC12PDIV_0 = 0,
    #[doc = "1: ADC12 Clock predivider Select 1"]
    ADC12PDIV_1 = 1,
    #[doc = "2: ADC12 Clock predivider Select 2"]
    ADC12PDIV_2 = 2,
    #[doc = "3: ADC12 Clock predivider Select 3"]
    ADC12PDIV_3 = 3,
}
impl From<ADC12PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12PDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12PDIV`"]
pub type ADC12PDIV_R = crate::R<u8, ADC12PDIV_A>;
impl ADC12PDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12PDIV_A {
        match self.bits {
            0 => ADC12PDIV_A::ADC12PDIV_0,
            1 => ADC12PDIV_A::ADC12PDIV_1,
            2 => ADC12PDIV_A::ADC12PDIV_2,
            3 => ADC12PDIV_A::ADC12PDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12PDIV_0`"]
    #[inline(always)]
    pub fn is_adc12pdiv_0(&self) -> bool {
        *self == ADC12PDIV_A::ADC12PDIV_0
    }
    #[doc = "Checks if the value of the field is `ADC12PDIV_1`"]
    #[inline(always)]
    pub fn is_adc12pdiv_1(&self) -> bool {
        *self == ADC12PDIV_A::ADC12PDIV_1
    }
    #[doc = "Checks if the value of the field is `ADC12PDIV_2`"]
    #[inline(always)]
    pub fn is_adc12pdiv_2(&self) -> bool {
        *self == ADC12PDIV_A::ADC12PDIV_2
    }
    #[doc = "Checks if the value of the field is `ADC12PDIV_3`"]
    #[inline(always)]
    pub fn is_adc12pdiv_3(&self) -> bool {
        *self == ADC12PDIV_A::ADC12PDIV_3
    }
}
#[doc = "Write proxy for field `ADC12PDIV`"]
pub struct ADC12PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12PDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12PDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Clock predivider Select 0"]
    #[inline(always)]
    pub fn adc12pdiv_0(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::ADC12PDIV_0)
    }
    #[doc = "ADC12 Clock predivider Select 1"]
    #[inline(always)]
    pub fn adc12pdiv_1(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::ADC12PDIV_1)
    }
    #[doc = "ADC12 Clock predivider Select 2"]
    #[inline(always)]
    pub fn adc12pdiv_2(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::ADC12PDIV_2)
    }
    #[doc = "ADC12 Clock predivider Select 3"]
    #[inline(always)]
    pub fn adc12pdiv_3(self) -> &'a mut W {
        self.variant(ADC12PDIV_A::ADC12PDIV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    pub fn adc12busy(&self) -> ADC12BUSY_R {
        ADC12BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select Bit: 0"]
    #[inline(always)]
    pub fn adc12conseq(&self) -> ADC12CONSEQ_R {
        ADC12CONSEQ_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc12ssel(&self) -> ADC12SSEL_R {
        ADC12SSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc12div(&self) -> ADC12DIV_R {
        ADC12DIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adc12issh(&self) -> ADC12ISSH_R {
        ADC12ISSH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adc12shp(&self) -> ADC12SHP_R {
        ADC12SHP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - ADC12 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn adc12shs(&self) -> ADC12SHS_R {
        ADC12SHS_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:14 - ADC12 Predivider Bit: 0"]
    #[inline(always)]
    pub fn adc12pdiv(&self) -> ADC12PDIV_R {
        ADC12PDIV_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Busy"]
    #[inline(always)]
    pub fn adc12busy(&mut self) -> ADC12BUSY_W {
        ADC12BUSY_W { w: self }
    }
    #[doc = "Bits 1:2 - ADC12 Conversion Sequence Select Bit: 0"]
    #[inline(always)]
    pub fn adc12conseq(&mut self) -> ADC12CONSEQ_W {
        ADC12CONSEQ_W { w: self }
    }
    #[doc = "Bits 3:4 - ADC12 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc12ssel(&mut self) -> ADC12SSEL_W {
        ADC12SSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - ADC12 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc12div(&mut self) -> ADC12DIV_W {
        ADC12DIV_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn adc12issh(&mut self) -> ADC12ISSH_W {
        ADC12ISSH_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Sample/Hold Pulse Mode"]
    #[inline(always)]
    pub fn adc12shp(&mut self) -> ADC12SHP_W {
        ADC12SHP_W { w: self }
    }
    #[doc = "Bits 10:12 - ADC12 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn adc12shs(&mut self) -> ADC12SHS_W {
        ADC12SHS_W { w: self }
    }
    #[doc = "Bits 13:14 - ADC12 Predivider Bit: 0"]
    #[inline(always)]
    pub fn adc12pdiv(&mut self) -> ADC12PDIV_W {
        ADC12PDIV_W { w: self }
    }
}
