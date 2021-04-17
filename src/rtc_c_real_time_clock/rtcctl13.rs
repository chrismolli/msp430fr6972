#[doc = "Reader of register RTCCTL13"]
pub type R = crate::R<u16, super::RTCCTL13>;
#[doc = "Writer for register RTCCTL13"]
pub type W = crate::W<u16, super::RTCCTL13>;
#[doc = "Register RTCCTL13 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCTL13 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RTC Time Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: RTC Time Event: 0 (Min. changed)"]
    RTCTEV_0 = 0,
    #[doc = "1: RTC Time Event: 1 (Hour changed)"]
    RTCTEV_1 = 1,
    #[doc = "2: RTC Time Event: 2 (12:00 changed)"]
    RTCTEV_2 = 2,
    #[doc = "3: RTC Time Event: 3 (00:00 changed)"]
    RTCTEV_3 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCTEV`"]
pub type RTCTEV_R = crate::R<u8, RTCTEV_A>;
impl RTCTEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::RTCTEV_0,
            1 => RTCTEV_A::RTCTEV_1,
            2 => RTCTEV_A::RTCTEV_2,
            3 => RTCTEV_A::RTCTEV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEV_0`"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_0
    }
    #[doc = "Checks if the value of the field is `RTCTEV_1`"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_1
    }
    #[doc = "Checks if the value of the field is `RTCTEV_2`"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_2
    }
    #[doc = "Checks if the value of the field is `RTCTEV_3`"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        *self == RTCTEV_A::RTCTEV_3
    }
}
#[doc = "Write proxy for field `RTCTEV`"]
pub struct RTCTEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Time Event: 0 (Min. changed)"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_0)
    }
    #[doc = "RTC Time Event: 1 (Hour changed)"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_1)
    }
    #[doc = "RTC Time Event: 2 (12:00 changed)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_2)
    }
    #[doc = "RTC Time Event: 3 (00:00 changed)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "RTC Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: RTC Source Select ACLK"]
    RTCSSEL_0 = 0,
    #[doc = "1: RTC Source Select SMCLK"]
    RTCSSEL_1 = 1,
    #[doc = "2: RTC Source Select RT1PS"]
    RTCSSEL_2 = 2,
    #[doc = "3: RTC Source Select RT1PS"]
    RTCSSEL_3 = 3,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCSSEL`"]
pub type RTCSSEL_R = crate::R<u8, RTCSSEL_A>;
impl RTCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSSEL_A {
        match self.bits {
            0 => RTCSSEL_A::RTCSSEL_0,
            1 => RTCSSEL_A::RTCSSEL_1,
            2 => RTCSSEL_A::RTCSSEL_2,
            3 => RTCSSEL_A::RTCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_0`"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_0
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_1`"]
    #[inline(always)]
    pub fn is_rtcssel_1(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_1
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_2`"]
    #[inline(always)]
    pub fn is_rtcssel_2(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_2
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_3`"]
    #[inline(always)]
    pub fn is_rtcssel_3(&self) -> bool {
        *self == RTCSSEL_A::RTCSSEL_3
    }
}
#[doc = "Write proxy for field `RTCSSEL`"]
pub struct RTCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Source Select ACLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_0)
    }
    #[doc = "RTC Source Select SMCLK"]
    #[inline(always)]
    pub fn rtcssel_1(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_1)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_2(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_2)
    }
    #[doc = "RTC Source Select RT1PS"]
    #[inline(always)]
    pub fn rtcssel_3(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RTCRDY`"]
pub type RTCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCRDY`"]
pub struct RTCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRDY_W<'a> {
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
#[doc = "Reader of field `RTCMODE`"]
pub type RTCMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCMODE`"]
pub struct RTCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMODE_W<'a> {
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
#[doc = "Reader of field `RTCHOLD`"]
pub type RTCHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCHOLD`"]
pub struct RTCHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCHOLD_W<'a> {
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
#[doc = "Reader of field `RTCBCD`"]
pub type RTCBCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCBCD`"]
pub struct RTCBCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCBCD_W<'a> {
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
#[doc = "RTC Calibration Frequency Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: RTC Calibration Frequency: No Output"]
    RTCCALF_0 = 0,
    #[doc = "1: RTC Calibration Frequency: 512 Hz"]
    RTCCALF_1 = 1,
    #[doc = "2: RTC Calibration Frequency: 256 Hz"]
    RTCCALF_2 = 2,
    #[doc = "3: RTC Calibration Frequency: 1 Hz"]
    RTCCALF_3 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCCALF`"]
pub type RTCCALF_R = crate::R<u8, RTCCALF_A>;
impl RTCCALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCALF_0`"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "Checks if the value of the field is `RTCCALF_1`"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "Checks if the value of the field is `RTCCALF_2`"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "Checks if the value of the field is `RTCCALF_3`"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        *self == RTCCALF_A::RTCCALF_3
    }
}
#[doc = "Write proxy for field `RTCCALF`"]
pub struct RTCCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCALF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RTC Calibration Frequency: No Output"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "RTC Calibration Frequency: 512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "RTC Calibration Frequency: 256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "RTC Calibration Frequency: 1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC Time Event 1"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RTCTEV_W {
        RTCTEV_W { w: self }
    }
    #[doc = "Bits 2:3 - RTC Source Select 1"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RTCSSEL_W {
        RTCSSEL_W { w: self }
    }
    #[doc = "Bit 4 - RTC Ready"]
    #[inline(always)]
    pub fn rtcrdy(&mut self) -> RTCRDY_W {
        RTCRDY_W { w: self }
    }
    #[doc = "Bit 5 - RTC Mode 0:Counter / 1: Calendar"]
    #[inline(always)]
    pub fn rtcmode(&mut self) -> RTCMODE_W {
        RTCMODE_W { w: self }
    }
    #[doc = "Bit 6 - RTC Hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RTCHOLD_W {
        RTCHOLD_W { w: self }
    }
    #[doc = "Bit 7 - RTC BCD 0:Binary / 1:BCD"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RTCBCD_W {
        RTCBCD_W { w: self }
    }
    #[doc = "Bits 8:9 - RTC Calibration Frequency Bit 1"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W { w: self }
    }
}
