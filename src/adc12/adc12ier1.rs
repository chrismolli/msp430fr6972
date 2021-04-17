#[doc = "Reader of register ADC12IER1"]
pub type R = crate::R<u16, super::ADC12IER1>;
#[doc = "Writer for register ADC12IER1"]
pub type W = crate::W<u16, super::ADC12IER1>;
#[doc = "Register ADC12IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IER1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12IE16`"]
pub type ADC12IE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE16`"]
pub struct ADC12IE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE16_W<'a> {
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
#[doc = "Reader of field `ADC12IE17`"]
pub type ADC12IE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE17`"]
pub struct ADC12IE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE17_W<'a> {
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
#[doc = "Reader of field `ADC12IE18`"]
pub type ADC12IE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE18`"]
pub struct ADC12IE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE18_W<'a> {
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
#[doc = "Reader of field `ADC12IE19`"]
pub type ADC12IE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE19`"]
pub struct ADC12IE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE19_W<'a> {
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
#[doc = "Reader of field `ADC12IE20`"]
pub type ADC12IE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE20`"]
pub struct ADC12IE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE20_W<'a> {
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
#[doc = "Reader of field `ADC12IE21`"]
pub type ADC12IE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE21`"]
pub struct ADC12IE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE21_W<'a> {
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
#[doc = "Reader of field `ADC12IE22`"]
pub type ADC12IE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE22`"]
pub struct ADC12IE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE22_W<'a> {
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
#[doc = "Reader of field `ADC12IE23`"]
pub type ADC12IE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE23`"]
pub struct ADC12IE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE23_W<'a> {
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
#[doc = "Reader of field `ADC12IE24`"]
pub type ADC12IE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE24`"]
pub struct ADC12IE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE24_W<'a> {
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
#[doc = "Reader of field `ADC12IE25`"]
pub type ADC12IE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE25`"]
pub struct ADC12IE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE25_W<'a> {
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
#[doc = "Reader of field `ADC12IE26`"]
pub type ADC12IE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE26`"]
pub struct ADC12IE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE26_W<'a> {
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
#[doc = "Reader of field `ADC12IE27`"]
pub type ADC12IE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE27`"]
pub struct ADC12IE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE27_W<'a> {
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
#[doc = "Reader of field `ADC12IE28`"]
pub type ADC12IE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE28`"]
pub struct ADC12IE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE28_W<'a> {
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
#[doc = "Reader of field `ADC12IE29`"]
pub type ADC12IE29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE29`"]
pub struct ADC12IE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE29_W<'a> {
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
#[doc = "Reader of field `ADC12IE30`"]
pub type ADC12IE30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE30`"]
pub struct ADC12IE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE30_W<'a> {
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
#[doc = "Reader of field `ADC12IE31`"]
pub type ADC12IE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12IE31`"]
pub struct ADC12IE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12IE31_W<'a> {
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
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie16(&self) -> ADC12IE16_R {
        ADC12IE16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie17(&self) -> ADC12IE17_R {
        ADC12IE17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie18(&self) -> ADC12IE18_R {
        ADC12IE18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie19(&self) -> ADC12IE19_R {
        ADC12IE19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie20(&self) -> ADC12IE20_R {
        ADC12IE20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie21(&self) -> ADC12IE21_R {
        ADC12IE21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie22(&self) -> ADC12IE22_R {
        ADC12IE22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie23(&self) -> ADC12IE23_R {
        ADC12IE23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie24(&self) -> ADC12IE24_R {
        ADC12IE24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie25(&self) -> ADC12IE25_R {
        ADC12IE25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie26(&self) -> ADC12IE26_R {
        ADC12IE26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie27(&self) -> ADC12IE27_R {
        ADC12IE27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie28(&self) -> ADC12IE28_R {
        ADC12IE28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie29(&self) -> ADC12IE29_R {
        ADC12IE29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie30(&self) -> ADC12IE30_R {
        ADC12IE30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie31(&self) -> ADC12IE31_R {
        ADC12IE31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Memory 16 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie16(&mut self) -> ADC12IE16_W {
        ADC12IE16_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Memory 17 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie17(&mut self) -> ADC12IE17_W {
        ADC12IE17_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Memory 18 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie18(&mut self) -> ADC12IE18_W {
        ADC12IE18_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Memory 19 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie19(&mut self) -> ADC12IE19_W {
        ADC12IE19_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 Memory 20 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie20(&mut self) -> ADC12IE20_W {
        ADC12IE20_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Memory 21 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie21(&mut self) -> ADC12IE21_W {
        ADC12IE21_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 Memory 22 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie22(&mut self) -> ADC12IE22_W {
        ADC12IE22_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Memory 23 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie23(&mut self) -> ADC12IE23_W {
        ADC12IE23_W { w: self }
    }
    #[doc = "Bit 8 - ADC12 Memory 24 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie24(&mut self) -> ADC12IE24_W {
        ADC12IE24_W { w: self }
    }
    #[doc = "Bit 9 - ADC12 Memory 25 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie25(&mut self) -> ADC12IE25_W {
        ADC12IE25_W { w: self }
    }
    #[doc = "Bit 10 - ADC12 Memory 26 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie26(&mut self) -> ADC12IE26_W {
        ADC12IE26_W { w: self }
    }
    #[doc = "Bit 11 - ADC12 Memory 27 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie27(&mut self) -> ADC12IE27_W {
        ADC12IE27_W { w: self }
    }
    #[doc = "Bit 12 - ADC12 Memory 28 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie28(&mut self) -> ADC12IE28_W {
        ADC12IE28_W { w: self }
    }
    #[doc = "Bit 13 - ADC12 Memory 29 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie29(&mut self) -> ADC12IE29_W {
        ADC12IE29_W { w: self }
    }
    #[doc = "Bit 14 - ADC12 Memory 30 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie30(&mut self) -> ADC12IE30_W {
        ADC12IE30_W { w: self }
    }
    #[doc = "Bit 15 - ADC12 Memory 31 Interrupt Enable"]
    #[inline(always)]
    pub fn adc12ie31(&mut self) -> ADC12IE31_W {
        ADC12IE31_W { w: self }
    }
}
