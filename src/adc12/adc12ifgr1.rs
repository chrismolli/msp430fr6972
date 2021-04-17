#[doc = "Reader of register ADC12IFGR1"]
pub type R = crate::R<u16, super::ADC12IFGR1>;
#[doc = "Writer for register ADC12IFGR1"]
pub type W = crate::W<u16, super::ADC12IFGR1>;
#[doc = "Register ADC12IFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFGR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12IFG16`"]
pub type ADC12IFG16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG16`"]
pub struct ADC12IFG16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG16_W<'a> {
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
#[doc = "Reader of field `ADC12IFG17`"]
pub type ADC12IFG17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG17`"]
pub struct ADC12IFG17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG17_W<'a> {
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
#[doc = "Reader of field `ADC12IFG18`"]
pub type ADC12IFG18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG18`"]
pub struct ADC12IFG18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG18_W<'a> {
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
#[doc = "Reader of field `ADC12IFG19`"]
pub type ADC12IFG19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG19`"]
pub struct ADC12IFG19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG19_W<'a> {
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
#[doc = "Reader of field `ADC12IFG20`"]
pub type ADC12IFG20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG20`"]
pub struct ADC12IFG20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG20_W<'a> {
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
#[doc = "Reader of field `ADC12IFG21`"]
pub type ADC12IFG21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG21`"]
pub struct ADC12IFG21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG21_W<'a> {
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
#[doc = "Reader of field `ADC12IFG22`"]
pub type ADC12IFG22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG22`"]
pub struct ADC12IFG22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG22_W<'a> {
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
#[doc = "Reader of field `ADC12IFG23`"]
pub type ADC12IFG23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG23`"]
pub struct ADC12IFG23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG23_W<'a> {
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
#[doc = "Reader of field `ADC12IFG24`"]
pub type ADC12IFG24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG24`"]
pub struct ADC12IFG24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG24_W<'a> {
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
#[doc = "Reader of field `ADC12IFG25`"]
pub type ADC12IFG25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG25`"]
pub struct ADC12IFG25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG25_W<'a> {
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
#[doc = "Reader of field `ADC12IFG26`"]
pub type ADC12IFG26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG26`"]
pub struct ADC12IFG26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG26_W<'a> {
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
#[doc = "Reader of field `ADC12IFG27`"]
pub type ADC12IFG27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG27`"]
pub struct ADC12IFG27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG27_W<'a> {
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
#[doc = "Reader of field `ADC12IFG28`"]
pub type ADC12IFG28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG28`"]
pub struct ADC12IFG28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG28_W<'a> {
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
#[doc = "Reader of field `ADC12IFG29`"]
pub type ADC12IFG29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG29`"]
pub struct ADC12IFG29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG29_W<'a> {
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
#[doc = "Reader of field `ADC12IFG30`"]
pub type ADC12IFG30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG30`"]
pub struct ADC12IFG30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG30_W<'a> {
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
#[doc = "Reader of field `ADC12IFG31`"]
pub type ADC12IFG31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IFG31`"]
pub struct ADC12IFG31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IFG31_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg16(&self) -> ADC12IFG16_R {
        ADC12IFG16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg17(&self) -> ADC12IFG17_R {
        ADC12IFG17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg18(&self) -> ADC12IFG18_R {
        ADC12IFG18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg19(&self) -> ADC12IFG19_R {
        ADC12IFG19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg20(&self) -> ADC12IFG20_R {
        ADC12IFG20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg21(&self) -> ADC12IFG21_R {
        ADC12IFG21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg22(&self) -> ADC12IFG22_R {
        ADC12IFG22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg23(&self) -> ADC12IFG23_R {
        ADC12IFG23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg24(&self) -> ADC12IFG24_R {
        ADC12IFG24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg25(&self) -> ADC12IFG25_R {
        ADC12IFG25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg26(&self) -> ADC12IFG26_R {
        ADC12IFG26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg27(&self) -> ADC12IFG27_R {
        ADC12IFG27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg28(&self) -> ADC12IFG28_R {
        ADC12IFG28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg29(&self) -> ADC12IFG29_R {
        ADC12IFG29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg30(&self) -> ADC12IFG30_R {
        ADC12IFG30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg31(&self) -> ADC12IFG31_R {
        ADC12IFG31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg16(&mut self) -> ADC12IFG16_W {
        ADC12IFG16_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg17(&mut self) -> ADC12IFG17_W {
        ADC12IFG17_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg18(&mut self) -> ADC12IFG18_W {
        ADC12IFG18_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg19(&mut self) -> ADC12IFG19_W {
        ADC12IFG19_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg20(&mut self) -> ADC12IFG20_W {
        ADC12IFG20_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg21(&mut self) -> ADC12IFG21_W {
        ADC12IFG21_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg22(&mut self) -> ADC12IFG22_W {
        ADC12IFG22_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg23(&mut self) -> ADC12IFG23_W {
        ADC12IFG23_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg24(&mut self) -> ADC12IFG24_W {
        ADC12IFG24_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg25(&mut self) -> ADC12IFG25_W {
        ADC12IFG25_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg26(&mut self) -> ADC12IFG26_W {
        ADC12IFG26_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg27(&mut self) -> ADC12IFG27_W {
        ADC12IFG27_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg28(&mut self) -> ADC12IFG28_W {
        ADC12IFG28_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg29(&mut self) -> ADC12IFG29_W {
        ADC12IFG29_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg30(&mut self) -> ADC12IFG30_W {
        ADC12IFG30_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Flag"]
    #[inline(always)]
    pub fn adc12ifg31(&mut self) -> ADC12IFG31_W {
        ADC12IFG31_W { w: self }
    }
}
