#[doc = "Reader of register ADC12IER2"]
pub type R = crate::R<u16, super::ADC12IER2>;
#[doc = "Writer for register ADC12IER2"]
pub type W = crate::W<u16, super::ADC12IER2>;
#[doc = "Register ADC12IER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IER2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12INIE`"]
pub type ADC12INIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12INIE`"]
pub struct ADC12INIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIE_W<'a> {
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
#[doc = "Reader of field `ADC12LOIE`"]
pub type ADC12LOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12LOIE`"]
pub struct ADC12LOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIE_W<'a> {
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
#[doc = "Reader of field `ADC12HIIE`"]
pub type ADC12HIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12HIIE`"]
pub struct ADC12HIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIE_W<'a> {
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
#[doc = "Reader of field `ADC12OVIE`"]
pub type ADC12OVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12OVIE`"]
pub struct ADC12OVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIE_W<'a> {
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
#[doc = "Reader of field `ADC12TOVIE`"]
pub type ADC12TOVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12TOVIE`"]
pub struct ADC12TOVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIE_W<'a> {
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
#[doc = "Reader of field `ADC12RDYIE`"]
pub type ADC12RDYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12RDYIE`"]
pub struct ADC12RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIE_W<'a> {
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
    #[doc = "Bit 1 - ADC12 Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inie(&self) -> ADC12INIE_R {
        ADC12INIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loie(&self) -> ADC12LOIE_R {
        ADC12LOIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiie(&self) -> ADC12HIIE_R {
        ADC12HIIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&self) -> ADC12OVIE_R {
        ADC12OVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&self) -> ADC12TOVIE_R {
        ADC12TOVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc12rdyie(&self) -> ADC12RDYIE_R {
        ADC12RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC12 Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inie(&mut self) -> ADC12INIE_W {
        ADC12INIE_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loie(&mut self) -> ADC12LOIE_W {
        ADC12LOIE_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiie(&mut self) -> ADC12HIIE_W {
        ADC12HIIE_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12ovie(&mut self) -> ADC12OVIE_W {
        ADC12OVIE_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt enable"]
    #[inline(always)]
    pub fn adc12tovie(&mut self) -> ADC12TOVIE_W {
        ADC12TOVIE_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc12rdyie(&mut self) -> ADC12RDYIE_W {
        ADC12RDYIE_W { w: self }
    }
}
