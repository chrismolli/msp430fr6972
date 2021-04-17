#[doc = "Reader of register CSCTL1"]
pub type R = crate::R<u16, super::CSCTL1>;
#[doc = "Writer for register CSCTL1"]
pub type W = crate::W<u16, super::CSCTL1>;
#[doc = "Register CSCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DCO frequency select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCOFSEL_A {
    #[doc = "0: DCO frequency select: 0"]
    DCOFSEL_0 = 0,
    #[doc = "1: DCO frequency select: 1"]
    DCOFSEL_1 = 1,
    #[doc = "2: DCO frequency select: 2"]
    DCOFSEL_2 = 2,
    #[doc = "3: DCO frequency select: 3"]
    DCOFSEL_3 = 3,
    #[doc = "4: DCO frequency select: 4"]
    DCOFSEL_4 = 4,
    #[doc = "5: DCO frequency select: 5"]
    DCOFSEL_5 = 5,
    #[doc = "6: DCO frequency select: 6"]
    DCOFSEL_6 = 6,
    #[doc = "7: DCO frequency select: 7"]
    DCOFSEL_7 = 7,
}
impl From<DCOFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCOFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCOFSEL`"]
pub type DCOFSEL_R = crate::R<u8, DCOFSEL_A>;
impl DCOFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFSEL_A {
        match self.bits {
            0 => DCOFSEL_A::DCOFSEL_0,
            1 => DCOFSEL_A::DCOFSEL_1,
            2 => DCOFSEL_A::DCOFSEL_2,
            3 => DCOFSEL_A::DCOFSEL_3,
            4 => DCOFSEL_A::DCOFSEL_4,
            5 => DCOFSEL_A::DCOFSEL_5,
            6 => DCOFSEL_A::DCOFSEL_6,
            7 => DCOFSEL_A::DCOFSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_0`"]
    #[inline(always)]
    pub fn is_dcofsel_0(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_0
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_1`"]
    #[inline(always)]
    pub fn is_dcofsel_1(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_1
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_2`"]
    #[inline(always)]
    pub fn is_dcofsel_2(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_2
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_3`"]
    #[inline(always)]
    pub fn is_dcofsel_3(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_3
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_4`"]
    #[inline(always)]
    pub fn is_dcofsel_4(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_4
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_5`"]
    #[inline(always)]
    pub fn is_dcofsel_5(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_5
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_6`"]
    #[inline(always)]
    pub fn is_dcofsel_6(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_6
    }
    #[doc = "Checks if the value of the field is `DCOFSEL_7`"]
    #[inline(always)]
    pub fn is_dcofsel_7(&self) -> bool {
        *self == DCOFSEL_A::DCOFSEL_7
    }
}
#[doc = "Write proxy for field `DCOFSEL`"]
pub struct DCOFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCOFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DCO frequency select: 0"]
    #[inline(always)]
    pub fn dcofsel_0(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_0)
    }
    #[doc = "DCO frequency select: 1"]
    #[inline(always)]
    pub fn dcofsel_1(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_1)
    }
    #[doc = "DCO frequency select: 2"]
    #[inline(always)]
    pub fn dcofsel_2(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_2)
    }
    #[doc = "DCO frequency select: 3"]
    #[inline(always)]
    pub fn dcofsel_3(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_3)
    }
    #[doc = "DCO frequency select: 4"]
    #[inline(always)]
    pub fn dcofsel_4(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_4)
    }
    #[doc = "DCO frequency select: 5"]
    #[inline(always)]
    pub fn dcofsel_5(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_5)
    }
    #[doc = "DCO frequency select: 6"]
    #[inline(always)]
    pub fn dcofsel_6(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_6)
    }
    #[doc = "DCO frequency select: 7"]
    #[inline(always)]
    pub fn dcofsel_7(self) -> &'a mut W {
        self.variant(DCOFSEL_A::DCOFSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u16) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `DCORSEL`"]
pub type DCORSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCORSEL`"]
pub struct DCORSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCORSEL_W<'a> {
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
impl R {
    #[doc = "Bits 1:3 - DCO frequency select Bit: 0"]
    #[inline(always)]
    pub fn dcofsel(&self) -> DCOFSEL_R {
        DCOFSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 6 - DCO range select."]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - DCO frequency select Bit: 0"]
    #[inline(always)]
    pub fn dcofsel(&mut self) -> DCOFSEL_W {
        DCOFSEL_W { w: self }
    }
    #[doc = "Bit 6 - DCO range select."]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W {
        DCORSEL_W { w: self }
    }
}
