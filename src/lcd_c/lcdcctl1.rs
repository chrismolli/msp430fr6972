#[doc = "Reader of register LCDCCTL1"]
pub type R = crate::R<u16, super::LCDCCTL1>;
#[doc = "Writer for register LCDCCTL1"]
pub type W = crate::W<u16, super::LCDCCTL1>;
#[doc = "Register LCDCCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LCDCCTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCDFRMIFG`"]
pub type LCDFRMIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDFRMIFG`"]
pub struct LCDFRMIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIFG_W<'a> {
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
#[doc = "Reader of field `LCDBLKOFFIFG`"]
pub type LCDBLKOFFIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKOFFIFG`"]
pub struct LCDBLKOFFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIFG_W<'a> {
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
#[doc = "Reader of field `LCDBLKONIFG`"]
pub type LCDBLKONIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKONIFG`"]
pub struct LCDBLKONIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIFG_W<'a> {
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
#[doc = "Reader of field `LCDNOCAPIFG`"]
pub type LCDNOCAPIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDNOCAPIFG`"]
pub struct LCDNOCAPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDNOCAPIFG_W<'a> {
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
#[doc = "Reader of field `LCDFRMIE`"]
pub type LCDFRMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDFRMIE`"]
pub struct LCDFRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDFRMIE_W<'a> {
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
#[doc = "Reader of field `LCDBLKOFFIE`"]
pub type LCDBLKOFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKOFFIE`"]
pub struct LCDBLKOFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKOFFIE_W<'a> {
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
#[doc = "Reader of field `LCDBLKONIE`"]
pub type LCDBLKONIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDBLKONIE`"]
pub struct LCDBLKONIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBLKONIE_W<'a> {
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
#[doc = "Reader of field `LCDNOCAPIE`"]
pub type LCDNOCAPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDNOCAPIE`"]
pub struct LCDNOCAPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDNOCAPIE_W<'a> {
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
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LCDFRMIFG_R {
        LCDFRMIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LCDBLKOFFIFG_R {
        LCDBLKOFFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LCDBLKONIFG_R {
        LCDBLKONIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&self) -> LCDNOCAPIFG_R {
        LCDNOCAPIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LCDFRMIE_R {
        LCDFRMIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LCDBLKOFFIE_R {
        LCDBLKOFFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LCDBLKONIE_R {
        LCDBLKONIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&self) -> LCDNOCAPIE_R {
        LCDNOCAPIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD_C LCD frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&mut self) -> LCDFRMIFG_W {
        LCDFRMIFG_W { w: self }
    }
    #[doc = "Bit 1 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&mut self) -> LCDBLKOFFIFG_W {
        LCDBLKOFFIFG_W { w: self }
    }
    #[doc = "Bit 2 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&mut self) -> LCDBLKONIFG_W {
        LCDBLKONIFG_W { w: self }
    }
    #[doc = "Bit 3 - LCD_C No cpacitance connected interrupt flag"]
    #[inline(always)]
    pub fn lcdnocapifg(&mut self) -> LCDNOCAPIFG_W {
        LCDNOCAPIFG_W { w: self }
    }
    #[doc = "Bit 8 - LCD_C LCD frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LCDFRMIE_W {
        LCDFRMIE_W { w: self }
    }
    #[doc = "Bit 9 - LCD_C LCD blinking off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LCDBLKOFFIE_W {
        LCDBLKOFFIE_W { w: self }
    }
    #[doc = "Bit 10 - LCD_C LCD blinking on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LCDBLKONIE_W {
        LCDBLKONIE_W { w: self }
    }
    #[doc = "Bit 11 - LCD_C No cpacitance connected interrupt enable"]
    #[inline(always)]
    pub fn lcdnocapie(&mut self) -> LCDNOCAPIE_W {
        LCDNOCAPIE_W { w: self }
    }
}
