#[doc = "Register `LCDCVCTL` reader"]
pub struct R(crate::R<LCDCVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCDCVCTL_SPEC>> for R {
    fn from(reader: crate::R<LCDCVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCVCTL` writer"]
pub struct W(crate::W<LCDCVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCVCTL_SPEC>;
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
impl core::convert::From<crate::W<LCDCVCTL_SPEC>> for W {
    fn from(writer: crate::W<LCDCVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD2B` reader - Selects 1/2 bias."]
pub struct LCD2B_R(crate::FieldReader<bool, bool>);
impl LCD2B_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCD2B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD2B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD2B` writer - Selects 1/2 bias."]
pub struct LCD2B_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD2B_W<'a> {
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
#[doc = "Selects reference voltage for regulated charge pump: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLCDREF_A {
    #[doc = "0: Internal"]
    VLCDREF_0 = 0,
    #[doc = "1: External"]
    VLCDREF_1 = 1,
    #[doc = "2: Reserved"]
    VLCDREF_2 = 2,
    #[doc = "3: Reserved"]
    VLCDREF_3 = 3,
}
impl From<VLCDREF_A> for u8 {
    #[inline(always)]
    fn from(variant: VLCDREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VLCDREF` reader - Selects reference voltage for regulated charge pump: 0"]
pub struct VLCDREF_R(crate::FieldReader<u8, VLCDREF_A>);
impl VLCDREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        VLCDREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLCDREF_A {
        match self.bits {
            0 => VLCDREF_A::VLCDREF_0,
            1 => VLCDREF_A::VLCDREF_1,
            2 => VLCDREF_A::VLCDREF_2,
            3 => VLCDREF_A::VLCDREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VLCDREF_0`"]
    #[inline(always)]
    pub fn is_vlcdref_0(&self) -> bool {
        **self == VLCDREF_A::VLCDREF_0
    }
    #[doc = "Checks if the value of the field is `VLCDREF_1`"]
    #[inline(always)]
    pub fn is_vlcdref_1(&self) -> bool {
        **self == VLCDREF_A::VLCDREF_1
    }
    #[doc = "Checks if the value of the field is `VLCDREF_2`"]
    #[inline(always)]
    pub fn is_vlcdref_2(&self) -> bool {
        **self == VLCDREF_A::VLCDREF_2
    }
    #[doc = "Checks if the value of the field is `VLCDREF_3`"]
    #[inline(always)]
    pub fn is_vlcdref_3(&self) -> bool {
        **self == VLCDREF_A::VLCDREF_3
    }
}
impl core::ops::Deref for VLCDREF_R {
    type Target = crate::FieldReader<u8, VLCDREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLCDREF` writer - Selects reference voltage for regulated charge pump: 0"]
pub struct VLCDREF_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCDREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLCDREF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Internal"]
    #[inline(always)]
    pub fn vlcdref_0(self) -> &'a mut W {
        self.variant(VLCDREF_A::VLCDREF_0)
    }
    #[doc = "External"]
    #[inline(always)]
    pub fn vlcdref_1(self) -> &'a mut W {
        self.variant(VLCDREF_A::VLCDREF_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn vlcdref_2(self) -> &'a mut W {
        self.variant(VLCDREF_A::VLCDREF_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn vlcdref_3(self) -> &'a mut W {
        self.variant(VLCDREF_A::VLCDREF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u16 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `LCDCPEN` reader - LCD Voltage Charge Pump Enable."]
pub struct LCDCPEN_R(crate::FieldReader<bool, bool>);
impl LCDCPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCPEN` writer - LCD Voltage Charge Pump Enable."]
pub struct LCDCPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDCPEN_W<'a> {
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
#[doc = "Field `VLCDEXT` reader - Select external source for VLCD."]
pub struct VLCDEXT_R(crate::FieldReader<bool, bool>);
impl VLCDEXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLCDEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLCDEXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLCDEXT` writer - Select external source for VLCD."]
pub struct VLCDEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCDEXT_W<'a> {
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
#[doc = "Field `LCDEXTBIAS` reader - V2 - V4 voltage select."]
pub struct LCDEXTBIAS_R(crate::FieldReader<bool, bool>);
impl LCDEXTBIAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDEXTBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDEXTBIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDEXTBIAS` writer - V2 - V4 voltage select."]
pub struct LCDEXTBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEXTBIAS_W<'a> {
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
#[doc = "Field `R03EXT` reader - Selects external connections for LCD mid voltages."]
pub struct R03EXT_R(crate::FieldReader<bool, bool>);
impl R03EXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        R03EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R03EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R03EXT` writer - Selects external connections for LCD mid voltages."]
pub struct R03EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> R03EXT_W<'a> {
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
#[doc = "Field `LCDREXT` reader - Selects external connection for lowest LCD voltage."]
pub struct LCDREXT_R(crate::FieldReader<bool, bool>);
impl LCDREXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDREXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDREXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDREXT` writer - Selects external connection for lowest LCD voltage."]
pub struct LCDREXT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDREXT_W<'a> {
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
#[doc = "VLCD select: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLCD_A {
    #[doc = "0: Charge pump disabled"]
    VLCD_0 = 0,
    #[doc = "1: VLCD = 2.60V"]
    VLCD_1 = 1,
    #[doc = "2: VLCD = 2.66V"]
    VLCD_2 = 2,
    #[doc = "3: VLCD = 2.72V"]
    VLCD_3 = 3,
    #[doc = "4: VLCD = 2.78V"]
    VLCD_4 = 4,
    #[doc = "5: VLCD = 2.84V"]
    VLCD_5 = 5,
    #[doc = "6: VLCD = 2.90V"]
    VLCD_6 = 6,
    #[doc = "7: VLCD = 2.96V"]
    VLCD_7 = 7,
    #[doc = "8: VLCD = 3.02V"]
    VLCD_8 = 8,
    #[doc = "9: VLCD = 3.08V"]
    VLCD_9 = 9,
    #[doc = "10: VLCD = 3.14V"]
    VLCD_10 = 10,
    #[doc = "11: VLCD = 3.20V"]
    VLCD_11 = 11,
    #[doc = "12: VLCD = 3.26V"]
    VLCD_12 = 12,
    #[doc = "13: VLCD = 3.32V"]
    VLCD_13 = 13,
    #[doc = "14: VLCD = 3.38V"]
    VLCD_14 = 14,
    #[doc = "15: VLCD = 3.44V"]
    VLCD_15 = 15,
}
impl From<VLCD_A> for u8 {
    #[inline(always)]
    fn from(variant: VLCD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VLCD` reader - VLCD select: 0"]
pub struct VLCD_R(crate::FieldReader<u8, VLCD_A>);
impl VLCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        VLCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VLCD_A> {
        match self.bits {
            0 => Some(VLCD_A::VLCD_0),
            1 => Some(VLCD_A::VLCD_1),
            2 => Some(VLCD_A::VLCD_2),
            3 => Some(VLCD_A::VLCD_3),
            4 => Some(VLCD_A::VLCD_4),
            5 => Some(VLCD_A::VLCD_5),
            6 => Some(VLCD_A::VLCD_6),
            7 => Some(VLCD_A::VLCD_7),
            8 => Some(VLCD_A::VLCD_8),
            9 => Some(VLCD_A::VLCD_9),
            10 => Some(VLCD_A::VLCD_10),
            11 => Some(VLCD_A::VLCD_11),
            12 => Some(VLCD_A::VLCD_12),
            13 => Some(VLCD_A::VLCD_13),
            14 => Some(VLCD_A::VLCD_14),
            15 => Some(VLCD_A::VLCD_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VLCD_0`"]
    #[inline(always)]
    pub fn is_vlcd_0(&self) -> bool {
        **self == VLCD_A::VLCD_0
    }
    #[doc = "Checks if the value of the field is `VLCD_1`"]
    #[inline(always)]
    pub fn is_vlcd_1(&self) -> bool {
        **self == VLCD_A::VLCD_1
    }
    #[doc = "Checks if the value of the field is `VLCD_2`"]
    #[inline(always)]
    pub fn is_vlcd_2(&self) -> bool {
        **self == VLCD_A::VLCD_2
    }
    #[doc = "Checks if the value of the field is `VLCD_3`"]
    #[inline(always)]
    pub fn is_vlcd_3(&self) -> bool {
        **self == VLCD_A::VLCD_3
    }
    #[doc = "Checks if the value of the field is `VLCD_4`"]
    #[inline(always)]
    pub fn is_vlcd_4(&self) -> bool {
        **self == VLCD_A::VLCD_4
    }
    #[doc = "Checks if the value of the field is `VLCD_5`"]
    #[inline(always)]
    pub fn is_vlcd_5(&self) -> bool {
        **self == VLCD_A::VLCD_5
    }
    #[doc = "Checks if the value of the field is `VLCD_6`"]
    #[inline(always)]
    pub fn is_vlcd_6(&self) -> bool {
        **self == VLCD_A::VLCD_6
    }
    #[doc = "Checks if the value of the field is `VLCD_7`"]
    #[inline(always)]
    pub fn is_vlcd_7(&self) -> bool {
        **self == VLCD_A::VLCD_7
    }
    #[doc = "Checks if the value of the field is `VLCD_8`"]
    #[inline(always)]
    pub fn is_vlcd_8(&self) -> bool {
        **self == VLCD_A::VLCD_8
    }
    #[doc = "Checks if the value of the field is `VLCD_9`"]
    #[inline(always)]
    pub fn is_vlcd_9(&self) -> bool {
        **self == VLCD_A::VLCD_9
    }
    #[doc = "Checks if the value of the field is `VLCD_10`"]
    #[inline(always)]
    pub fn is_vlcd_10(&self) -> bool {
        **self == VLCD_A::VLCD_10
    }
    #[doc = "Checks if the value of the field is `VLCD_11`"]
    #[inline(always)]
    pub fn is_vlcd_11(&self) -> bool {
        **self == VLCD_A::VLCD_11
    }
    #[doc = "Checks if the value of the field is `VLCD_12`"]
    #[inline(always)]
    pub fn is_vlcd_12(&self) -> bool {
        **self == VLCD_A::VLCD_12
    }
    #[doc = "Checks if the value of the field is `VLCD_13`"]
    #[inline(always)]
    pub fn is_vlcd_13(&self) -> bool {
        **self == VLCD_A::VLCD_13
    }
    #[doc = "Checks if the value of the field is `VLCD_14`"]
    #[inline(always)]
    pub fn is_vlcd_14(&self) -> bool {
        **self == VLCD_A::VLCD_14
    }
    #[doc = "Checks if the value of the field is `VLCD_15`"]
    #[inline(always)]
    pub fn is_vlcd_15(&self) -> bool {
        **self == VLCD_A::VLCD_15
    }
}
impl core::ops::Deref for VLCD_R {
    type Target = crate::FieldReader<u8, VLCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLCD` writer - VLCD select: 0"]
pub struct VLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLCD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Charge pump disabled"]
    #[inline(always)]
    pub fn vlcd_0(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_0)
    }
    #[doc = "VLCD = 2.60V"]
    #[inline(always)]
    pub fn vlcd_1(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_1)
    }
    #[doc = "VLCD = 2.66V"]
    #[inline(always)]
    pub fn vlcd_2(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_2)
    }
    #[doc = "VLCD = 2.72V"]
    #[inline(always)]
    pub fn vlcd_3(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_3)
    }
    #[doc = "VLCD = 2.78V"]
    #[inline(always)]
    pub fn vlcd_4(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_4)
    }
    #[doc = "VLCD = 2.84V"]
    #[inline(always)]
    pub fn vlcd_5(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_5)
    }
    #[doc = "VLCD = 2.90V"]
    #[inline(always)]
    pub fn vlcd_6(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_6)
    }
    #[doc = "VLCD = 2.96V"]
    #[inline(always)]
    pub fn vlcd_7(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_7)
    }
    #[doc = "VLCD = 3.02V"]
    #[inline(always)]
    pub fn vlcd_8(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_8)
    }
    #[doc = "VLCD = 3.08V"]
    #[inline(always)]
    pub fn vlcd_9(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_9)
    }
    #[doc = "VLCD = 3.14V"]
    #[inline(always)]
    pub fn vlcd_10(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_10)
    }
    #[doc = "VLCD = 3.20V"]
    #[inline(always)]
    pub fn vlcd_11(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_11)
    }
    #[doc = "VLCD = 3.26V"]
    #[inline(always)]
    pub fn vlcd_12(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_12)
    }
    #[doc = "VLCD = 3.32V"]
    #[inline(always)]
    pub fn vlcd_13(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_13)
    }
    #[doc = "VLCD = 3.38V"]
    #[inline(always)]
    pub fn vlcd_14(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_14)
    }
    #[doc = "VLCD = 3.44V"]
    #[inline(always)]
    pub fn vlcd_15(self) -> &'a mut W {
        self.variant(VLCD_A::VLCD_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | ((value as u16 & 0x3f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects 1/2 bias."]
    #[inline(always)]
    pub fn lcd2b(&self) -> LCD2B_R {
        LCD2B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Selects reference voltage for regulated charge pump: 0"]
    #[inline(always)]
    pub fn vlcdref(&self) -> VLCDREF_R {
        VLCDREF_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - LCD Voltage Charge Pump Enable."]
    #[inline(always)]
    pub fn lcdcpen(&self) -> LCDCPEN_R {
        LCDCPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select external source for VLCD."]
    #[inline(always)]
    pub fn vlcdext(&self) -> VLCDEXT_R {
        VLCDEXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - V2 - V4 voltage select."]
    #[inline(always)]
    pub fn lcdextbias(&self) -> LCDEXTBIAS_R {
        LCDEXTBIAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects external connections for LCD mid voltages."]
    #[inline(always)]
    pub fn r03ext(&self) -> R03EXT_R {
        R03EXT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects external connection for lowest LCD voltage."]
    #[inline(always)]
    pub fn lcdrext(&self) -> LCDREXT_R {
        LCDREXT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:14 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects 1/2 bias."]
    #[inline(always)]
    pub fn lcd2b(&mut self) -> LCD2B_W {
        LCD2B_W { w: self }
    }
    #[doc = "Bits 1:2 - Selects reference voltage for regulated charge pump: 0"]
    #[inline(always)]
    pub fn vlcdref(&mut self) -> VLCDREF_W {
        VLCDREF_W { w: self }
    }
    #[doc = "Bit 3 - LCD Voltage Charge Pump Enable."]
    #[inline(always)]
    pub fn lcdcpen(&mut self) -> LCDCPEN_W {
        LCDCPEN_W { w: self }
    }
    #[doc = "Bit 4 - Select external source for VLCD."]
    #[inline(always)]
    pub fn vlcdext(&mut self) -> VLCDEXT_W {
        VLCDEXT_W { w: self }
    }
    #[doc = "Bit 5 - V2 - V4 voltage select."]
    #[inline(always)]
    pub fn lcdextbias(&mut self) -> LCDEXTBIAS_W {
        LCDEXTBIAS_W { w: self }
    }
    #[doc = "Bit 6 - Selects external connections for LCD mid voltages."]
    #[inline(always)]
    pub fn r03ext(&mut self) -> R03EXT_W {
        R03EXT_W { w: self }
    }
    #[doc = "Bit 7 - Selects external connection for lowest LCD voltage."]
    #[inline(always)]
    pub fn lcdrext(&mut self) -> LCDREXT_W {
        LCDREXT_W { w: self }
    }
    #[doc = "Bits 9:14 - VLCD select: 0"]
    #[inline(always)]
    pub fn vlcd(&mut self) -> VLCD_W {
        VLCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_C Voltage Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcvctl](index.html) module"]
pub struct LCDCVCTL_SPEC;
impl crate::RegisterSpec for LCDCVCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcdcvctl::R](R) reader structure"]
impl crate::Readable for LCDCVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcvctl::W](W) writer structure"]
impl crate::Writable for LCDCVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCVCTL to value 0"]
impl crate::Resettable for LCDCVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
