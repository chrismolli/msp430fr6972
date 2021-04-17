#[doc = "Reader of register ADC12IFGR2"]
pub type R = crate::R<u16, super::ADC12IFGR2>;
#[doc = "Writer for register ADC12IFGR2"]
pub type W = crate::W<u16, super::ADC12IFGR2>;
#[doc = "Register ADC12IFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12IFGR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12INIFG`"]
pub type ADC12INIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12INIFG`"]
pub struct ADC12INIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12INIFG_W<'a> {
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
#[doc = "Reader of field `ADC12LOIFG`"]
pub type ADC12LOIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12LOIFG`"]
pub struct ADC12LOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LOIFG_W<'a> {
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
#[doc = "Reader of field `ADC12HIIFG`"]
pub type ADC12HIIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12HIIFG`"]
pub struct ADC12HIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12HIIFG_W<'a> {
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
#[doc = "Reader of field `ADC12OVIFG`"]
pub type ADC12OVIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12OVIFG`"]
pub struct ADC12OVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12OVIFG_W<'a> {
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
#[doc = "Reader of field `ADC12TOVIFG`"]
pub type ADC12TOVIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12TOVIFG`"]
pub struct ADC12TOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12TOVIFG_W<'a> {
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
#[doc = "Reader of field `ADC12RDYIFG`"]
pub type ADC12RDYIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12RDYIFG`"]
pub struct ADC12RDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RDYIFG_W<'a> {
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
    #[doc = "Bit 1 - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inifg(&self) -> ADC12INIFG_R {
        ADC12INIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loifg(&self) -> ADC12LOIFG_R {
        ADC12LOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiifg(&self) -> ADC12HIIFG_R {
        ADC12HIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12ovifg(&self) -> ADC12OVIFG_R {
        ADC12OVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12tovifg(&self) -> ADC12TOVIFG_R {
        ADC12TOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt Flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&self) -> ADC12RDYIFG_R {
        ADC12RDYIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC12 Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adc12inifg(&mut self) -> ADC12INIFG_W {
        ADC12INIFG_W { w: self }
    }
    #[doc = "Bit 2 - ADC12 Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12loifg(&mut self) -> ADC12LOIFG_W {
        ADC12LOIFG_W { w: self }
    }
    #[doc = "Bit 3 - ADC12 Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adc12hiifg(&mut self) -> ADC12HIIFG_W {
        ADC12HIIFG_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 ADC12MEMx Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12ovifg(&mut self) -> ADC12OVIFG_W {
        ADC12OVIFG_W { w: self }
    }
    #[doc = "Bit 5 - ADC12 Timer Overflow interrupt Flag"]
    #[inline(always)]
    pub fn adc12tovifg(&mut self) -> ADC12TOVIFG_W {
        ADC12TOVIFG_W { w: self }
    }
    #[doc = "Bit 6 - ADC12 local buffered reference ready interrupt Flag"]
    #[inline(always)]
    pub fn adc12rdyifg(&mut self) -> ADC12RDYIFG_W {
        ADC12RDYIFG_W { w: self }
    }
}
